pub mod check;
pub mod contribute;
pub mod initialize;
pub mod refund;

use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FundraiserInstruction {
    Initialize = 0,
    Contribute = 1,
    CheckContributions = 2,
    Refund = 3,
}

impl TryFrom<&u8> for FundraiserInstruction {
    type Error = ProgramError;

    fn try_from(instruction: &u8) -> Result<Self, ProgramError> {
        match instruction {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Contribute),
            2 => Ok(Self::CheckContributions),
            3 => Ok(Self::Refund),
            _ => panic!("Wrong instruction"),
        }
    }
}
