use pinocchio::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

pub struct CreateIdempotent<'a> {
    pub funding_account: &'a AccountInfo,
    pub ata_account: &'a AccountInfo,
    pub wallet_address: &'a AccountInfo,
    pub token_mint: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> CreateIdempotent<'a> {
    pub fn invoke_signed(&self, signer_seeds: &[Signer]) -> ProgramResult {
        let account_metas = [
            AccountMeta::writable_signer(self.funding_account.key()),
            AccountMeta::writable(self.ata_account.key()),
            AccountMeta::readonly(self.wallet_address.key()),
            AccountMeta::readonly(self.token_mint.key()),
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.token_program.key()),
        ];

        let instruction_data = [1];

        let instruction = Instruction {
            program_id: &crate::ID,
            data: &instruction_data,
            accounts: &account_metas,
        };

        invoke_signed(
            &instruction,
            &[
                self.funding_account,
                self.ata_account,
                self.wallet_address,
                self.system_program,
                self.token_program,
            ],
            signer_seeds,
        )?;

        Ok(())
    }
}
