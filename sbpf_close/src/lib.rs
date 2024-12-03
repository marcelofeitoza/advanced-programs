#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn withdraw() {
        let program_id_keypair_bytes = std::fs::read("deploy/sbpf_close-keypair.json").unwrap()
            [..32]
            .try_into()
            .expect("slice with incorrect length");
        let program_id = Pubkey::new_from_array(program_id_keypair_bytes);

        let signer = Pubkey::new_unique();
        let account = Pubkey::new_unique();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(account, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "deploy/sbpf_close");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[
                (signer, AccountSharedData::new(0, 0, &Pubkey::default())),
                (
                    account,
                    AccountSharedData::new(1_000_000_000u64, 32, &program_id),
                ),
            ],
            &[
                Check::success(),
                Check::account(&signer).lamports(1_000_000_000).build(),
                Check::account(&account).lamports(0).build(),
            ],
        );
        assert!(!result.program_result.is_err());
    }
}
