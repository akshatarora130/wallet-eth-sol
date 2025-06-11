use ethers::{core::{k256::{EncodedPoint}, rand::thread_rng}, signers::{LocalWallet, Signer}};
use solana_sdk::{signature::Keypair, signer::Signer as EthSigner};

// EdDSA Algorithm used by SOL 
// Eleptic curve used in this is ed25519
fn sol_keys() {
    // Step 1 - Create key pair
    let key_pair = Keypair::new();
    
    let public_key = key_pair.pubkey();
    let public_key_bytes = public_key.to_bytes();
    println!("SOL Public Key -> {}", public_key);

    let private_key_bytes = key_pair.secret_bytes();
    let private_key = bs58::encode(private_key_bytes).into_string();
    
    println!("SOL Private Key -> {}", private_key);

    // Step 2 - Define a message to sign
    let string = String::from("Hello");
    // Step 3 - Convert the message to bytes
    let message = string.as_bytes();

    // let wrong_message_string = String::from("testing");
    // let wrong_message = wrong_message_string.as_bytes();

    // Step 4 - Sign the message
    let signature = key_pair.sign_message(message);
    println!("SOL Signature -> {:?}", signature);

    // Step 5 - Verify the message
    let verify = signature.verify(&public_key_bytes, &message);
    println!("SOL Verify -> {}", verify);
}

// ECDSA algorithm used by ETH 
// Eleptic curve used in this is secp256k1
async fn eth_keys() {
    let wallet = LocalWallet::new(&mut thread_rng());

    let wallet_address = wallet.address();
    println!("ETH Wallet address -> {:?}", wallet_address);
    
    let verifying_key = wallet.signer().verifying_key();
    let pubkey_point: EncodedPoint = verifying_key.to_encoded_point(false); // false = uncompressed
    let public_key_bytes = pubkey_point.as_bytes();
    let public_key = bs58::encode(public_key_bytes).into_string();
    println!("ETH Public Key -> {}", public_key);

    let private_key_bytes = wallet.signer().to_bytes();
    let private_key = bs58::encode(private_key_bytes).into_string();
    println!("ETH Private Key -> {}", private_key);

    let message_string = String::from("Hello");
    let message = message_string.as_bytes(); 

    // let wrong_message_string = String::from("testing");
    // let wrong_message = wrong_message_string.as_bytes();
    let signature = wallet.sign_message(message).await;

    match signature {
        Ok(sign) => {
            match sign.recover(message) {
                Ok(recovered_address) => {
                    println!("ETH Recovered Address -> {:?}", recovered_address);
                    println!("ETH Signature is valid -> {}", recovered_address == wallet_address);
                },
                Err(e) => println!("Failed to recover address: {:?}", e),
            }
        }
        Err(e) => println!("Signing failed: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    sol_keys();
    eth_keys().await;
}
