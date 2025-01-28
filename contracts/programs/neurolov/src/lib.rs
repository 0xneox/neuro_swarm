use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};

declare_id!("Neu1o1ovTokenProgramID1111111111111111111");

#[program]
pub mod neurolov {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let task_manager = &mut ctx.accounts.task_manager;
        task_manager.authority = ctx.accounts.authority.key();
        task_manager.total_tasks = 0;
        Ok(())
    }

    pub fn create_task(ctx: Context<CreateTask>, task_data: String, reward: u64) -> Result<()> {
        let task_manager = &mut ctx.accounts.task_manager;
        let task = &mut ctx.accounts.task;

        task.id = task_manager.total_tasks;
        task.data = task_data;
        task.reward = reward;
        task.completed = false;
        task.assigned_to = None;

        task_manager.total_tasks += 1;
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>, result: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let token_program = &ctx.accounts.token_program;

        if !task.completed {
            task.completed = true;
            task.result = Some(result);

            // Transfer rewards
            let transfer_ctx = CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.reward_vault.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.task_manager.to_account_info(),
                },
            );

            token::transfer(transfer_ctx, task.reward)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8)]
    pub task_manager: Account<'info, TaskManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub task_manager: Account<'info, TaskManager>,
    #[account(init, payer = authority, space = 8 + 32 + 8 + 200)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub task_manager: Account<'info, TaskManager>,
    #[account(mut)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub reward_vault: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub user: Signer<'info>,
}

#[account]
pub struct TaskManager {
    pub authority: Pubkey,
    pub total_tasks: u64,
}

#[account]
pub struct Task {
    pub id: u64,
    pub data: String,
    pub reward: u64,
    pub completed: bool,
    pub assigned_to: Option<Pubkey>,
    pub result: Option<String>,
}
