

use super::*;


#[tokio::test]
async fn test_len_async() {
    let shared_stg = SharedStorage::new();
    assert_eq!(len(shared_stg).await, 16);
}


#[tokio::test]
async fn test_read() {
    let shared_stg = SharedStorage::new();

    let mut buf = [];
    assert_eq!(read(shared_stg.clone(), 0, &mut buf).await.unwrap(), 0);
    assert_eq!(buf, []);

    let mut buf = [0x00; 1];
    assert_eq!(read(shared_stg.clone(), 0, &mut buf).await.unwrap(), 1);
    assert_eq!(buf, [b'M']);

    let mut buf = [0x00; 1];
    assert_eq!(read(shared_stg.clone(), 1, &mut buf).await.unwrap(), 1);
    assert_eq!(buf, [b'A']);

    let mut buf = [0x00; 1];
    assert_eq!(read(shared_stg.clone(), 2, &mut buf).await.unwrap(), 1);
    assert_eq!(buf, [b'C']);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 0, &mut buf).await.unwrap(), 2);
    assert_eq!(buf, [b'M',b'A']);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 2, &mut buf).await.unwrap(), 2);
    assert_eq!(buf, [b'C',b'H']);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 4, &mut buf).await.unwrap(), 2);
    assert_eq!(buf, [b'I',b'N']);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 14, &mut buf).await.unwrap(), 2);
    assert_eq!(buf, [b'I',b'S']);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 15, &mut buf).await.unwrap(), 1);
    assert_eq!(buf, [b'S',0x00]);

    let mut buf = [0x00; 2];
    assert_eq!(read(shared_stg.clone(), 16, &mut buf).await.unwrap(), 0);
    assert_eq!(buf, [0x00,0x00]);

}

