use std::env;
use std::str::FromStr;

use secp256k1::PublicKey;

pub fn parse_cli_args() -> (PublicKey, String) {
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
