use crate::{
    cli::Args,
    utils::{
        display_position_balances, display_wallet_balances, fetch_position, fetch_whirlpool,
        send_transaction,
    },
};
use colored::Colorize;

use orca_whirlpools_client::{get_position_address, Position};
use orca_whirlpools_core::{sqrt_price_to_price, tick_index_to_price, tick_index_to_sqrt_price};
use solana_client::nonblocking::rpc_client::RpcClient;
use spl_token_2022::state::Mint;

use solana_sdk::{
    message::Message, program_pack::Pack, pubkey::Pubkey, signature::Signature, signer::Signer, transaction::Transaction,
};

use orca_whirlpools::{
    close_position_instructions, open_position_instructions, open_full_range_position_instructions, IncreaseLiquidityParam
};


pub async fn open_position(
    rpc: &RpcClient,
    whirlpool_address: Pubkey,
    args: &Args,
    wallet: &Box<dyn Signer>
) {
    let param = IncreaseLiquidityParam::TokenA(1);

    let instructions = open_full_range_position_instructions(
        &rpc,
        whirlpool_address,
        param,
        Some(100),
        Some(wallet.pubkey())
    ).await.unwrap();

    println!("Instructions: {:?}", instructions);

    let mut all_instructions = vec![];
    all_instructions.extend(instructions.instructions);

    let mut signers: Vec<&dyn Signer> = vec![wallet.as_ref()];
    signers.extend(
        instructions
            .additional_signers
            .iter()
            .map(|kp| kp as &dyn Signer),
    );

    let signature = send_transaction(
        &rpc,
        wallet.as_ref(),
        &whirlpool_address,
        all_instructions,
        signers,
        args.priority_fee_tier,
        args.max_priority_fee_lamports,
    )
    .await.unwrap();

    println!("Signature: {:?}", signature);
}