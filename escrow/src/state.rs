use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = std::mem::size_of::<Escrow>();
}
