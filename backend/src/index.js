import express from 'express';
import cors from 'cors';
import { WebSocketServer } from 'ws';
import { createServer } from 'http';
import { taskRouter } from './routes/tasks.js';
import { swarmRouter } from './routes/swarms.js';
import { setupWebSocket } from './websocket/index.js';
import { connectDatabase } from './database/index.js';

const app = express();
const server = createServer(app);
const wss = new WebSocketServer({ server });

// Middleware
app.use(cors());
app.use(express.json());

// Routes
app.use('/api/tasks', taskRouter);
app.use('/api/swarms', swarmRouter);

// WebSocket setup
setupWebSocket(wss);

// Database connection
connectDatabase();

const PORT = process.env.PORT || 3001;
server.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});

export default server;
