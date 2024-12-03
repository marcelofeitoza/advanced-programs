use crate::{state::Fundraiser, tests::setup};
use mollusk_svm::result::Check;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    clock::Clock,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar::Sysvar,
};

#[test]
pub fn initialize_test() {
    let (program_id, mollusk) = setup();

    let maker = Pubkey::new_from_array(five8_const::decode_32_const(
        "11111111111111111111111111111111111111111111",
    ));
    let signer = maker;

    let (fundraiser_pda, bump) =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);

    let mint = Pubkey::new_from_array(five8_const::decode_32_const(
        "44444444444444444444444444444444444444444444",
    ));

    let mut mint_account = crate::tests::pack_mint(&signer, 1_000_000);
    let mut mint_account_data = mint_account.data().to_vec();
    mint_account_data[36..44].copy_from_slice(&1_000_000u64.to_le_bytes());
    mint_account.set_data_from_slice(&mint_account_data);

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
}
