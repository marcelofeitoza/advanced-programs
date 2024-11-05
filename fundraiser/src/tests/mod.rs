#[cfg(test)]
mod tests_module {
    use crate::state::Fundraiser;
    use mollusk_svm::{program, Mollusk};
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
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let slot = mollusk.sysvars.clock.slot + 200;

        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let (fundraiser_pda, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

        let data = [
            vec![0],
            mint.to_bytes().to_vec(),
            100_000_000u64.to_le_bytes().to_vec(),
            slot.to_le_bytes().to_vec(),
            1u8.to_le_bytes().to_vec(),
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

        let lamports = mollusk.sysvars.rent.minimum_balance(90);

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
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());
    }

    #[test]
    fn contribute() {
        // Setup and fundraiser initialization
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");

        let maker = Pubkey::new_unique();
        let fundraiser = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let slot = mollusk.sysvars.clock.slot + 200;

        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let (fundraiser_pda, bump) =
            Pubkey::try_find_program_address(&[fundraiser.as_ref()], &program_id).unwrap();

        let data = [
            vec![0],
            mint.to_bytes().to_vec(),
            100_000_000u64.to_le_bytes().to_vec(),
            slot.to_le_bytes().to_vec(),
            1u8.to_le_bytes().to_vec(),
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

        let lamports = mollusk.sysvars.rent.minimum_balance(90);

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
                (system_program.clone(), system_program_account.clone()),
            ],
        );

        assert!(!result.program_result.is_err());

        // Contribution
        let data = [vec![1], 1000u64.to_le_bytes().to_vec()].concat();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(fundraiser, false),
            ],
        );

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
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());
    }
}
