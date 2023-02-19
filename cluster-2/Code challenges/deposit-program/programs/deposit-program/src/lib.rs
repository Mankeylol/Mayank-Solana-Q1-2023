use anchor_lang::{prelude::*};

declare_id!("");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.vault;
        deposit_account.owner = ctx.accounts.initializer.key();
        deposit_account.balance = 0;
        deposit_account.bump = *ctx.bumps.get("vault").unwrap();

        Ok(())
    }

    pub fn deposit (ctx: Context<Transfer>, deposit_amount: u64) -> Result<()> {

        let deposit_account = &mut ctx.accounts.vault;
        let owner_main_account = &mut ctx.accounts.owner;
        require_keys_eq!(owner_main_account.key(),deposit_account.owner,BankError::Unauthorized);
        require!(**owner_main_account.try_borrow_lamports()? >= deposit_amount, BankError::InsufficientFunds);
    
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &owner_main_account.key(),
            &deposit_account.key(),
            deposit_amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                owner_main_account.to_account_info(),
                deposit_account.to_account_info(),
            ],
        )?;

        deposit_account.balance += deposit_amount;

        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, withdraw_amount: u64) -> Result<()> {
        let receiver = &mut ctx.accounts.owner;
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(receiver.key(),vault.owner,BankError::Unauthorized);
        require!(withdraw_amount <= vault.balance, BankError::InsufficientFunds);

        **vault.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
        **receiver.try_borrow_mut_lamports()? += withdraw_amount;
        vault.balance -= withdraw_amount;

        Ok(())
    }
    pub fn withdraw_og (ctx: Context<Transfer>, withdraw_amount: u64) -> Result<()> {
        //doesn't work. 
        let deposit_account = &mut ctx.accounts.vault;
        let owner_main_account = &mut ctx.accounts.owner;
        
        require!(withdraw_amount <= deposit_account.balance, BankError::InsufficientFunds);

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &deposit_account.key(),
            &owner_main_account.key(),
            withdraw_amount,
        );

        let user_key = &owner_main_account.key();

        let seeds = &[
            "vault".as_ref(),
            user_key.as_ref(),
            &[deposit_account.bump],
        ];
        let signer = &[&seeds[..]];


        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                deposit_account.to_account_info(),
                owner_main_account.to_account_info()
            ],
            signer
        )?;

        deposit_account.balance -= withdraw_amount;

        Ok(())
    }

    pub fn close_account (ctx: Context<Close>) -> Result<()> {

        let deposit_account = &mut ctx.accounts.vault;
        let owner_main_account = &mut ctx.accounts.owner;

        // Withdraw 100%
        let withdraw_amount: u64 = deposit_account.balance;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &deposit_account.key(),
            &owner_main_account.key(),
            withdraw_amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                deposit_account.to_account_info(),
                owner_main_account.to_account_info()
            ],
        )?;

        // Inside of our context we use close = owner to close/collect rent at end of this function

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault".as_ref(), owner.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        // TO DO CONSTRAINT .owner = initializer
        mut,
        close = owner,
        seeds=[b"vault".as_ref(), owner.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init, 
        payer = initializer, 
        space = Vault::LEN,
        seeds=[b"vault".as_ref(), initializer.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    owner: Pubkey,
    bump: u8,
    balance: u64
    //token_account: Pubkey
}

impl Vault {
    const LEN: usize = 
        8 +     // discriminator
        1 +     // bump
        32 +    // owner
        8;      //balance
}

#[error_code]
pub enum BankError {
    #[msg("Trying to withdraw more funds than in the account")]
    InsufficientFunds,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}