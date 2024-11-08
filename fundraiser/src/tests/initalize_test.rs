use crate::state::Fundraiser;
use crate::tests::{setup, utils};
use solana_sdk::account::ReadableAccount;
use solana_sdk::{
    account::AccountSharedData,
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
};

#[test]
fn test_initialize() {
    let (program_id, mollusk) = setup();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let (fundraiser, _) =
        Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
    let mint = Pubkey::new_unique();
    let mint_account = utils::pack_mint(&maker, 1_000_000);

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
        &vec![
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
