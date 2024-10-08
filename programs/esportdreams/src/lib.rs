use anchor_lang::prelude::*;

declare_id!("FAmj5kvAaDgTdS1nzknYDDkS3hCYvAS2EJVNtHKfNqg6");

#[program]
pub mod esports_league {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.game_state.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn mint_player(ctx: Context<MintPlayer>, player_name: String) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.name = player_name;
        player.battles_won = 0;
        player.battles_lost = 0;
        player.daily_check_in = 0;
        player.xp = 0;
        player.power = 100;

        msg!("Minted player: {}", player.name);
        Ok(())
    }

    pub fn battle(ctx: Context<Battle>, _opponent: Pubkey) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let opponent_info = &ctx.accounts.opponent;
    
        let player_won = player.power > opponent_info.to_account_info().owner.as_ref()[0] as u64;
        if player_won {
            player.battles_won += 1;
            player.xp += 10;
            player.power += 5;
            msg!("Player won the battle!");
        } else {
            player.battles_lost += 1;
            player.xp += 5;
            msg!("Player lost the battle!");
        }
    
        Ok(())
    }

    pub fn daily_check_in(ctx: Context<DailyCheckIn>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let current_time = Clock::get()?.unix_timestamp;

        let one_day: i64 = 24 * 60 * 60;
        if current_time - player.daily_check_in > one_day {
            player.daily_check_in = current_time;
            player.xp += 50;
            player.power += 10;
            
            msg!("Daily check-in successful. Player rewarded!");
        } else {
            return Err(ErrorCode::CheckInTooEarly.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintPlayer<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(init, payer = user, space = 8 + 32 + 8 + 8 + 8 + 8 + 8)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Battle<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    /// CHECK: This is not dangerous as we don't write to this account
    pub opponent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DailyCheckIn<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
}

#[account]
pub struct GameState {
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Player {
    pub name: String,
    pub battles_won: u64,
    pub battles_lost: u64,
    pub daily_check_in: i64,
    pub xp: u64,
    pub power: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You can only check in once every 24 hours")]
    CheckInTooEarly,
}