use processor::process_instruction;
use solana_program::{declare_id, entrypoint};

mod instructions;
mod processor;
mod state;

declare_id!("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);
