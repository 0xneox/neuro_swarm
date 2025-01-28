use anchor_lang::prelude::*;

#[program]
pub mod task_manager {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, task_data: TaskData) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.authority.key();
        task.data = task_data;
        task.status = TaskStatus::Active;
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>, result: Vec<u8>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.status = TaskStatus::Completed;
        task.result = result;
        // Trigger reward distribution
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TaskData {
    pub task_type: String,
    pub requirements: Vec<u8>,
    pub reward: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TaskStatus {
    Active,
    Completed,
    Failed,
}

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(init, payer = authority, space = 1000)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Task {
    pub owner: Pubkey,
    pub data: TaskData,
    pub status: TaskStatus,
    pub result: Vec<u8>,
}
