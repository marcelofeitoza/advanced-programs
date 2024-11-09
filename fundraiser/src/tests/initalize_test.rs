use crate::state::Fundraiser;
use crate::tests::setup;
use solana_sdk::account::ReadableAccount;
use solana_sdk::clock::Clock;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::{
    account::AccountSharedData,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

#[test]
pub fn initialize_test() {
    let (program_id, mollusk) = setup();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let (fundraiser, _) =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
    let mint = Pubkey::new_from_array([0x02; 32]);
    let clock = Pubkey::from_str_const("SysvarC1ock11111111111111111111111111111111");

    let time_started: i64 = 1_600_000_000;
    let duration = 1u8;

    let data = [
        vec![0],
        time_started.to_le_bytes().to_vec(),   // 0 - 8
        maker.to_bytes().to_vec(),             // 8 - 40
        mint.to_bytes().to_vec(),              // 40 - 72
        100_000_000u64.to_le_bytes().to_vec(), // 72 - 80
        duration.to_le_bytes().to_vec(),       // 80
        1u8.to_le_bytes().to_vec(),            // 81
    ]
    .concat();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(fundraiser, false),
        ],
    );

    let lamports = mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN);

    let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
        &instruction,
        &[
            (
                fundraiser,
                AccountSharedData::new(lamports, Fundraiser::LEN, &program_id),
            ),
        ],
    );
    assert!(
        !result.program_result.is_err(),
        "Initialize instruction failed."
    );

    let fundraiser_result_account = result
        .get_account(&fundraiser)
        .expect("Failed to find fundraiser account");
    let data = fundraiser_result_account.data();
    println!("{:?}", data);
    println!(
        "Amount to raise {} {}",
        i64::from_le_bytes(data[80..88].try_into().unwrap()),
        100_000_000u64
    );
}
