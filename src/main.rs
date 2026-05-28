mod block;
mod blockchain;
mod mining;
mod transaction;
mod wallet;

use block::Block;
use blockchain::Blockchain;
use transaction::Transaction;
use wallet::Wallet;

fn main() {
    println!("Money - Rust blockchain");
    println!("================================\n");

    println!("Creating wallets...");
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();
    let miner_wallet = Wallet::new();

    println!("Alice: {}", alice_wallet.address);
    println!("Bob:   {}", bob_wallet.address);
    println!("Miner: {}", miner_wallet.address);
    println!();

    println!("Blockchain initialization");
    let mut bc = Blockchain::new();

    bc.register_public_key(
        alice_wallet.address.clone(),
        alice_wallet.public_key_hex(),
    );
    bc.register_public_key(
        bob_wallet.address.clone(),
        bob_wallet.public_key_hex(),
    );
    bc.register_public_key(
        miner_wallet.address.clone(),
        miner_wallet.public_key_hex(),
    );

    ///giving alice start money coins
    bc.balances.insert(alice_wallet.address.clone(), 500.0);
    println!("Alice's balance: {} RTC", bc.get_balance(&alice_wallet.address));
    println!();


    println!("Creating and signing transactions...");

    ///alice sends 100 money coins to bob
    let tx1 = Transaction::new(
        alice_wallet.address.clone(),
        bob_wallet.address.clone(),
        100.0,
    );
    let tx1_signed = alice_wallet.sign_transaction(&tx1);

    ///alice sends 50 money coins to miner(f.ex. tax)
    let tx2 = Transaction::new(
        alice_wallet.address.clone(),
        miner_wallet.address.clone(),
        50.0,
    );
    let tx2_signed = alice_wallet.sign_transaction(&tx2);

    /// Adding transactions to mempool
    match bc.add_transaction(tx1_signed) {
        Ok(_) => println!("Success! Alice sent 100 money to Bob"),
        Err(e) => println!("TX1 error: {}", e),
    }

    match bc.add_transaction(tx2_signed) {
        Ok(_) => println!("Success! Alice sent 50 money to Miner"),
        Err(e) => println!("TX2 error: {}", e),
    }
    println!();

    ///Demonstration of mining first block
    println!("Mining the first block...");
    bc.mine_pending_transactions(miner_wallet.address.clone());
    Block::print_merkle_tree(&bc.last_block().transactions);
    println!();


    println!("Balances after first block:");
    println!("Alice: {:.2} RTC", bc.get_balance(&alice_wallet.address));
    println!("Bob:   {:.2} RTC", bc.get_balance(&bob_wallet.address));
    println!("Miner: {:.2} RTC", bc.get_balance(&miner_wallet.address));
    println!();


    println!("Bob sends money to Alice...");
    let tx3 = Transaction::new(
        bob_wallet.address.clone(),
        alice_wallet.address.clone(),
        30.0,
    );
    let tx3_signed = bob_wallet.sign_transaction(&tx3);

    match bc.add_transaction(tx3_signed) {
        Ok(_) => println!("Success! Bob sent to Alice 30 money"),
        Err(e) => println!("TX3 error: {}", e),
    }

    println!("\nMining second block...");
    bc.mine_pending_transactions(miner_wallet.address.clone());
    Block::print_merkle_tree(&bc.last_block().transactions);
    println!();


    bc.print_chain();


    println!("\nChecking the chain integrity...");
    bc.is_chain_valid();

    ///simulating the hacker attack on my blockchain
    println!("\nSimulating attack: changing data of block 1:");
    bc.chain[1].transactions[0].amount = 999999.0;  // Пытаемся изменить сумму!

    println!("Checking chain interity...");
    bc.is_chain_valid();

    println!("\nTesting the securities...");

    let bad_tx = Transaction::new(
        bob_wallet.address.clone(),
        alice_wallet.address.clone(),
        99999.0,
    );
    let bad_tx_signed = bob_wallet.sign_transaction(&bad_tx);
    match bc.add_transaction(bad_tx_signed) {
        Ok(_) => println!("Bad transaction succeeded"),
        Err(e) => println!("Security worked: {}", e),
    }
}