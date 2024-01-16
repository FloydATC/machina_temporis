
// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

#[cfg(test)]
mod test;


use std::io::{Read, Write, Seek, SeekFrom, SeekFrom::Start, Error};
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use futures::join;
use rand::Rng;
use futures::executor::block_on;


// Define a set of async functions to act as an example front end

async fn len(shared_stg: SharedStorage) -> usize {
    shared_stg.len()
}

async fn read(shared_stg: SharedStorage, index: usize, buf: &mut [u8]) -> Result<usize, Error> {
    shared_stg.read(index, buf)
}

async fn write(shared_stg: SharedStorage, index: usize, buf: &[u8]) -> Result<usize, Error> {
    shared_stg.write(index, buf)
}

async fn print(shared_stg: SharedStorage) {
    shared_stg.print()
}

async fn switch_case(shared_stg: SharedStorage) {
    let len = len(shared_stg.clone()).await;
    let pos = rand::thread_rng().gen_range(0..len);
    let mut buf = [0x00; 1];
    read(shared_stg.clone(), pos, &mut buf).await.unwrap();
    match buf[0] {
        b'a' ..= b'z' => { buf[0] -= 32 }
        b'A' ..= b'Z' => { buf[0] += 32 }
        _ => {}
    }
    write(shared_stg, pos, &buf).await.unwrap();
}

async fn work(shared_stg: SharedStorage) {
    let f1 = switch_case(shared_stg.clone());
    let f2 = switch_case(shared_stg.clone());
    let f3 = switch_case(shared_stg.clone());
    let f4 = switch_case(shared_stg.clone());
    join!(f1, f2, f3, f4);
}

fn main() {
    let shared_stg = SharedStorage::new();

    loop {
        // Do some random work, reading and writing parts of our shared_stg at random, in paralell
        for _ in 0..10000 {
            let future = work(shared_stg.clone());
            block_on(future);
        }
        // Periodically print shared_stg to STDOUT
        let future = print(shared_stg.clone());
        block_on(future);
    }
}


// Wrapper for our sample Storage, to keep Mutex locking and RefCell borrowing separate from its logic

#[allow(dead_code)]
struct SharedStorage {
    inner: Arc<Mutex<RefCell<Storage>>>,
}

#[allow(dead_code)]
impl SharedStorage {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(RefCell::new(Storage::new()))) }
    }
    pub fn len(&self) -> usize {
        let mutex = self.inner.lock().unwrap();
        let storage = mutex.borrow();
        storage.len()
    }
    pub fn read(&self, index: usize, buf: &mut [u8]) -> Result<usize, Error> {
        // Note: vscode likes to automatically insert 'use std::borrow::BorrowMut' and break borrow_mut()!
        let mutex = self.inner.lock().unwrap();
        let mut storage = mutex.borrow_mut();
        storage.seek(Start(index as u64))?;
        storage.read(buf)
    }
    pub fn write(&self, index: usize, buf: &[u8]) -> Result<usize, Error> {
        // Note: vscode likes to automatically insert 'use std::borrow::BorrowMut' and break borrow_mut()!
        let mutex = self.inner.lock().unwrap();
        let mut storage = mutex.borrow_mut();
        storage.seek(Start(index as u64))?;
        storage.write(buf)
    }
    pub fn print(&self) {
        let mutex = self.inner.lock().unwrap();
        let storage = mutex.borrow();
        storage.print()
    }
}

impl Clone for SharedStorage {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner ) }
    }
}


// Our sample Storage, allowing random read/write operations

#[allow(dead_code)]
struct Storage {
    str: String,
    pos: u64,
}

#[allow(dead_code)]
impl Storage {
    pub fn new() -> Self {
        Self { str: String::from("MACHINA TEMPORIS"), pos: 0 }
    }
    pub fn len(&self) -> usize {
        self.str.len()
    }
    pub fn print(&self) {
        print!("\r{}", self.str);
        std::io::stdout().flush().unwrap();
    }
}


impl Read for Storage {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let pos = self.pos as usize;
        if pos >= self.str.len() { return Ok(0); }
        let len = std::cmp::min(self.str.len()-pos, buf.len());
        buf[..len].copy_from_slice(&self.str.as_bytes()[pos..pos+len]);
        self.pos += len as u64;
        Ok(len)
    }
}


impl Write for Storage {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let pos = self.pos as usize;
        if pos >= self.str.len() { return Ok(0); }
        let len = std::cmp::min(self.str.len()-pos, buf.len());
        // The following is unsafe because resulting String might be invalid UTF8
        // For this test, the string is hardcoded as "MACHINA TEMPORIS" (mixed case) so we know it's ok.
        unsafe { self.str.as_bytes_mut()[pos..pos+len].copy_from_slice(&buf[..len]); }
        self.pos += len as u64;
        Ok(len)
    }
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}


impl Seek for Storage {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        match pos {
            SeekFrom::Start(offset) => self.pos = offset,
            SeekFrom::End(offset) => self.pos = (self.str.len() as i64 + offset) as u64,
            SeekFrom::Current(offset) => self.pos = (self.pos as i64 + offset) as u64,
        }
        Ok(self.pos)
    }
}

