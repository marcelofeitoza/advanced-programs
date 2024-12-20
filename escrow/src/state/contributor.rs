use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

pub struct Contributor(*mut u8);

impl Contributor {
    pub const LEN: usize = 8; // amount

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
    pub fn amount(&self) -> u64 {
        unsafe { *(self.0.add(0) as *const u64) } // 0..8
    }
}
