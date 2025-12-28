use anchor_lang::prelude::*;

declare_id!("72LKA8UeGArYUVCPEmvuDmUnCph89f34u4tYqQ9VXrqc");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    // solana instruction handlers
    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();

        msg!(
            "User {:?} favorite color is {} and their favorite number is {}",
            user_public_key,
            color,
            number
        );
        msg!("Their hobbies are: {hobbies:?}");

        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account] // This macro marks the struct as an account
#[derive(InitSpace)] // This macro initializes the space of the struct
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    // Set the signer as a mut account, and since the user account's lamports are changed, we need to mark the field as mut
    pub user: Signer<'info>,

    #[account(init_if_needed,
            payer = user,
            space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
            seeds = [b"favorites", user.key().as_ref()],
            bump)]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
