use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;

pub struct Fundraiser(*mut u8);

impl Fundraiser {
    // Total = 32 + 32 + 8 + 8 + 1 = 81
    pub const LEN: usize = 32 // maker
        + 32 // mint_to_raise
        + 8  // remaining_amount
        + 8  // end_time
        + 1; // bump

    #[inline(always)]
    pub fn init(&mut self, data: &[u8; Self::LEN]) -> ProgramResult {
        unsafe { *(self.0 as *mut [u8; Self::LEN]) = *data };
        Ok(())
    }

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_mut_data_unchecked().as_mut_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        assert_eq!(*account_info.owner(), crate::ID);
        assert_eq!(account_info.data_len(), Self::LEN);
        Ok(Self::from_account_info_unchecked(account_info))
    }

    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) } // 0 - 32
    }

    pub fn mint_to_raise(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) } // 32 - 64
    }

    pub fn remaining_amount(&self) -> u64 {
        unsafe { *(self.0.add(64) as *const u64) } // 64 - 72
    }

    pub fn end_time(&self) -> i64 {
        unsafe { *(self.0.add(72) as *const i64) } // 72 - 80
    }

    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(80) as *const u8) } // 80 - 81
    }

    pub fn set_remaining_amount(&mut self, value: u64) {
        unsafe { *(self.0.add(64) as *mut u64) = value }
    }
}
