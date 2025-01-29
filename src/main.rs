
use std::fs;
use serde_json::Value;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::Address;

#[tokio::main]
async fn main() {
    // Connect to Ganache
    let transport = Http::new("http://127.0.0.1:7545")
        .expect("Failed to connect to Ethereum RPC");
    let web3 = web3::Web3::new(transport);

    // Replace this with the actual contract address from Truffle
    let contract_address: Address = "0x740eC423170F3318196AED6ce64979c0563a5E69"
        .parse()
        .expect("Invalid contract address");

    // Load the ABI JSON File
    let abi_path = "../voting-contract/build/contracts/Voting.json";
    let abi_content = fs::read_to_string(abi_path)
        .expect("Failed to read ABI file");

    // Parse ABI JSON
    let abi_json: Value = serde_json::from_str(&abi_content)
        .expect("Failed to parse ABI JSON");

    // Extract ABI section
    let abi_str = abi_json["abi"].to_string();
    let abi_bytes = abi_str.as_bytes();

    // Load the contract
    let contract = Contract::from_json(web3.eth(), contract_address, abi_bytes)
        .expect("Failed to create contract instance");

    // Query number of candidates
    let candidates_count: u64 = contract
        .query("candidatesCount", (), None, Options::default(), None)
        .await
        .expect("Failed to fetch candidates count");

    println!("✅ Total Candidates: {}", candidates_count);
}
