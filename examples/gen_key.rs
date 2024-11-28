use anyhow::Result;
use base64_url::base64;
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits
    rand::thread_rng().fill_bytes(&mut key);

    let b64u = base64_url::encode(&key);
    println!("Key b64u encoded:\n{}", b64u);
    Ok(())
}
