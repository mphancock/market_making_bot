mod cli;
mod position_manager;
mod utils;
mod wallet;

use clap::Parser;
use cli::Args;
use colored::Colorize;
use dotenv::dotenv;
use orca_whirlpools::{set_funder, set_whirlpools_config_address, WhirlpoolsConfigInput};
use orca_whirlpools_client::get_position_address;
use position_manager::run_position_manager;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;
use tokio::time::{sleep, Duration};
use utils::{
    display_position_balances, display_wallet_balances, fetch_mint, fetch_position, fetch_whirlpool,
};

use orca_whirlpools::{
    close_position_instructions, open_position_instructions, IncreaseLiquidityParam,
};


#[tokio::main]
async fn main() {
    let args = Args::parse();
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap();
    let rpc = RpcClient::new(rpc_url.to_string());
    set_whirlpools_config_address(WhirlpoolsConfigInput::SolanaMainnet)
        .expect("Failed to set Whirlpools config address for specified network.");
    let wallet = wallet::load_wallet();
    set_funder(wallet.pubkey()).expect("Failed to set funder address.");

    let position_mint_address = Pubkey::from_str(&args.position_mint_address)
        .expect("Invalid position mint address provided.");

    println!(
        "\n\
        ====================\n\
        🌀 Whirlpool LP BOT \n\
        ====================\n"
    );
    println!("Configuration:");
    println!(
        "  Position Mint Address: {}\n  Threshold: {:.2}bps\n  Interval: {} seconds\n  Priority Fee Tier: {:?}\n  Slippage tolerance bps: {:?}\n",
        args.position_mint_address, args.threshold, args.interval, args.priority_fee_tier, args.slippage_tolerance_bps
    );
    println!("RPC URL: {}", rpc_url);
    println!("WALLET PUB KEY: {}", wallet.pubkey());

    println!("-------------------------------------\n");

    let (position_address, _) =
        get_position_address(&position_mint_address).expect("Failed to derive position address.");
    let mut position = fetch_position(&rpc, &position_address)
        .await
        .expect("Failed to fetch position data.");
    let whirlpool = fetch_whirlpool(&rpc, &position.whirlpool)
        .await
        .expect("Failed to fetch Whirlpool data.");
    let token_mint_a = fetch_mint(&rpc, &whirlpool.token_mint_a)
        .await
        .expect("Failed to fetch Token Mint A data.");
    let token_mint_b = fetch_mint(&rpc, &whirlpool.token_mint_b)
        .await
        .expect("Failed to fetch Token Mint B data.");

    display_wallet_balances(
        &rpc,
        &wallet.pubkey(),
        &whirlpool.token_mint_a,
        &whirlpool.token_mint_b,
    )
    .await
    .expect("Failed to display wallet balances.");

    display_position_balances(
        &rpc,
        &position,
        &whirlpool.token_mint_a,
        &whirlpool.token_mint_b,
        token_mint_a.decimals,
        token_mint_b.decimals,
        args.slippage_tolerance_bps,
    )
    .await
    .expect("Failed to display position balances.");

    let whirlpool_pubkey = Pubkey::from_str("8wXA3oeY8EUpmHu2yqzr6k2WJEodTFLuKqTmoQJtM6wP").unwrap();
    let lower_price = 0.6;
    let upper_price = 0.61;

    // let liquidity_delta = convert_to_liquidity_delta(1, true)?;
    // let param = IncreaseLiquidityParam::TokenB(1);
    let param = IncreaseLiquidityParam::TokenB(1);

    let slippage_tolerance_bps = Some(100);

    // let wallet = Keypair::new();
    // let funder = Some(wallet.pubkey());
    // funder = Some(wallet.pubkey());
    let wallet_pub_key = wallet.pubkey();
    let result = open_position_instructions(
        &rpc,
        whirlpool_pubkey,
        lower_price,
        upper_price,
        param,
        slippage_tolerance_bps,
        None,
     )
     .await;

    println!("Result: {:?}", result);

    // println!("Position Mint: {:?}", result.position_mint);
    // println!("Initialization Cost: {} lamports", result.initialization_cost);


    // arguments are
    // rpc: &RpcClient,
    // pool_address: &Pubkey,
    // lower_price: f64,
    // upper_price: f64,
    // param: IncreaseLiquidityParam,
    // slippage_tolerance_bps: Option<u16>,
    // funder: Option<Pubkey>,
    // let open_position_instructions = open_position_instructions(
    //     rpc,
    //     whirlpool_address,
    //     new_lower_price,
    //     new_upper_price,
    //     increase_liquidity_param,
    //     Some(100),
    //     None,
    // )

    // // Replace these with the actual values relevant to your use case.
    // let token_a_amount: u64 = 1_000_000; // amount of token A you wish to deposit
    // let token_b_amount: u64 = 500_000;   // amount of token B you wish to deposit
    // let liquidity_amount: u128 = 100;    // liquidity amount to add
    // let token_min_a: u64 = 990_000;      // minimum amount of token A acceptable (to account for slippage)
    // let token_min_b: u64 = 495_000;      // minimum amount of token B acceptable (to account for slippage)

    // // If required, you may also need to specify additional parameters such as tick indexes:
    // // let tick_lower: i32 = ...;
    // // let tick_upper: i32 = ...;

    // let close_position_instructions = close_position_instructions(
    //     &rpc,
    //     &position.position_mint,
    //     Some(args.slippage_tolerance_bps),
    //     None,
    // )


    // let open_position_instructions = open_position_instructions(
    //     rpc,
    //     position_address,
    //     1,
    //     1.1,
    //     increase_liquidity_param,
    //     Some(100),
    //     None,
    // )

    // loop {
    //     if let Err(err) = run_position_manager(
    //         &rpc,
    //         &args,
    //         &wallet,
    //         &mut position,
    //         &token_mint_a,
    //         &token_mint_b,
    //     )
    //     .await
    //     {
    //         eprintln!("{}", format!("Error: {}", err).to_string().red());
    //     }
    //     sleep(Duration::from_secs(args.interval)).await;
    // }
}
