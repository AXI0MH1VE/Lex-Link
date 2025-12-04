#!/bin/bash
curl -f http://localhost:3000/health && echo "✅ Portal healthy" || echo "❌ Portal down"
curl -f http://localhost:3001/health && echo "✅ Audit healthy" || echo "❌ Audit down"
