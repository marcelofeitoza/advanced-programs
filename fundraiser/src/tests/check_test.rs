use crate::state::{Contributor, Fundraiser};
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
    let signer_account = utils::create_account(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &program_id,
    );
    let signer_ta = Pubkey::new_unique();
    let fundraiser =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id).0;
    let contributor = Pubkey::find_program_address(
        &[
            b"contributor",
            fundraiser.as_ref(),
            signer.to_bytes().as_ref(),
        ],
        &program_id,
    )
    .0;
    let mint = Pubkey::new_unique();
    let vault = Pubkey::new_unique();

    let mut mint_account = utils::pack_mint(&signer, 1_000_000);
    let mut mint_account_data = mint_account.data().to_vec();
    mint_account_data[36..44].copy_from_slice(&1_000_000u64.to_le_bytes());
    mint_account.set_data_from_slice(&mint_account_data);

    let signer_ta_account = utils::pack_token_account(&signer, &mint, 1_000_000);
    let vault_account = utils::pack_token_account(&fundraiser, &mint, 0);

    let amount_to_raise: u64 = 100;
    let current_amount: u64 = 0;
    let time_started = 200i64;
    let duration = 1u8;
    let bump = 1u8;

    let mut fundraiser_account = AccountSharedData::new(
        mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN),
        Fundraiser::LEN,
        &program_id,
    );
    let mut contributor_account = utils::create_account(
        mollusk.sysvars.rent.minimum_balance(Contributor::LEN),
        Contributor::LEN,
        &program_id,
    );

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

    assert_eq!(
        fundraiser_account.lamports(),
        mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN)
    );
    assert_eq!(fundraiser_account.data().len(), Fundraiser::LEN);

    let amount_to_contribute: u64 = 100; // all the amount_to_raise
    let contribute_data = [vec![1], amount_to_contribute.to_le_bytes().to_vec()].concat();

    let contribute_instruction = Instruction::new_with_bytes(
        program_id,
        &contribute_data,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(contributor, true),
            AccountMeta::new(signer_ta, false),
            AccountMeta::new(fundraiser, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(token_program, false),
        ],
    );

    let result = mollusk.process_instruction(
        &contribute_instruction,
        &vec![
            (signer.clone(), signer_account.clone()),
            (contributor, contributor_account),
            (signer_ta, signer_ta_account.clone()),
            (fundraiser, fundraiser_account.clone()),
            (mint, mint_account),
            (vault, vault_account.clone()),
            (token_program.clone(), token_program_account.clone()),
        ],
    );
    assert!(
        !result.program_result.is_err(),
        "process_contribute_instruction failed."
    );

    let fundraiser_result_account = result
        .get_account(&fundraiser)
        .expect("Failed to find fundraiser account");
    let fundraiser_data = fundraiser_result_account.data();
    println!(
        "Amount to raise: {:?}",
        u64::from_le_bytes(fundraiser_data[64..72].try_into().unwrap())
    );
    println!(
        "Current amount: {:?}",
        u64::from_le_bytes(fundraiser_data[72..80].try_into().unwrap())
    );
    assert_eq!(
        u64::from_le_bytes(fundraiser_data[72..80].try_into().unwrap()),
        amount_to_contribute,
        "Current amount should be updated after contribution"
    );

    let check_instruction = Instruction::new_with_bytes(
        program_id,
        &[vec![2]].concat(),
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(signer_ta, false),
            AccountMeta::new(fundraiser, true),
            AccountMeta::new(vault, true),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    let vault_result_account = result
        .get_account(&vault)
        .expect("Failed to find vault account");
    let vault_data = vault_result_account.data();
    let vault_ta_before = unsafe { TokenAccount::from_bytes(vault_data) };
    println!("Vault balance before: {:?}", vault_ta_before.amount());

    let signer_ta_result_account = result
        .get_account(&signer_ta)
        .expect("Failed to find signer_ta account");
    let signer_ta_data = signer_ta_result_account.data();
    let signer_ta_before = unsafe { TokenAccount::from_bytes(signer_ta_data) };
    println!(
        "Signer Token Account balance before: {:?}",
        signer_ta_before.amount()
    );

    let result = mollusk.process_instruction(
        &check_instruction,
        &vec![
            (signer, AccountSharedData::new(1_000_000, 0, &program_id)),
            (signer_ta, signer_ta_account),
            (fundraiser, fundraiser_account.clone()),
            (vault, vault_account),
            (token_program, token_program_account),
        ],
    );
    assert!(
        !result.program_result.is_err(),
        "process_check_instruction failed."
    );

    let vault_result_account = result
        .get_account(&vault)
        .expect("Failed to find vault account");
    let vault_data = vault_result_account.data();
    let vault_ta_after = unsafe { TokenAccount::from_bytes(vault_data) };
    println!("Vault balance: {:?}", vault_ta_after.amount());

    let signer_ta_result_account = result
        .get_account(&signer_ta)
        .expect("Failed to find signer_ta account");
    let signer_ta_data = signer_ta_result_account.data();
    let signer_ta_after = unsafe { TokenAccount::from_bytes(signer_ta_data) };
    println!(
        "Signer Token Account balance: {:?}",
        signer_ta_after.amount()
    );

    assert_eq!(
        vault_ta_after.amount(),
        0,
        "Vault balance should be 0 after transfer"
    );
    assert_eq!(
        signer_ta_after.amount(),
        1_000_000,
        "Signer Token Account should have received the raised amount"
    );
}
