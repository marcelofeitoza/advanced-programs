// use crate::{state::Escrow, tests::setup};
// use mollusk_svm::result::Check;
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     program_pack::Pack,
//     pubkey::Pubkey,
// };

// #[test]
// fn make_test() {
//     let (program_id, mollusk) = setup();

//     let mint_a = Pubkey::new_unique();
//     let mint_b = Pubkey::new_unique();

//     let maker = Pubkey::new_unique();
//     let _maker_ta_a = Pubkey::new_unique();
//     let maker_ta_b = Pubkey::new_unique();

//     let seed = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//     let (escrow, _escrow_bump) = Pubkey::find_program_address(
//         &[b"escrow", maker.as_ref(), &seed.to_le_bytes()],
//         &program_id,
//     );
//     let (vault, _) =
//         Pubkey::find_program_address(&[b"vault", escrow.as_ref(), maker.as_ref()], &program_id);

//     let maker_account = crate::tests::create_account(
//         mollusk
//             .sysvars
//             .rent
//             .minimum_balance(spl_token::state::Account::LEN),
//         spl_token::state::Account::LEN,
//         &program_id,
//     );

//     let _maker_ta_a_account = crate::tests::pack_token_account(&maker, &mint_a, 1_000_000);
//     let _maker_ta_b_account = crate::tests::pack_token_account(&maker, &mint_b, 1_000_000);

//     let escrow_account = crate::tests::create_account(
//         mollusk.sysvars.rent.minimum_balance(Escrow::LEN),
//         Escrow::LEN,
//         &program_id,
//     );
//     let _vault_account = crate::tests::pack_token_account(&vault, &mint_a, 0);

//     let amount = 100_u64;

//     let data = [
//         vec![0],
//         maker_ta_b.to_bytes().to_vec(), // 0 - 32
//         mint_a.to_bytes().to_vec(),     // 32 - 64
//         mint_b.to_bytes().to_vec(),     // 64 - 96
//         amount.to_le_bytes().to_vec(),  // 96 - 104
//     ]
//     .concat();

//     let instruction_accounts = vec![
//         AccountMeta::new(maker, true),
//         AccountMeta::new(escrow, false),
//     ];

//     let instruction = Instruction::new_with_bytes(program_id, &data, instruction_accounts);

//     let accounts = vec![(maker, maker_account), (escrow, escrow_account)];

//     let result =
//         mollusk.process_and_validate_instruction(&instruction, &accounts, &[Check::success()]);
//     assert!(!result.program_result.is_err());
// }
