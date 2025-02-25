use cli::parse_cli_args;
use lightning::bitcoin::secp256k1::PublicKey;
use lightning::ln::peer_handler::{IgnoringMessageHandler, MessageHandler, SimpleRefPeerManager};
use lightning::sign::KeysManager;
use logger::MyLogger;
use messages::MsgHandler;
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

mod cli;
mod logger;
mod messages;
mod socket;

#[tokio::main]
async fn main() {
    let (_target_pubkey, addr) = parse_cli_args();

    let logger = MyLogger::new();
    let secp = Secp256k1::new();
    let (my_secret_key, my_public_key) = secp.generate_keypair(&mut OsRng);
    let current_timestamp = 0;
    let initial_random_data = [0u8; 32];
    let private_seed = [1u8; 32];

    let pubkey = PublicKey::from_str(&my_public_key.to_string()).unwrap();
    let pubkey_connected = mpsc::channel(0).0;
    let pubkey_disconnected = mpsc::channel(0).0;
    let disconnected_flag = AtomicBool::new(false);
    let msg_events = Mutex::new(Vec::new());

    let channel_message_handler = MsgHandler::new(
        pubkey,
        pubkey_connected,
        pubkey_disconnected,
        disconnected_flag,
        msg_events,
    );
    let routing_message_handler = channel_message_handler;
    let onion_handler = IgnoringMessageHandler {};
    let custom_handler = IgnoringMessageHandler {};
    let message_handler = MessageHandler {
        channel_message_handler,
        routing_message_handler,
        onion_message_handler: onion_handler,
        custom_message_handler: custom_handler,
    };

    let keys_manager = KeysManager::new(&private_seed, 0, 0);

    let pm = SimpleRefPeerManager::new(
        message_handler,
        current_timestamp,
        &initial_random_data,
        &logger,
        &keys_manager,
    );

    match TcpStream::connect(addr.clone()).await {
        Ok(_stream) => {
            println!("Successfully connected to {}", addr);
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", addr, e);
        }
    }
}
