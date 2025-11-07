//use iroh::{Endpoint, SecretKey, protocol::Router};
use iroh::{Endpoint, EndpointId, PublicKey, SecretKey};
//use std::path::PathBuf;
use std::fs;
use std::path::Path;

const KEY_DIR: &str = "./.keys";
const KEY_FILE: &str = "./.keys/secret";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    match arg_refs.as_slice() {
        ["endpoint", "create"] => {
            // note: this is just for testing, keys will be stored in ./.keys/secret
            if Path::new(KEY_FILE).exists() {
                anyhow::bail!("stable endpoint already exists");
            }

            println!("storing keys in .keys");
            let key = SecretKey::generate(&mut rand::rng()).to_bytes();
            fs::create_dir_all(KEY_DIR)?;
            fs::write(KEY_FILE, key)?;
            println!("wrote private key to {}", KEY_FILE);
        }
        ["endpoint", "read"] => {
            let secret_key = get_secret_key()?;
            println!(
                "this device's public key (and endpoint id) is {}",
                secret_key.public()
            );
        }
        ["ping", "listen"] => {
            println!("listening for ping");
            let secret_key = get_secret_key()?;
            //let endpoint_id: EndpointId = secret_key.public();
            let endpoint = Endpoint::builder().secret_key(secret_key).bind().await?;
            if let Some(incoming) = endpoint.accept().await {
                println!("someone wants to know");
                let iconn = incoming.accept()?;
                let conn = iconn.await?;
                let (mut send, mut recv) = conn.accept_bi().await?;
                let m = recv.read_to_end(100).await?;
                println!("{}", String::from_utf8(m)?);
                send.write_all(b"looks like we made it").await?;
                send.finish()?;
            }
        }
        ["ping", "send", addr] => {
            println!("pinging {}", addr);

            let secret_key = get_secret_key()?;
            let endpoint = Endpoint::builder().secret_key(secret_key).bind().await?;
            let addr: PublicKey = addr.parse()?;
            let conn = endpoint.connect(addr, b"nateha/iroh-cli/1").await?;
            let (mut send, mut recv) = conn.open_bi().await?;
            println!("connection opened");
            send.write_all(b"did we make it?").await?;
            println!("checking to see if we made it");
            send.finish()?;
            let m = recv.read_to_end(100).await?;
            println!("{}", String::from_utf8(m)?);
        }
        _ => {
            println!("unknown command");
        }
    }

    //let endpoint = Endpoint::bind().await?;

    Ok(())
}

fn get_secret_key() -> anyhow::Result<SecretKey> {
    let secret_key_bytes = fs::read(KEY_FILE)?;
    let secret_key_array: [u8; 32] = secret_key_bytes.try_into().expect("failed to read key");
    let secret_key = SecretKey::from_bytes(&secret_key_array);
    Ok(secret_key)
}
