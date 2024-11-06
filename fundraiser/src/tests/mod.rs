#[cfg(test)]
mod tests_module {
    use crate::state::Fundraiser;
    use mollusk_svm::{program, Mollusk};
    use solana_sdk::account::ReadableAccount;
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        let maker = Pubkey::new_unique();
        let (fundraiser, _) =
            Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
        let mint = Pubkey::new_unique();

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

        let fundraiser_data = result.get_account(&fundraiser).unwrap().data();
        assert_eq!(fundraiser_data.len(), Fundraiser::LEN);
        assert_eq!(&fundraiser_data[0..32], maker.to_bytes(), "Maker mismatch"); // Maker
        assert_eq!(&fundraiser_data[32..64], mint.to_bytes(), "Mint mismatch"); // Mint
        assert_eq!(
            &fundraiser_data[64..72],
            &100_000_000u64.to_le_bytes(),
            "Amount to raise mismatch"
        ); // Amount to raise
        assert_eq!(
            &fundraiser_data[72..80],
            &0u64.to_le_bytes(),
            "Current amount mismatch"
        ); // Current amount
        assert_eq!(
            &fundraiser_data[80..88],
            &time_started.to_le_bytes(),
            "Time started mismatch"
        ); // Time started
        assert_eq!(
            &fundraiser_data[88..89],
            &duration.to_le_bytes(),
            "Duration mismatch"
        ); // Duration
        assert_eq!(&fundraiser_data[89..90], &[0u8], "Bump mismatch"); // Bump
    }

    #[test]
    fn contribute() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        let maker = Pubkey::new_unique();
        let (fundraiser, _) =
            Pubkey::find_program_address(&[b"fundraiser", &maker.to_bytes()], &program_id);
        let mint = Pubkey::new_unique();
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let time_started: i64 = 1_600_000_000;
        let duration = 1u8;
        let amount_to_raise = 100_000_000_u64;

        let initialize_data = [
            vec![0],
            mint.to_bytes().to_vec(),
            amount_to_raise.to_le_bytes().to_vec(),
            time_started.to_le_bytes().to_vec(),
            duration.to_le_bytes().to_vec(),
        ]
        .concat();

        let initialize_instruction = Instruction::new_with_bytes(
            program_id,
            &initialize_data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, false),
            ],
        );

        let lamports = mollusk.sysvars.rent.minimum_balance(Fundraiser::LEN);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &initialize_instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    fundraiser,
                    AccountSharedData::new(lamports, Fundraiser::LEN, &program_id),
                ),
                (system_program, system_program_account.clone()),
            ],
        );
        assert!(
            !result.program_result.is_err(),
            "Initialize instruction failed."
        );

        println!(
            "Amount to raise: {}",
            u64::from_le_bytes(
                result.get_account(&fundraiser).unwrap().data()[64..72]
                    .try_into()
                    .unwrap()
            )
        );
        println!(
            "Current amount: {}",
            u64::from_le_bytes(
                result.get_account(&fundraiser).unwrap().data()[72..80]
                    .try_into()
                    .unwrap()
            )
        );

        let amount_to_contribute = 1000u64.to_le_bytes();

        let contributor = Pubkey::new_unique();
        let contributor_ata = Pubkey::new_unique();
        let (contributor_account, _) = Pubkey::find_program_address(
            &[
                b"contributor",
                &fundraiser.to_bytes(),
                &contributor.to_bytes(),
            ],
            &program_id,
        );
        let vault = Pubkey::new_unique();

        let contribute_data = vec![vec![1], amount_to_contribute.to_vec()].concat();

        let contribute_instruction = Instruction::new_with_bytes(
            program_id,
            &contribute_data,
            vec![
                AccountMeta::new(contributor, true),
                AccountMeta::new(mint, false),
                AccountMeta::new(fundraiser, false),
                AccountMeta::new(contributor_ata, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(contributor_account, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &contribute_instruction,
            &vec![
                (
                    contributor,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    mint,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    fundraiser,
                    AccountSharedData::new(lamports, Fundraiser::LEN, &program_id),
                ),
                (
                    contributor_ata,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    vault,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (
                    contributor_account,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (system_program, system_program_account.clone()),
            ],
        );
        assert!(
            !result.program_result.is_err(),
            "Contribute instruction failed."
        );

        let fundraiser_data_after = result.get_account(&fundraiser).unwrap().data();
        println!(
            "Amount to raise: {}\nCurrent amount after contribution: {}\nExpected amount to add: {}",
            u64::from_le_bytes(fundraiser_data_after[64..72].try_into().unwrap()),
            u64::from_le_bytes(fundraiser_data_after[72..80].try_into().unwrap()),
            u64::from_le_bytes(amount_to_contribute)
        );
    }
}
