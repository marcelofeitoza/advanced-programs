// use solana_sdk::{
//     account::ReadableAccount,
//     instruction::{AccountMeta, Instruction},
//     program_pack::Pack,
//     pubkey::Pubkey,
// };
//
// use crate::state::Contributor;
// use crate::tests::utils;
// use crate::{constants::MIN_AMOUNT_TO_RAISE, state::Fundraiser, tests::setup};
//
// #[test]
// fn contribute_test() {
//     let (program_id, mollusk) = setup();
//     let (token_program, token_program_account) = mollusk_token::token::keyed_account();
//
//     let maker = Pubkey::new_from_array([0x1; 32]);
//     let signer = Pubkey::new_unique();
//     let signer_account = utils::create_account(
//         mollusk
//             .sysvars
//             .rent
//             .minimum_balance(spl_token::state::Account::LEN),
//         spl_token::state::Account::LEN,
//         &program_id,
//     );
//     let signer_ta = Pubkey::new_unique();
//     let fundraiser =
//         Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id).0;
//     let contributor = Pubkey::find_program_address(
//         &[
//             b"contributor",
//             fundraiser.as_ref(),
//             signer.to_bytes().as_ref(),
//         ],
//         &program_id,
//     )
//     .0;
//     let mint = Pubkey::new_unique();
//     let vault = Pubkey::new_unique();
//
//     let mint_account = utils::pack_mint(&contributor, 1_000_000);
//     let signer_ta_account = utils::pack_token_account(&signer, &mint, 1_000_000);
//     let vault_account = utils::pack_token_account(&fundraiser, &mint, 0);
//
//     let mut fundraiser_account = utils::create_account(
//         mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN),
//         Fundraiser::LEN,
//         &program_id,
//     );
//     let mut contributor_account = utils::create_account(
//         mollusk.sysvars.rent.minimum_balance(Contributor::LEN),
//         Contributor::LEN,
//         &program_id,
//     );
//
//     // let amount: u64 = unsafe { *(data.as_ptr() as *const u64) };
//     // assert!(amount >= MIN_AMOUNT_TO_RAISE, "Amount too low"); // 2 CUs
//     // let [signer, contributor, signer_ta, fundraiser, mint, vault, _token_program] = accounts else {
//     //     return Err(ProgramError::NotEnoughAccountKeys);
//     // };
//     // let fundraiser_account = Fundraiser::from_account_info_unchecked(
//     //     fundraiser);
//     // let contributor_account = Contributor::from_account_info_unchecked(contributor);
//     // assert_eq!(
//     //     &fundraiser_account.mint_to_raise(),
//     //     mint.key(),
//     //     "Invalid mint"
//     // ); // 28 CUs
//     // assert!(
//     //     fundraiser_account.time_started() > 0,
//     //     "Fundraiser not started yet"
//     // ); // 2 CUs
//     // assert!(
//     //     fundraiser_account.time_started() + i64::from(fundraiser_account.duration()) > 0,
//     //     "Fundraiser ended"
//     // ); // 4 CUs
//     // Transfer {
//     //     from: signer_ta,
//     //     to: vault,
//     //     authority: signer,
//     //     amount,
//     // }
//     //     .invoke()?; // 5951 CUs
//
//     let slot = mollusk.sysvars.clock.slot + 200;
//
//     fundraiser_account.set_data_from_slice(
//         &[
//             maker.to_bytes().to_vec(),
//             mint.to_bytes().to_vec(),
//             100_000_000u64.to_le_bytes().to_vec(),
//             0u64.to_le_bytes().to_vec(),
//             slot.to_le_bytes().to_vec(),
//             1u8.to_le_bytes().to_vec(),
//             0u8.to_le_bytes().to_vec(),
//         ]
//         .concat(),
//     );
//
//     let amount = MIN_AMOUNT_TO_RAISE;
//     let data = [vec![1], amount.to_le_bytes().to_vec()].concat();
//
//     let contribute_instruction = Instruction::new_with_bytes(
//         program_id,
//         &data,
//         vec![
//             AccountMeta::new(signer, true),
//             AccountMeta::new(contributor, true),
//             AccountMeta::new(signer_ta, false),
//             AccountMeta::new(fundraiser, false),
//             AccountMeta::new(mint, false),
//             AccountMeta::new(vault, false),
//             AccountMeta::new(token_program, false),
//         ],
//     );
//
//     let result = mollusk.process_instruction_chain(
//         &[contribute_instruction],
//         &vec![
//             (signer, signer_account),
//             (contributor, contributor_account),
//             (signer_ta, signer_ta_account),
//             (fundraiser, fundraiser_account.clone()),
//             (mint, mint_account),
//             (vault, vault_account.clone()),
//             (token_program, token_program_account),
//         ],
//     );
//
//     assert!(
//         !result.program_result.is_err(),
//         "Contribute instruction failed."
//     );
//
//     let fundraiser_result_account = result
//         .get_account(&fundraiser)
//         .expect("Failed to find fundraiser account");
//     let data = fundraiser_result_account.data();
//     println!("Fundraiser data:");
//     println!(
//         "Maker: {:?}",
//         Pubkey::new_from_array(data[0..32].try_into().unwrap())
//     );
//     println!(
//         "Mint to raise: {:?}",
//         Pubkey::new_from_array(data[32..64].try_into().unwrap())
//     );
//     println!(
//         "Amount to raise: {:?}",
//         u64::from_le_bytes(data[64..72].try_into().unwrap())
//     );
//     println!(
//         "Current amount: {:?}",
//         u64::from_le_bytes(data[72..80].try_into().unwrap())
//     );
//     println!(
//         "Time started: {:?}",
//         i64::from_le_bytes(data[80..88].try_into().unwrap())
//     );
//     println!(
//         "Duration: {:?}",
//         u8::from_le_bytes(data[88..89].try_into().unwrap())
//     );
//     println!(
//         "Bump seed: {:?}",
//         u8::from_le_bytes(data[89..90].try_into().unwrap())
//     );
//
//     let contributor_result_account = result
//         .get_account(&contributor)
//         .expect("Failed to find contributor account");
//     let data = contributor_result_account.data();
//     println!("Contributor data:");
//     println!(
//         "Amount: {:?}",
//         u64::from_le_bytes(data[0..8].try_into().unwrap())
//     );
// }
