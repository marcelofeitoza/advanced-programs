use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::{signer, ProgramResult};
use pinocchio_token::instructions::Transfer;
use crate::state::{Contributor, Fundraiser};

pub fn process_refund_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [maker, fundraiser, contributor_account, contributor_ta, vault, token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
    let contributor_account = Contributor::from_account_info(contributor_account)?;
    let bump = fundraiser_account.bump();
    let fundraiser_seed = b"fundraiser".as_ref();
    let signer_key_seed = maker.key().as_ref();
    let bump_seed = &[bump];
    Transfer {
        from: vault,
        to: contributor_ta,
        authority: fundraiser,
        amount: contributor_account.amount(),
    }
        .invoke_signed(&[signer!(fundraiser_seed, signer_key_seed, bump_seed)])?;


    Ok(())
}
