use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub struct Fundraiser(*mut u8);

impl Fundraiser {
    // 8 + 8 + 32 + 32 + 8 + 1 + 1 = 90
    pub const LEN: usize = 8 // current_amount
        + 8  // time_started
        + 32 // maker
        + 32 // mint_to_raise
        + 8  // amount_to_raise
        + 1  // duration
        + 1; // bump

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_mut_data_unchecked().as_mut_ptr()) }
    }

    #[inline(always)]
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        assert_eq!(*account_info.owner(), crate::ID);
        assert_eq!(account_info.data_len(), Self::LEN);
        Ok(Self::from_account_info_unchecked(account_info))
    }

    #[inline(always)]
    pub fn current_amount(&self) -> u64 {
        unsafe { *(self.0.add(0) as *const u64) } // 0 - 8
    }
    // #[inline(always)]
    // pub fn time_started(&self) -> i64 {
    //     unsafe { *(self.0.add(8) as *const i64) } // 8 - 16
    // }
    #[inline(always)]
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0.add(16) as *const Pubkey) } // 16 - 48
    }
    // #[inline(always)]
    // pub fn duration(&self) -> u8 {
    //     unsafe { *(self.0.add(88) as *const u8) } // 88 - 89
    // }
    #[inline(always)]
    pub fn bump(&self) -> u8 {
        unsafe { *(self.0.add(89) as *const u8) } // 89 - 90
    }
}
