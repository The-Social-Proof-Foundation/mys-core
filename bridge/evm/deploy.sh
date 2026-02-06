#!/bin/bash

# MySo Bridge Deployment Script for Base Sepolia
# Make sure you have set up your .env file with the required variables

set -e

echo "🚀 MySo Bridge Deployment to Base Sepolia"
echo "=========================================="

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ .env file not found!"
    echo "📋 Please create a .env file with the following variables:"
    echo ""
    echo "BASE_SEPOLIA_RPC_URL=https://sepolia.base.org"
    echo "BASESCAN_API_KEY=your_basescan_api_key_here"
    echo "PRIVATE_KEY=your_private_key_for_deployment"
    echo ""
    exit 1
fi

# Load environment variables
source .env

# Check required variables
if [ -z "$BASE_SEPOLIA_RPC_URL" ] || [ -z "$PRIVATE_KEY" ]; then
    echo "❌ Missing required environment variables!"
    echo "📋 Make sure your .env file contains:"
    echo "   - BASE_SEPOLIA_RPC_URL"
    echo "   - PRIVATE_KEY"
    exit 1
fi

echo "🔧 Environment check passed"
echo "📡 RPC URL: $BASE_SEPOLIA_RPC_URL"

# Build contracts
echo "🏗️  Building contracts..."
forge build

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful"

# Deploy contracts
echo "🚀 Deploying MySo Bridge contracts..."
echo "⏳ This may take a few minutes..."

forge script script/deploy_bridge.s.sol \
    --rpc-url $BASE_SEPOLIA_RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --chain-id 84532

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Deployment completed successfully!"
    echo ""
    echo "📋 Next steps:"
    echo "1. Save the deployed contract addresses from the output above"
    echo "2. Update your bridge node configuration with the contract addresses"
    echo "3. Deploy to Railway using the tutorial"
    echo ""
    echo "📄 Contract verification will happen automatically on BaseScan"
else
    echo "❌ Deployment failed!"
    echo "💡 Check the error messages above and try again"
    exit 1
fi 