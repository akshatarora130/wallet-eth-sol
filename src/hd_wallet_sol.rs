// use std::str::FromStr;
use bip39::Language;
use ed25519_hd_key::derive_from_path;
use solana_sdk::{signature::Keypair, signer::Signer};

pub fn generate_n_key_pair(n: u32) {
    let result = bip39::Mnemonic::generate_in(Language::English, 12);
    // let my_words = "";
    // let my_mnemonic = bip39::Mnemonic::from_str(&my_words);

    match result {
        Ok(words) =>  {
            println!("{}", words.to_string());

            let seed = words.to_seed("");
            println!("Seed -> {:?}", seed);

            for i in 0..n {
                let path = format!("m/44'/501'/{}'/0'", i);
                let derivation_path = derive_from_path(&path, &seed);
                let key_pair = Keypair::new_from_array(derivation_path.0);
                let public_key = key_pair.pubkey();
                let secret_key = key_pair.to_base58_string();

                println!("Private Key -> {}", secret_key);
                println!("Pulic Key -> {}", public_key);
            }
        }
        Err(e) => println!("Error in generating words {:?}", e)
    }
}
