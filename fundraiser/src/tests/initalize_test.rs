use crate::state::Fundraiser;
use crate::tests::setup;
use solana_sdk::account::ReadableAccount;
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

    let time_started: i64 = 1_600_000_000;
    let duration = 1u8;

    let data = [
        vec![0],
        mint.to_bytes().to_vec(),
        100_000_000u64.to_le_bytes().to_vec(),
        time_started.to_le_bytes().to_vec(),
        duration.to_le_bytes().to_vec(),
    ]
    .concat();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new(fundraiser, false),
        ],
    );

    let lamports = mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN);

    let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
        &instruction,
        &[
            (
                maker,
                AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
            ),
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
    println!("Compute Units: {}", result.compute_units_consumed);

    let fundraiser_result_account = result
        .get_account(&fundraiser)
        .expect("Failed to find fundraiser account");
    let data = fundraiser_result_account.data();
    assert_eq!(u64::from_le_bytes(data[72..80].try_into().unwrap()), 0);
    assert_eq!(
        i64::from_le_bytes(data[80..88].try_into().unwrap()),
        time_started
    );
    assert_eq!(
        u8::from_le_bytes(data[88..89].try_into().unwrap()),
        duration
    );
    assert_eq!(u8::from_le_bytes(data[89..90].try_into().unwrap()), 0);
}
