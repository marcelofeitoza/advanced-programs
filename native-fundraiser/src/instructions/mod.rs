pub mod check;
pub mod contribute;
pub mod initialize;
pub mod refund;

// pub use check::*;
// pub use contribute::*;
pub use initialize::*;
// pub use refund::*;

use pinocchio::program_error::ProgramError;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FundraiserInstruction {
    InitializeInstruction = 0,
    ContributeInstruction = 1,
    CheckContributionsInstruction = 2,
    RefundInstruction = 3,
}

impl TryFrom<&u8> for FundraiserInstruction {
    type Error = ProgramError;
    fn try_from(instruction: &u8) -> Result<Self, ProgramError> {
        match instruction {
            0 => Ok(Self::InitializeInstruction),
            1 => Ok(Self::ContributeInstruction),
            2 => Ok(Self::CheckContributionsInstruction),
            3 => Ok(Self::RefundInstruction),
            _ => panic!("Wrong instruction"),
        }
    }
}
