use anyhow::Ok;
use iroh::{Endpoint, PublicKey, SecretKey};
use std::fs;
use std::path::Path;

const KEY_DIR: &str = "./.keys";
const ALPN_PING: &[u8] = b"nateha/iroh-cli/ping";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    match arg_refs.as_slice() {
        ["endpoint", "create"] => {
            create_secret_key("secret").await?;
        }
        ["endpoint", "create", keyname] => {
            // note: this is just for testing, keys will be stored in ./.keys/keyname
            create_secret_key(&keyname).await?;
        }
        ["endpoint", "read"] => {
            print_endpoint("secret").await?;
        }
        ["endpoint", "read", keyname] => {
            print_endpoint(keyname).await?;
        }
        ["ping", "listen"] => {
            iroh_ping_listen("secret").await?;
        }
        ["ping", "listen", keyname] => {
            iroh_ping_listen(keyname).await?;
        }
        ["ping", "connect", addr] => {
            iroh_ping_connect("secret", addr).await?;
        }
        ["ping", "connect", from_keyname, to_endpoint_id] => {
            iroh_ping_connect(from_keyname, to_endpoint_id).await?;
        }
        ["sync", "listen", keyname] => {
            sync_listen(keyname).await?;
        }
        ["sync", "push", from_keyname, to_keyname] => {
            sync_push(from_keyname, to_keyname).await?;
        }
        _ => {
            println!("unknown command");
        }
    }

    Ok(())
}

async fn create_secret_key(name: &str) -> anyhow::Result<()> {
    let key_file = Path::new(KEY_DIR).join(name);
    if key_file.exists() {
        anyhow::bail!("endpoint for {} already exists", name);
    }

    println!("generated key and storing at .keys/{}", name);
    let key = SecretKey::generate(&mut rand::rng()).to_bytes();
    fs::create_dir_all(KEY_DIR)?;
    fs::write(key_file, key)?;
    println!("wrote private key to {}", name);

    Ok(())
}

async fn print_endpoint(name: &str) -> anyhow::Result<()> {
    let secret_key = get_secret_key(name)?;
    eprintln!("this public key (and endpoint id) for {}:", name);
    println!("{}", secret_key.public());

    Ok(())
}

async fn iroh_ping_listen(keyname: &str) -> anyhow::Result<()> {
    let secret_key = get_secret_key(keyname)?;
    println!(
        "listening for ping on key '{}' at {}",
        keyname,
        secret_key.public()
    );
    //let endpoint_id: EndpointId = secret_key.public();
    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![ALPN_PING.to_vec()])
        .bind()
        .await?;
    if let Some(incoming) = endpoint.accept().await {
        println!("someone wants to know");
        let iconn = incoming.accept()?;
        let conn = iconn.await?;
        let (mut send, mut recv) = conn.accept_bi().await?;
        let m = recv.read_to_end(100).await?;
        println!("{}", String::from_utf8(m)?);
        send.write_all(b"looks like we made it").await?;
        send.finish()?;
        conn.closed().await;
    }
    Ok(())
}

async fn iroh_ping_connect(from_keyname: &str, to_endpoint: &str) -> anyhow::Result<()> {
    println!("pinging from {} to {}", from_keyname, to_endpoint);
    let secret_key = get_secret_key(from_keyname)?;
    let endpoint = Endpoint::builder().secret_key(secret_key).bind().await?;
    let addr: PublicKey = to_endpoint.parse()?;
    let conn = endpoint.connect(addr, b"nateha/iroh-cli/ping").await?;
    let (mut send, mut recv) = conn.open_bi().await?;
    println!("connection opened");
    send.write_all(b"did we make it?").await?;
    println!("checking to see if we made it");
    send.finish()?;
    let m = recv.read_to_end(100).await?;
    println!("{}", String::from_utf8(m)?);
    conn.close(0u8.into(), b"done");
    conn.closed().await;
    Ok(())
}

async fn sync_listen(keyname: &str) -> anyhow::Result<()> {
    // plan for this function
    //
    // get_secret_key(name)
    //
    // listen for connections
    // - on connect:
    //   - get remote endpoint id
    //   - check ./keys/.* for a petname that matches the remote endpoint id
    //     - matching key:
    //       - sync_protocol_listener(remote_endpoint_id)
    //       -
    //
    // sync_protocol_listener(my_endpoint_id, their_patname, their_endpoint):
    // // presumably, the connecting party has changes for us, so:
    // - wait for new manifest, which will be a list of files and checksums
    // - compare manifest to on-disk layout at data/{my_endpoint_id}/mirror_for/{their_petname}
    // - for each file, request file
    //   - wait for file
    //   - write file to disk
    // - send eof

    Ok(())
}

async fn sync_push(from_keyname: &str, to_keyname: &str) -> anyhow::Result<()> {
    // plan
    //
    // create_manifest_for ./data/from_keyname/mirror_to/to_keyname
    // connect_send_manifest to keyname.address
    // while not eof:
    //   - verify_file_request
    //   - send_file

    Ok(())
}

fn get_secret_key(name: &str) -> anyhow::Result<SecretKey> {
    let key_file = Path::new(KEY_DIR).join(name);
    if !key_file.exists() {
        anyhow::bail!("no key for {}", name);
    }
    let secret_key_bytes = fs::read(key_file)?;
    let secret_key_array: [u8; 32] = secret_key_bytes.try_into().expect("failed to read key");
    let secret_key = SecretKey::from_bytes(&secret_key_array);
    Ok(secret_key)
}
