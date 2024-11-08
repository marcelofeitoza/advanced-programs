#[cfg(test)]
mod contribute_test;
#[cfg(test)]
mod initalize_test;

use mollusk_svm::Mollusk;
use solana_sdk::pubkey::Pubkey;

mod check_test;
mod utils;

pub fn setup() -> (Pubkey, Mollusk) {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));
    let mut mollusk = Mollusk::new(&program_id, "../target/deploy/fundraiser");
    mollusk_token::token::add_program(&mut mollusk);
    (program_id, mollusk)
}
