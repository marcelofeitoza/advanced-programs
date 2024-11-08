use crate::state::Fundraiser;
use crate::tests::{setup, utils};
use pinocchio_token::state::TokenAccount;
use solana_sdk::account::{AccountSharedData, ReadableAccount};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
};

#[test]
fn check_test() {
    let (program_id, mut mollusk) = setup();
    let (token_program, token_program_account) = mollusk_token::token::keyed_account();

    let maker = Pubkey::new_unique();
    let signer = maker;
    let signer_ta = Pubkey::new_unique();
    let fundraiser =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id).0;
    let mint = Pubkey::new_unique();
    let vault = Pubkey::new_unique();
    let mint_account = utils::pack_mint(&signer, 1_000_000);
    let signer_ta_account = utils::pack_token_account(&signer, &mint, 1_000_000);
    let vault_account = utils::pack_token_account(&fundraiser, &mint, 0);

    let mut fundraiser_account = AccountSharedData::new(
        mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN),
        Fundraiser::LEN,
        &program_id,
    );

    let amount_to_raise: u64 = 100;
    let current_amount: u64 = 0;
    let time_started = 200i64;
    let duration = 1u8;
    let bump = 1u8;

    fundraiser_account.set_data_from_slice(
        &[
            maker.to_bytes().to_vec(),
            mint.to_bytes().to_vec(),
            amount_to_raise.to_le_bytes().to_vec(),
            current_amount.to_le_bytes().to_vec(),
            time_started.to_le_bytes().to_vec(),
            duration.to_le_bytes().to_vec(),
            bump.to_le_bytes().to_vec(),
        ]
        .concat(),
    );

    let data = &[vec![2]].concat();

    let check_instruction = Instruction::new_with_bytes(
        program_id,
        data,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(signer_ta, false),
            AccountMeta::new(fundraiser, true),
            AccountMeta::new(vault, true),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    let result = mollusk.process_instruction(
        &check_instruction,
        &vec![
            (signer, AccountSharedData::new(1_000_000, 0, &program_id)),
            (signer_ta, signer_ta_account),
            (fundraiser, fundraiser_account.clone()),
            (vault, vault_account.clone()),
            (token_program, token_program_account),
        ],
    );
    assert!(
        !result.program_result.is_err(),
        "process_check_instruction failed."
    );

    let fundraiser_result_account = result
        .get_account(&fundraiser)
        .expect("Failed to find fundraiser account");
    let data = fundraiser_result_account.data();
    println!("Fundraiser data:");
    println!(
        "Maker: {:?}",
        Pubkey::new_from_array(data[0..32].try_into().unwrap())
    );
    println!(
        "Mint to raise: {:?}",
        Pubkey::new_from_array(data[32..64].try_into().unwrap())
    );
    println!(
        "Amount to raise: {:?}",
        u64::from_le_bytes(data[64..72].try_into().unwrap())
    );
    println!(
        "Current amount: {:?}",
        u64::from_le_bytes(data[72..80].try_into().unwrap())
    );
    println!(
        "Time started: {:?}",
        i64::from_le_bytes(data[80..88].try_into().unwrap())
    );
    println!(
        "Duration: {:?}",
        u8::from_le_bytes(data[88..89].try_into().unwrap())
    );
    println!(
        "Bump seed: {:?}",
        u8::from_le_bytes(data[89..90].try_into().unwrap())
    );
}
