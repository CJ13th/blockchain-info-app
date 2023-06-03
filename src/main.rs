#[macro_use]
extern crate serde_derive;
mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

use dotenv;
use std::{io, thread, time};

use crate::{
    blockchain_address::BlockchainAddress, blockchain_status::BlockchainStatus,
    blockchain_transaction::BlockchainTransaction,
};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    let coin = &blockchain_status.blockbook.coin;
    let chain = &blockchain_status.backend.chain;
    println!("\n\nQuerying {} - chain: {}\n\n", coin, chain);

    let blockchain_address: BlockchainAddress =
        blockchain_info::blockchain_address_request(address);

    println!(
        "\n\nAnalysing transactions for {} address {}",
        coin, &blockchain_address.address
    );

    let sleep_time = time::Duration::from_millis(2500);

    thread::sleep(sleep_time);

    println!(
        "\nYou have a total of {} transactions",
        &blockchain_address.txs
    );

    println!("\n Do you want to query these transactions? (y/n)\n");

    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("\n We will look up the following transactions:\n");
        println!("\n{:#?}", &blockchain_address.txids);
        thread::sleep(sleep_time);

        let mut balance: i32 = 0;
        for tx_id in &blockchain_address.txids {
            let mut subtotal_vin: i32 = 0; //senders balance
            let mut subtotal_vout: i32 = 0; // recipient amount received

            let blockchain_transaction: BlockchainTransaction =
                blockchain_info::blockchain_transaction_request(&tx_id);

            let match_address = String::from(address);

            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_address) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }
            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }
            balance += &subtotal_vout - &subtotal_vin;

            println!("-----------------------------------------------------");
            println!("TX ID:           {}", &blockchain_transaction.txid);
            println!("SATOSHIS IN:     {}", &subtotal_vout);
            println!("SATOSHIS OUT:    {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("-----------------------------------------------------");
        }

        println!("CURRENT BALANCE:     {}", &balance);
        println!("         IN BTC:     {}\n\n", balance as f32 * 0.00000001);

        assert_eq!(
            &blockchain_address.balance.parse::<i32>().unwrap(),
            &balance
        );
    }
}

fn main() {
    let entered_address = dotenv::var("ADDRESS").expect("Could not find wallet address");
    blockchain_info_app(&entered_address)
}
