import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';
import { TaskDB } from '../database/models/Task.js';
import { SwarmService } from './swarmService.js';

class TaskManager {
  constructor() {
    this.connection = new Connection(process.env.SOLANA_RPC_URL);
    this.program = new Program(/* program config */);
    this.swarmService = new SwarmService();
  }

  async createTask(data, reward) {
    const task = new TaskDB({
      data,
      reward,
      status: 'available'
    });

    await task.save();

    // Create task on-chain
    const transaction = await this.program.methods
      .createTask(data, reward)
      .rpc();

    return {
      task: task.toJSON(),
      transaction
    };
  }

  async assignTask(taskId, swarmId) {
    const task = await TaskDB.findById(taskId);
    const swarm = await this.swarmService.getSwarm(swarmId);

    task.status = 'assigned';
    task.assignedTo = swarmId;
    await task.save();

    this.swarmService.notifySwarm(swarmId, {
      type: 'TASK_ASSIGNED',
      task: task.toJSON()
    });

    return task;
  }

  async completeTask(taskId, result, walletAddress) {
    const task = await TaskDB.findById(taskId);
    
    // Verify result
    const isValid = await this.verifyTaskResult(result);
    if (!isValid) {
      throw new Error('Invalid task result');
    }

    // Update on-chain
    const transaction = await this.program.methods
      .completeTask(result)
      .accounts({
        task: new PublicKey(taskId),
        user: walletAddress,
      })
      .rpc();

    // Update in database
    task.status = 'completed';
    task.result = result;
    await task.save();

    return {
      task: task.toJSON(),
      transaction
    };
  }

  async verifyTaskResult(result) {
    // Implement result verification logic
    return true;
  }
}

export const taskManager = new TaskManager();
