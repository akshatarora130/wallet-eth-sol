use bip39::Language;
use ed25519_hd_key::derive_from_path;
use solana_sdk::{signature::Keypair, signer::Signer};

mod raw_key_pairs;

#[tokio::main]
async fn main() {
    raw_key_pairs::sol_keys();
    raw_key_pairs::eth_keys().await;

    let result = bip39::Mnemonic::generate_in(Language::English, 12);
    match result {
        Ok(words) =>  {
            println!("{}", words.to_string());

            let seed = words.to_seed("");
            println!("Seed -> {:?}", seed);

            for i in 0..1 {
                let path = format!("m/44'/501'/{}'/0'", i);
                let derivation_path = derive_from_path(&path, &seed);
                let secret_key = bs58::encode(derivation_path.0).into_string();
                let key_pair = Keypair::new_from_array(derivation_path.0);
                let public_key = key_pair.pubkey();

                println!("Private Key -> {}", secret_key);
                println!("Pulic Key -> {}", public_key);
            }
        }
        Err(e) => println!("Error in generating words {:?}", e)
    }
}
