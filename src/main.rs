use lightning::ln::peer_handler::IgnoringMessageHandler;
use lightning::ln::peer_handler::PeerManager;
use lightning::util::logger::Logger;
use lightning::util::logger::Record;
use secp256k1::rand;
use secp256k1::rand::rngs::OsRng;
use secp256k1::PublicKey;
use secp256k1::Secp256k1;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpStream;

mod messages;
mod socket;

struct MyLogger;
impl Logger for MyLogger {
    fn log(&self, record: Record) {
        println!("{}", record.args);
    }
}

fn parse_cli_args() -> (PublicKey, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <pubkey@ip:port>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    let (target_pubkey_str, addr) = input.split_once('@').expect("Invalid input format");
    let target_pubkey = PublicKey::from_str(target_pubkey_str).expect("Invalid public key");
    (target_pubkey, addr.to_string())
}

#[tokio::main]
async fn main() {
    let logger = Arc::new(MyLogger);
    let secp = Secp256k1::new();
    let (_my_secret_key, _my_public_key) = secp.generate_keypair(&mut OsRng);
    let (_target_pubkey, addr) = parse_cli_args();

    let current_time = std::time::Instant::now();
    // let ephemeral_random_state = secp.generate_keypair(&mut OsRng);

    let ignoring_handler = IgnoringMessageHandler {};
    let message_handler = MessageHandler::new(ignoring_handler);

    // https://lightningdevkit.org/introduction/peer-management/
    // https://docs.rs/lightning/latest/lightning/ln/peer_handler/struct.PeerManager.html
    let peer_manager = Arc::new(PeerManager::new(
        message_handler,
        current_time,
        rand::random::<u32>(), // Random seed
        logger.clone(),
        node_signer,
    ));

    match TcpStream::connect(addr.clone()).await {
        Ok(mut stream) => {
            println!("Successfully connected to {}", addr);
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", addr, e);
        }
    }
>>>>>>> ltcp
}
