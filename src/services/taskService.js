import { atom, selector } from 'recoil';
import { initializeComputeEnvironment } from '../utils/webgpu';

// Task State Management
export const taskListState = atom({
  key: 'taskListState',
  default: []
});

export const activeTaskState = atom({
  key: 'activeTaskState',
  default: null
});

export const taskStatsState = selector({
  key: 'taskStatsState',
  get: ({ get }) => {
    const tasks = get(taskListState);
    return {
      completed: tasks.filter(t => t.status === 'completed').length,
      active: tasks.filter(t => t.status === 'active').length,
      totalEarnings: tasks.reduce((acc, t) => acc + (t.earnings || 0), 0)
    };
  }
});

class TaskService {
  constructor() {
    this.computeEnvironment = null;
  }

  async initialize() {
    this.computeEnvironment = await initializeComputeEnvironment();
  }

  async fetchAvailableTasks() {
    // Fetch tasks from backend
    const response = await fetch('https://api.neurolov.xyz/tasks');
    return await response.json();
  }

  async startTask(taskId) {
    if (!this.computeEnvironment) {
      await this.initialize();
    }

    // Implementation for starting a specific task
    const task = await fetch(`https://api.neurolov.xyz/tasks/${taskId}/start`, {
      method: 'POST'
    });

    return await task.json();
  }

  async submitTaskResult(taskId, result) {
    // Submit completed task results
    return await fetch(`https://api.neurolov.xyz/tasks/${taskId}/submit`, {
      method: 'POST',
      body: JSON.stringify(result)
    });
  }
}

export const taskService = new TaskService();
