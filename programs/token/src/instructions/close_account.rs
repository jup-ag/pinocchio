use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

/// Close an account by transferring all its SOL to the destination account.
///
/// ### Accounts:
///   0. `[WRITE]` The account to close.
///   1. `[WRITE]` The destination account.
///   2. `[SIGNER]` The account's owner.
pub struct CloseAccount<'a> {
    /// Token Program Account.
    pub token_program: &'a AccountInfo,
    /// Token Account.
    pub account: &'a AccountInfo,
    /// Destination Account
    pub destination: &'a AccountInfo,
    /// Owner Account
    pub authority: &'a AccountInfo,
}

impl<'a> CloseAccount<'a> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // account metadata
        let account_metas: [AccountMeta; 3] = [
            AccountMeta::writable(self.account.key()),
            AccountMeta::writable(self.destination.key()),
            AccountMeta::readonly_signer(self.authority.key()),
        ];

        let instruction = Instruction {
            program_id: self.token_program.key(),
            accounts: &account_metas,
            data: &[9],
        };

        invoke_signed(
            &instruction,
            &[self.account, self.destination, self.authority],
            signers,
        )
    }
}
