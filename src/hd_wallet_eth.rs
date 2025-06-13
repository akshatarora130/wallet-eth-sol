// use std::str::FromStr;
use bip39::Language;
use ethers::signers::{coins_bip39::English, MnemonicBuilder, Signer};

pub fn generate_n_key_pair(n: u32) {
    let result = bip39::Mnemonic::generate_in(Language::English, 12);
    // let my_words = "";
    // let my_mnemonic = bip39::Mnemonic::from_str(&my_words);

    match result {
        Ok(words) => {
            println!("{}", words.to_string());
            let word_string = words.to_string();

            for i in 0..n {
                let path = format!("m/44'/60'/{}'/0'", i);
                let wallet = MnemonicBuilder::<English>::default()
                    .phrase(&*word_string)
                    .derivation_path(&path).unwrap()
                    .build().unwrap();

                let address = wallet.address();
                let private_key_bytes = wallet.signer().to_bytes();
                let private_key = hex::encode(private_key_bytes);

                println!("Private Key Eth-> 0x{}", private_key);
                println!("Pulic Key Eth -> {:?}", address);
            }
        }
        Err(e) => println!("Error in generating words {:?}", e)
    }
}
