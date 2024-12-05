use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub struct Escrow(*const u8);

impl Escrow {
    pub const LEN: usize = 32 // maker 0 - 32
                         + 32 // maker_ta_b 32 - 64
                         + 32 // mint_a 64 - 96
                         + 32 // mint_b 96 - 128
                         + 8; // amount_b 128 - 136

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    #[inline(always)]
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    #[inline(always)]
    pub fn maker_ta_b(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn mint_a(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn mint_b(&self) -> Pubkey {
        unsafe { *(self.0.add(96) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn amount_b(&self) -> u64 {
        unsafe { *(self.0.add(128) as *const u64) }
    }
}
