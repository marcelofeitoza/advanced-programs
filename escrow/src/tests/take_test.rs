use crate::{
    state::Escrow,
    tests::{pack_mint, setup},
};
use mollusk_svm::result::Check;
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::AccountState;

#[test]
fn take_test() {
    let (program_id, mollusk) = setup();
    let (token_program, token_program_account) = mollusk_token::token::keyed_account();

    // Accounts
    let taker = Pubkey::new_unique();
    let taker_ta_a = Pubkey::new_unique();
    let taker_ta_b = Pubkey::new_unique();
    let maker_ta_b = Pubkey::new_unique();
    let escrow = Pubkey::new_unique();
    let vault = Pubkey::new_unique();
    let (authority, bump) =
        Pubkey::try_find_program_address(&[escrow.as_ref()], &program_id).unwrap();
    let maker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();

    // Fill out our account data
    let mut mint_a_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_a_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut mint_b_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut taker_ta_a_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_a,
            owner: taker,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        taker_ta_a_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut taker_ta_b_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_b,
            owner: taker,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        taker_ta_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut maker_ta_b_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_b,
            owner: maker,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        maker_ta_b_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut vault_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_a,
            owner: authority,
            amount: 1_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_account.data_as_mut_slice(),
    )
    .unwrap();

    let mut escrow_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(std::mem::size_of::<Escrow>()),
        std::mem::size_of::<Escrow>(),
        &program_id,
    );
    escrow_account.set_data_from_slice(
        &[
            maker.to_bytes().to_vec(),
            maker_ta_b.to_bytes().to_vec(),
            mint_a.to_bytes().to_vec(),
            mint_b.to_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ]
        .concat(),
    );

    // Data
    let data = [1, bump];

    // Instruction
    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(taker_ta_a, false),
            AccountMeta::new(taker_ta_b, false),
            AccountMeta::new(maker_ta_b, false),
            AccountMeta::new(escrow, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(authority, true),
            AccountMeta::new(token_program, false),
        ],
    );

    let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
        &instruction,
        &vec![
            (
                taker,
                AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
            ),
            (taker_ta_a, taker_ta_a_account),
            (taker_ta_b, taker_ta_b_account),
            (maker_ta_b, maker_ta_b_account),
            (escrow, escrow_account),
            (vault, vault_account),
            (authority, AccountSharedData::new(0, 0, &Pubkey::default())),
            (token_program, token_program_account),
        ],
    );

    assert!(!result.program_result.is_err());
}
