#!/bin/bash
set -e
echo "Deploying AXIOM HIVE..."
docker-compose build
docker-compose up -d
echo "✅ Deployment complete"
