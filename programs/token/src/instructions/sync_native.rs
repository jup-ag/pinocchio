use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

/// Given a native token account updates its amount field based
/// on the account's underlying `lamports`.
///
/// ### Accounts:
///   0. `[WRITE]`  The native token account to sync with its underlying
///      lamports.
pub struct SyncNative<'a> {
    /// Token Program Account.
    pub token_program: &'a AccountInfo, 
    /// Native Token Account
    pub native_token: &'a AccountInfo,
}

impl<'a> SyncNative<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 1] = [AccountMeta::writable(self.native_token.key())];

        let instruction = Instruction {
            program_id: self.token_program.key(),
            accounts: &account_metas,
            data: &[17],
        };

        invoke_signed(&instruction, &[self.native_token], signers)
    }
}
