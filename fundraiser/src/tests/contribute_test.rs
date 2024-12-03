use crate::{state::Fundraiser, tests::setup};
use mollusk_svm::result::Check;
use pinocchio_token::state::TokenAccount;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    clock::Clock,
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

#[test]
fn contribute_test() {
    let (program_id, mollusk) = setup();
    let (token_program, token_program_account) = mollusk_token::token::keyed_account();

    let maker = Pubkey::new_from_array(five8_const::decode_32_const(
        "11111111111111111111111111111111111111111111",
    ));
    let signer = maker;
    let signer_account = crate::tests::create_account(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &program_id,
    );

    let (fundraiser_pda, bump) =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
    let (contributor_pda, _) = Pubkey::find_program_address(
        &[b"contributor", fundraiser_pda.as_ref(), signer.as_ref()],
        &program_id,
    );

    let contributor_account =
        crate::tests::create_account(mollusk.sysvars.rent.minimum_balance(0), 0, &program_id);
    let signer_ta = Pubkey::new_from_array(five8_const::decode_32_const(
        "33333333333333333333333333333333333333333333",
    ));
    let mint = Pubkey::new_from_array(five8_const::decode_32_const(
        "44444444444444444444444444444444444444444444",
    ));
    let vault = Pubkey::new_from_array(five8_const::decode_32_const(
        "55555555555555555555555555555555555555555555",
    ));

    let mut mint_account = crate::tests::pack_mint(&signer, 1_000_000);
    let mut mint_account_data = mint_account.data().to_vec();
    mint_account_data[36..44].copy_from_slice(&1_000_000u64.to_le_bytes());
    mint_account.set_data_from_slice(&mint_account_data);

    let signer_ta_account = crate::tests::pack_token_account(&signer, &mint, 1_000_000);
    let vault_account = crate::tests::pack_token_account(&fundraiser_pda, &mint, 0);

    let duration = 10u8;
    let init_data = [
        vec![0],                         // instruction discriminant
        maker.to_bytes().to_vec(),       // maker pubkey 0 - 32
        mint.to_bytes().to_vec(),        // mint pubkey 32 - 64
        100u64.to_le_bytes().to_vec(),   // amount_to_raise 64 - 72
        duration.to_le_bytes().to_vec(), // duration 72 - 73
        bump.to_le_bytes().to_vec(),     // bump 73 - 74
    ]
    .concat();

    let clock = Pubkey::from_str_const("SysvarC1ock11111111111111111111111111111111");

    let init_instruction = Instruction::new_with_bytes(
        program_id,
        &init_data,
        vec![
            AccountMeta::new(fundraiser_pda, false),
            AccountMeta::new(clock, false),
        ],
    );

    let init_accounts = vec![
        (
            fundraiser_pda,
            AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN),
                Fundraiser::LEN,
                &program_id,
            ),
        ),
        (
            clock,
            AccountSharedData::new(
                mollusk.sysvars.rent.minimum_balance(Clock::size_of()),
                Clock::size_of(),
                &solana_sdk::sysvar::id(),
            ),
        ),
    ];

    let init_result = mollusk.process_and_validate_instruction(
        &init_instruction,
        &init_accounts,
        &[Check::success()],
    );
    assert!(
        !init_result.program_result.is_err(),
        "Initialize fundraiser failed"
    );

    let fundraiser_account = init_result.get_account(&fundraiser_pda).unwrap();
    let fundraiser_data = fundraiser_account.data();
    println!("Fundraiser data:");
    println!(
        "Current amount: {}",
        u64::from_le_bytes(fundraiser_data[0..8].try_into().unwrap())
    );
    println!(
        "Time started: {}",
        i64::from_le_bytes(fundraiser_data[8..16].try_into().unwrap())
    );
    println!(
        "Maker: {}",
        Pubkey::new_from_array(fundraiser_data[16..48].try_into().unwrap())
    );
    println!(
        "Mint: {}",
        Pubkey::new_from_array(fundraiser_data[48..80].try_into().unwrap())
    );
    println!(
        "Amount to raise: {}",
        u64::from_le_bytes(fundraiser_data[80..88].try_into().unwrap())
    );
    println!(
        "Duration: {}",
        u8::from_le_bytes(fundraiser_data[88..89].try_into().unwrap())
    );
    println!(
        "Bump: {}",
        u8::from_le_bytes(fundraiser_data[89..90].try_into().unwrap())
    );

    let amount_to_contribute: u64 = 100;
    let contribute_data = [vec![1], amount_to_contribute.to_le_bytes().to_vec()].concat();

    let contribute_instruction = Instruction::new_with_bytes(
        program_id,
        &contribute_data,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(contributor_pda, true),
            AccountMeta::new(signer_ta, false),
            AccountMeta::new(fundraiser_pda, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(token_program, false),
        ],
    );

    let contribute_accounts = vec![
        (signer, signer_account.clone()),
        (contributor_pda, contributor_account),
        (signer_ta, signer_ta_account.clone()),
        (fundraiser_pda, fundraiser_account.clone()),
        (vault, vault_account.clone()),
        (token_program, token_program_account.clone()),
        (mint, mint_account.clone()),
    ];

    let contribute_result = mollusk.process_and_validate_instruction(
        &contribute_instruction,
        &contribute_accounts,
        &[Check::success()],
    );

    assert!(
        !contribute_result.program_result.is_err(),
        "Contribution failed"
    );

    let updated_vault_account = contribute_result.get_account(&vault).unwrap().clone();
    let updated_fundraiser_account = contribute_result
        .get_account(&fundraiser_pda)
        .unwrap()
        .clone();
    assert_eq!(
        u64::from_le_bytes(updated_fundraiser_account.data()[0..8].try_into().unwrap()),
        100,
        "Fundraiser balance after contribution"
    );

    let vault_balance = unsafe { TokenAccount::from_bytes(updated_vault_account.data()) }.amount();
    assert_eq!(
        vault_balance, 100,
        "Vault balance after contribution: {}",
        vault_balance
    );
}
