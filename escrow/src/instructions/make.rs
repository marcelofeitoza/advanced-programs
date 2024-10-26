use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};
use spl_token::instruction::transfer_checked;

use crate::{processor::EscrowArgs, state::Escrow};

pub fn make(program_id: &Pubkey, accounts: &[AccountInfo], args: EscrowArgs) -> ProgramResult {
    let [maker, mint_a, escrow, maker_ta_a, vault, token_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(spl_token::check_id(token_program.key));
    assert!(system_program::check_id(system_program.key));
    assert!(crate::check_id(program_id));
    assert!(maker.is_signer);

    let mint_unpacked = spl_token::state::Mint::unpack(&mint_a.data.borrow())?;

    let escrow_seeds: &[&[u8]] = &[b"escrow", maker.key.as_ref()];

    let expected_escrow = Pubkey::create_program_address(escrow_seeds, program_id)?;

    invoke_signed(
        &system_instruction::create_account(
            maker.key,
            escrow.key,
            Rent::get()?.minimum_balance(Escrow::LEN),
            Escrow::LEN as u64,
            &crate::id(),
        ),
        accounts,
        &[escrow_seeds],
    );

    let new_escrow = Escrow {
        maker: *maker.key,
        mint_a: *mint_a.key,
        mint_b: args.mint_b,
        receive: args.receive,
        bump: args.escrow_bump,
    };

    let mut escrow_data = bytemuck::try_from_bytes_mut::<Escrow>(&mut escrow.data.borrow_mut())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    escrow_data.clone_from(&new_escrow);

    if vault.data_is_empty() {
        invoke(
            &transfer_checked(
                token_program.key,
                maker.key,
                mint_a.key,
                vault.key,
                maker.key,
                &[],
                args.amount,
                mint_unpacked.decimals,
            )?,
            &[maker.clone(), maker_ta_a.clone()],
        );
    }

    Ok(())
}
