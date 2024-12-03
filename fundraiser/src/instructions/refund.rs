use crate::state::{Contributor, Fundraiser};
use pinocchio::{account_info::AccountInfo, msg, program_error::ProgramError, signer, ProgramResult};
use pinocchio_token::instructions::Transfer;

pub fn process_refund_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [fundraiser, contributor_account, contributor_ta, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser)?;
    let contributor_account = Contributor::from_account_info(contributor_account)?;

    msg!("Contributor account amount: {}", contributor_account.amount());

    assert!(contributor_account.amount() > 0, "No amount to refund");

    let maker = fundraiser_account.maker();
    let bump = fundraiser_account.bump();
    let fundraiser_seed = b"fundraiser".as_ref();
    let signer_key_seed = maker.as_ref();
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
