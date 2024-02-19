use bitcoincore_rpc::RpcApi;
use std::process::Command;

fn get_client(wallet: Option<&str>) -> bitcoincore_rpc::Client {
    let auth = bitcoincore_rpc::Auth::UserPass(
        "foo".to_string(),
        "qDDZdeQ5vw9XXFeVnXT4PZ--tGN2xNjjR4nrtyszZx0=".to_string(),
    );
    if let Some(wallet) = wallet {
        bitcoincore_rpc::Client::new(&format!("http://localhost:18443/wallet/{}", wallet), auth)
            .unwrap()
    } else {
        bitcoincore_rpc::Client::new("http://localhost:18443", auth).unwrap()
    }
}

fn main() {
    // start bitcoin node
    let mut command = Command::new("docker");
    command.arg("run")
        .arg("--name")
        .arg("bitcoin-server")
        .arg("--rm")
        .arg("-it")
        .arg("-d")
        .arg("-p18443:18443")
        .arg("-p18444:18444")
        .arg("ruimarinho/bitcoin-core") // Image name
        .arg("-printtoconsole") // Print to console
        .arg("-regtest=1") // Use regtest network
        .arg("-rpcallowip=0.0.0.0/0") // Allow RPC access from any IP
        .arg("-rpcbind=0.0.0.0") // Bind RPC to all interfaces
        .arg("-rpcauth=foo:7d9ba5ae63c3d4dc30583ff4fe65a67e$9e3634e81c11659e3de036d0bf88f89cd169c1039e6e09607562d54765c649cc");

    command
        .spawn()
        .expect("failed to execute command")
        .wait()
        .expect("failed to wait for process");

    // sleep for 30 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    // setup wallets
    setup_wallets();
}

fn setup_wallets() {
    let client = get_client(None);
    let wallets = client.list_wallets().expect("Bitcoin Core is not running");
    dbg!(&wallets);
    if !wallets.contains(&"sender".to_string()) {
        dbg!("creating sender wallet");
        assert!(client
            .create_wallet("sender", None, None, None, None)
            .is_ok());
    }
    if !wallets.contains(&"receiver".to_string()) {
        dbg!("creating receiver wallet");
        assert!(client
            .create_wallet("receiver", None, None, None, None)
            .is_ok());
    }

    let sender_client = get_client(Some("sender"));
    let receiver_client = get_client(Some("receiver"));
    let sender_balance = sender_client.get_balances().unwrap().mine.trusted.to_btc();
    if sender_balance == 0 as f64 {
        let sender_address = sender_client
            .get_new_address(None, None)
            .unwrap()
            .assume_checked();
        assert!(sender_client
            .generate_to_address(101, &sender_address)
            .is_ok());
    }
    let receiver_balance = receiver_client
        .get_balances()
        .unwrap()
        .mine
        .trusted
        .to_btc();

    if receiver_balance == 0 as f64 {
        let receiver_address = receiver_client
            .get_new_address(None, None)
            .unwrap()
            .assume_checked();
        assert!(receiver_client
            .generate_to_address(101, &receiver_address)
            .is_ok());
    }
    dbg!(
        "sender balance: {}",
        sender_client.get_balances().unwrap().mine.trusted.to_btc()
    );
    dbg!(
        "receiver balance: {}",
        receiver_client
        .get_balances()
        .unwrap()
        .mine
        .trusted
        .to_btc()
    );
}
