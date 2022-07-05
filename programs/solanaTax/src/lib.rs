use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer, Token};

declare_id!("EP1LCMGBnETvbEZRkwUQh8ttjjtwLik7wAsV6eHWVYcj");

#[program]
pub mod token_contract {
    use super::*;

    pub fn transfer_token(ctx: Context<TransferToken>,ammount: u64) -> Result<()> {
        // Create the Transfer struct for our context
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
        let transfer_instruction2 = Transfer{
            from: ctx.accounts.from.to_account_info(),
            //to: ctx.#[account(init, payer = "G4mGVtvsMrguS4emsW4bNaU6btojBAjqZcKhE1QoRWWH", space = 8+8)],
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info(); 
        let cpi_program2 = ctx.accounts.token_program.to_account_info();
        // Create the Context for our Transfer request
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);
        let cpi_ctx2 = CpiContext::new(cpi_program2, transfer_instruction2);

        // Execute anchor's helper function to transfer tokens
        let tax = (ammount*5)/100;
        anchor_spl::token::transfer(cpi_ctx, tax)?;
        anchor_spl::token::transfer(cpi_ctx2, ammount-tax)?;
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    // the authority of the from account 
    pub from_authority: Signer<'info>,
}