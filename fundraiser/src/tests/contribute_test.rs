use solana_sdk::{
    account::ReadableAccount,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::program_pack::Pack;
use crate::state::Contributor;
use crate::{constants::MIN_AMOUNT_TO_RAISE, state::Fundraiser, tests::setup};

#[test]
fn contribute_test() {
    let (program_id, mollusk) = setup();
    let (token_program, token_program_account) = mollusk_token::token::keyed_account();

    let maker = Pubkey::new_from_array([0x1; 32]);
    let signer = Pubkey::new_from_array([0x2; 32]);
    let signer_account = crate::tests::create_account(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &program_id,
    );
    let signer_ta = Pubkey::new_from_array([0x3; 32]);
    let fundraiser = Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id).0;
    let contributor = Pubkey::find_program_address(
        &[b"contributor", fundraiser.as_ref(), signer.to_bytes().as_ref()],
        &program_id,
    )
        .0;
    let mint = Pubkey::new_from_array([0x4; 32]);
    let vault = Pubkey::new_from_array([0x5; 32]);

    let signer_ta_account = crate::tests::pack_token_account(&signer, &mint, 1_000_000);
    let vault_account = crate::tests::pack_token_account(&fundraiser, &mint, 0);

    let mut fundraiser_account = crate::tests::create_account(
        mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN),
        Fundraiser::LEN,
        &program_id,
    );
    let contributor_account = crate::tests::create_account(
        mollusk.sysvars.rent.minimum_balance(Contributor::LEN),
        Contributor::LEN,
        &program_id,
    );

    let end_time: i64 = (mollusk.sysvars.clock.slot + 200) as i64;
    fundraiser_account.set_data_from_slice(
        &[
            maker.to_bytes().to_vec(),
            mint.to_bytes().to_vec(),
            100_000_000u64.to_le_bytes().to_vec(),
            end_time.to_le_bytes().to_vec(),
            1u8.to_le_bytes().to_vec(),
        ]
            .concat(),
    );

    let amount = MIN_AMOUNT_TO_RAISE;
    let data = amount.to_le_bytes().to_vec();

    let contribute_instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(contributor, true),
            AccountMeta::new(signer_ta, false),
            AccountMeta::new(fundraiser, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(token_program, false),
        ],
    );

    let result = mollusk.process_instruction(
        &contribute_instruction,
        &[
            (signer, signer_account),
            (contributor, contributor_account),
            (signer_ta, signer_ta_account),
            (fundraiser, fundraiser_account.clone()),
            (vault, vault_account.clone()),
            (token_program, token_program_account),
        ],
    );
    assert!(
        !result.program_result.is_err(),
        "Contribute instruction failed."
    );
    println!("Compute Units: {}", result.compute_units_consumed);

    let fundraiser_result_account = result
        .get_account(&fundraiser)
        .expect("Failed to find fundraiser account");
    let data = fundraiser_result_account.data();
    assert_eq!(
        u64::from_le_bytes(data[64..72].try_into().unwrap()),
        100_000_000 - amount,
        "Remaining amount should be updated after contribution"
    );

    let contributor_result_account = result
        .get_account(&contributor)
        .expect("Failed to find contributor account");
    let data = contributor_result_account.data();
    assert_eq!(
        u64::from_le_bytes(data[0..8].try_into().unwrap()),
        amount,
        "Contributor amount should be updated after contribution"
    );
}