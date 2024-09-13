use anchor_lang::prelude::*;

declare_id!("FuzzByLgWcN44Lbfhg91y6XVG2enqieuVZQot7p9mXUp");

#[program]
pub mod fuzzme {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;

    pub fn fuzzme(ctx: Context<Initialize>, value: u8) -> Result<u8> {
        msg!("Greetings from the program");

        if value >= 0xF0 {
            return err!(FuzzMeError::NumberTooHigh);
        }

        let mut result = value;

        if value == 0xDD {
            result = value/(value-value)
        };

        if value == 0x2a {
            let cpi_accounts = Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.receiver.to_account_info(),
            };
    
            let cpi_ctx = CpiContext::new(ctx.accounts.system_progrm.to_account_info(), cpi_accounts);
            transfer(cpi_ctx, ctx.accounts.payer.lamports())?;
        }


        Ok(result)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,
    pub system_progrm: Program<'info, System>
}

#[error_code]
pub enum FuzzMeError{
    FoundDead = 0,
    NumberTooHigh
}