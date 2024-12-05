pub mod make;
pub mod refund;
pub mod take;

use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EscrowInstruction {
    Make = 0,
    Take = 1,
    Refund = 2,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(instruction: &u8) -> Result<Self, ProgramError> {
        match instruction {
            0 => Ok(EscrowInstruction::Make),
            1 => Ok(EscrowInstruction::Take),
            2 => Ok(EscrowInstruction::Refund),
            _ => panic!("Wrong instruction"),
        }
    }
}
