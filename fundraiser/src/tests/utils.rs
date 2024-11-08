use solana_sdk::account::{AccountSharedData, WritableAccount};
use solana_sdk::program_option::COption;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::AccountState;

pub fn create_account(lamports: u64, data_len: usize, owner: &Pubkey) -> AccountSharedData {
    AccountSharedData::new(lamports, data_len, owner)
}

pub fn pack_mint(mint_authority: &Pubkey, supply: u64) -> AccountSharedData {
    let mut account = create_account(0, spl_token::state::Mint::LEN, &spl_token::id());
    spl_token::state::Mint {
        mint_authority: COption::Some(*mint_authority),
        supply,
        decimals: 9,
        is_initialized: true,
        freeze_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());
    account
}

pub fn pack_token_account(owner: &Pubkey, mint: &Pubkey, amount: u64) -> AccountSharedData {
    let mut account = create_account(0, spl_token::state::Account::LEN, &spl_token::id());
    spl_token::state::Account {
        mint: *mint,
        owner: *owner,
        amount,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());
    account
}
