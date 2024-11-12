use anchor_lang::prelude::*;

declare_id!("9EEKqW83vxEStkjVQ44f9gMKyviySsxKFH8APC2XVVSg");

#[program]
pub mod my_solana_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.new_game_data_account.set_inner(GameDataAccount {
            player_position: 0
        });

        Ok(())
    }

    pub fn move_right(ctx: Context<MoveRight>) -> Result<()> {
        // const 
        let game_account = &mut ctx.accounts.new_game_data_account;
        game_account.player_position += 1;
        Ok(())
    }

    pub fn move_left(ctx: Context<MoveLeft>) -> Result<()> {
        let game_account = &mut ctx.accounts.new_game_data_account;
        game_account.player_position -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        // init_if_needed,
        init,
        seeds = [b"level1"],
        bump,
        payer = signer,
        space = 8+1
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>,

    #[account(
        mut
    )]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
    #[account(
        mut,
        seeds = [b"level1"],
        bump
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>
}

#[derive(Accounts)]
pub struct MoveLeft<'info> {
    #[account(
        mut,
        seeds = [b"level1"],
        bump
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>
}

#[account]
pub struct GameDataAccount {
    pub player_position: u8
}


