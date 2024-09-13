use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct FuzzmeSnapshot<'info> {
    pub payer: Signer<'info>
}
impl<'info> FuzzmeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let payer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("payer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("payer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("payer".to_string()))?;
        Ok(Self {payer})
    }
}
