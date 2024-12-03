// Signer account offsets
.equ SIGNER_LAMPORTS, 0x0050

// Account to close offsets
.equ ACCOUNT_OWNER_0, 0x2890
.equ ACCOUNT_OWNER_1, 0x2898
.equ ACCOUNT_OWNER_2, 0x28a0
.equ ACCOUNT_OWNER_3, 0x28a8
.equ ACCOUNT_LAMPORTS, 0x28b0
.equ ACCOUNT_DATA_LEN, 0x28b8

/// 
/// 
/// r0 = u64
/// r1 = *mut
/// 
/// 
/// 
/// pub unsafe extern "C" fn entrypoint(data: *mut u8) -> u64 {}

.globl entrypoint
entrypoint:
    // Store our Signer's balance
    ldxdw r2, [r1+SIGNER_LAMPORTS]
    // Store our Account's balance
    ldxdw r3, [r1+ACCOUNT_LAMPORTS]
    // Add out balances together
    add64 r2, r3
    // Store the new balance in our Signer's lamports
    stxdw [r1+SIGNER_LAMPORTS], r2

    // Set account data length to zero
    stxdw [r1+ACCOUNT_DATA_LEN], r0
    // Set account lamports to zero
    stxdw [r1+ACCOUNT_LAMPORTS], r0

    // Set account owner to System program 0..8
    stxdw [r1+ACCOUNT_OWNER_0], r0
    // Set account owner to System program 8..16
    stxdw [r1+ACCOUNT_OWNER_1], r0
    // Set account owner to System program 16..24
    stxdw [r1+ACCOUNT_OWNER_2], r0
    // Set account owner to System program 24..32
    stxdw [r1+ACCOUNT_OWNER_3], r0
    exit