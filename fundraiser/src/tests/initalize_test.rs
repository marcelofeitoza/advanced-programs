// use crate::state::Fundraiser;
// use crate::tests::setup;
// use solana_sdk::account::{AccountSharedData, ReadableAccount};
// use solana_sdk::instruction::{AccountMeta, Instruction};
// use solana_sdk::pubkey::Pubkey;
//
// #[test]
// pub fn initialize_test() {
//     let (program_id, mollusk) = setup();
//
//     let maker = Pubkey::new_from_array([0x01; 32]);
//     let (fundraiser, _) =
//         Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
//     let mint = Pubkey::new_from_array([0x02; 32]);
//
//     let end_time: i64 = 1_600_000_000;
//     let bump = 1u8;
//
//     let data = [
//         vec![0],
//         mint.to_bytes().to_vec(),
//         100_000_000u64.to_le_bytes().to_vec(),
//         end_time.to_le_bytes().to_vec(),
//         bump.to_le_bytes().to_vec(),
//     ]
//     .concat();
//
//     let instruction = Instruction::new_with_bytes(
//         program_id,
//         &data,
//         vec![
//             AccountMeta::new(maker, true),
//             AccountMeta::new(fundraiser, false),
//         ],
//     );
//
//     let lamports = mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN);
//
//     let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
//         &instruction,
//         &[
//             (
//                 maker,
//                 AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
//             ),
//             (
//                 fundraiser,
//                 AccountSharedData::new(lamports, Fundraiser::LEN, &program_id),
//             ),
//         ],
//     );
//     assert!(
//         !result.program_result.is_err(),
//         "Initialize instruction failed."
//     );
//
//     let fundraiser_result_account = result
//         .get_account(&fundraiser)
//         .expect("Failed to find fundraiser account");
//     let data = fundraiser_result_account.data();
//     assert_eq!(
//         u64::from_le_bytes(data[64..72].try_into().unwrap()),
//         100_000_000
//     );
//     assert_eq!(
//         i64::from_le_bytes(data[72..80].try_into().unwrap()),
//         end_time
//     );
//     assert_eq!(u8::from_le_bytes(data[80..81].try_into().unwrap()), bump);
// }
