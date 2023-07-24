mod encstream;
use std::io::Cursor;
// use chacha20::ChaCha20;
use tokio::io::AsyncReadExt;
// Import relevant traits
// use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
// use hex_literal::hex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let f = tokio::fs::File::open("README.md").await?;

    let key = [0x42; 32];
    let nonce = [0x24; 12];

    let mut s = encstream::DecStream::new(f, &key, &nonce);

    let mut buffer = [0; 1024];
    let n = s.read(&mut buffer[..]).await?;

    println!("read: {}[bytes]", n);
    println!("{:?}", &buffer[..n]);
    println!(
        "{:?}",
        String::from_utf8(buffer[..n].iter().cloned().collect())
    );

    let cursor = Cursor::new(&mut buffer[..n]);
    let mut s = encstream::DecStream::new(cursor, &key, &nonce);

    let mut buffer1 = [0; 1024];
    let n = s.read(&mut buffer1[..]).await?;

    println!("read: {}[bytes]", n);
    println!(
        "{:?}",
        String::from_utf8(buffer1[..n].iter().cloned().collect())
    );

    // let mut f = tokio::fs::File::open("foo.txt").await?;
    // let mut buffer = [0; 10];

    // // 最大10バイト読み込む
    // let n = f.read(&mut buffer[..]).await?;

    // println!("The bytes: {:?}", &buffer[..n]);
    Ok(())

    // let mut key = Vec::new();
    // for i in 0..32 {
    //     key.push(i);
    // }

    // let key = [0x42; 32];
    // let nonce = [0x24; 12];
    // let plaintext = hex!("00010203 04050607 08090a0b 0c0d0e0f");
    // let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

    // // Key and IV must be references to the `GenericArray` type.
    // // Here we use the `Into` trait to convert arrays into it.
    // let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    // let mut buffer = plaintext.clone();

    // // apply keystream (encrypt)
    // cipher.apply_keystream(&mut buffer);
    // assert_eq!(buffer, ciphertext);

    // let ciphertext = buffer.clone();

    // // ChaCha ciphers support seeking
    // cipher.seek(0u32);

    // // decrypt ciphertext by applying keystream again
    // cipher.apply_keystream(&mut buffer);
    // assert_eq!(buffer, plaintext);

    // // stream ciphers can be used with streaming messages
    // cipher.seek(0u32);
    // for chunk in buffer.chunks_mut(3) {
    //     cipher.apply_keystream(chunk);
    // }
    // assert_eq!(buffer, ciphertext);
    // Ok(())
}
