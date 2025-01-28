import mongoose from 'mongoose';

const TaskSchema = new mongoose.Schema({
  data: {
    type: String,
    required: true
  },
  reward: {
    type: Number,
    required: true
  },
  status: {
    type: String,
    enum: ['available', 'assigned', 'completed'],
    default: 'available'
  },
  assignedTo: {
    type: String,
    ref: 'Swarm'
  },
  result: String,
  createdAt: {
    type: Date,
    default: Date.now
  }
});

export const TaskDB = mongoose.model('Task', TaskSchema);
