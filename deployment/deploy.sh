#!/bin/bash

# Build and deploy smart contracts
echo "Deploying smart contracts..."
cd ../contracts
anchor build
anchor deploy --provider.cluster testnet

# Build and deploy backend
echo "Deploying backend services..."
cd ../deployment
docker-compose up -d

# Build and deploy frontend
echo "Building frontend..."
cd ../frontend
npm run build

echo "Deployment complete!"
