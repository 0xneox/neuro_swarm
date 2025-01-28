import { atom } from 'recoil';
import { WebSocket } from 'ws';

export const swarmState = atom({
  key: 'swarmState',
  default: {
    id: null,
    members: [],
    totalPower: 0,
    activeTask: null
  }
});

class SwarmService {
  constructor() {
    this.ws = null;
    this.swarmId = null;
  }

  async connect(userId) {
    this.ws = new WebSocket('wss://swarm.neurolov.xyz');
    
    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.handleSwarmMessage(data);
    };

    return new Promise((resolve) => {
      this.ws.onopen = () => {
        this.joinSwarm(userId);
        resolve(true);
      };
    });
  }

  async joinSwarm(userId) {
    this.ws.send(JSON.stringify({
      type: 'JOIN_SWARM',
      userId
    }));
  }

  handleSwarmMessage(message) {
    switch (message.type) {
      case 'SWARM_JOINED':
        this.swarmId = message.swarmId;
        break;
      case 'TASK_ASSIGNED':
        // Handle new task assignment
        break;
      case 'MEMBER_UPDATE':
        // Handle member updates
        break;
    }
  }

  async leaveSwarm() {
    if (this.ws && this.swarmId) {
      this.ws.send(JSON.stringify({
        type: 'LEAVE_SWARM',
        swarmId: this.swarmId
      }));
    }
  }
}

export const swarmService = new SwarmService();
