mod raw_key_pairs;
mod hd_wallet_sol;

#[tokio::main]
async fn main() {
    raw_key_pairs::sol_keys();
    raw_key_pairs::eth_keys().await;
    hd_wallet_sol::generate_n_key_pair(1);
}
