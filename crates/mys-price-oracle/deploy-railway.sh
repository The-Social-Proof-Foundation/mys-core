#!/bin/bash

# Railway deployment script for mys-price-oracle
# Usage: ./scripts/deploy-railway.sh

set -e

echo "🚂 Deploying mys-price-oracle to Railway..."

# Check if Railway CLI is installed
if ! command -v railway &> /dev/null; then
    echo "❌ Railway CLI not found. Please install it first:"
    echo "   npm install -g @railway/cli"
    echo "   Or visit: https://docs.railway.app/develop/cli"
    exit 1
fi

# Check if user is logged in
if ! railway whoami &> /dev/null; then
    echo "❌ Not logged in to Railway. Please run:"
    echo "   railway login"
    exit 1
fi

echo "✅ Railway CLI found and authenticated"

# Ask for required environment variables
echo ""
echo "📋 Setting up required environment variables..."

if [[ -z "${MYS_ORACLE_SERVER_URL}" ]]; then
    read -p "Enter your bridge server URL (e.g., https://your-bridge.com): " BRIDGE_URL
    railway variables set MYS_ORACLE_SERVER_URL="$BRIDGE_URL"
else
    echo "✅ MYS_ORACLE_SERVER_URL already set"
fi

# Choose authentication method
echo ""
echo "🔐 Choose authentication method:"
echo "1) API Key (recommended for most users)"
echo "2) HMAC Secret (high security)"
read -p "Enter your choice (1 or 2): " AUTH_CHOICE

case $AUTH_CHOICE in
    1)
        if [[ -z "${MYS_ORACLE_AUTH_API_KEY}" ]]; then
            read -p "Enter your API key (or press Enter to generate one): " API_KEY
            if [[ -z "$API_KEY" ]]; then
                API_KEY=$(openssl rand -hex 32)
                echo "Generated API key: $API_KEY"
            fi
            railway variables set MYS_ORACLE_AUTH_API_KEY="$API_KEY"
        else
            echo "✅ MYS_ORACLE_AUTH_API_KEY already set"
        fi
        ;;
    2)
        if [[ -z "${MYS_ORACLE_AUTH_HMAC_SECRET}" ]]; then
            read -p "Enter your HMAC secret (or press Enter to generate one): " HMAC_SECRET
            if [[ -z "$HMAC_SECRET" ]]; then
                HMAC_SECRET=$(openssl rand -hex 64)
                echo "Generated HMAC secret: $HMAC_SECRET"
            fi
            railway variables set MYS_ORACLE_AUTH_HMAC_SECRET="$HMAC_SECRET"
        else
            echo "✅ MYS_ORACLE_AUTH_HMAC_SECRET already set"
        fi
        ;;
    *)
        echo "❌ Invalid choice. Exiting."
        exit 1
        ;;
esac

# Optional: Configure custom token
echo ""
read -p "Do you want to configure a custom token? (y/N): " CUSTOM_TOKEN
if [[ "$CUSTOM_TOKEN" =~ ^[Yy]$ ]]; then
    read -p "Enter token address (e.g., 0x...): " TOKEN_ADDRESS
    read -p "Enter chain ID (e.g., 1 for Ethereum, 8453 for Base): " CHAIN_ID
    read -p "Enter Uniswap V3 subgraph URL: " SUBGRAPH_URL
    
    railway variables set MYS_ORACLE_SOURCE_TOKEN_ADDRESS="$TOKEN_ADDRESS"
    railway variables set MYS_ORACLE_CHAIN_ID="$CHAIN_ID"
    railway variables set MYS_ORACLE_SOURCE_URL="$SUBGRAPH_URL"
fi

# Validate configuration before deploying
echo ""
echo "🔍 Validating configuration..."
if ! cargo run -- --env --validate-config; then
    echo "❌ Configuration validation failed. Please check your environment variables."
    exit 1
fi

echo "✅ Configuration validation passed"

# Deploy to Railway
echo ""
echo "🚀 Deploying to Railway..."
railway up

echo ""
echo "✅ Deployment initiated! 🎉"
echo ""
echo "📊 Monitor your deployment:"
echo "   • Logs: railway logs"
echo "   • Health: https://your-app.railway.app/health"
echo "   • Metrics: https://your-app.railway.app:9090/metrics"
echo ""
echo "⚠️  Important: Make sure to configure your bridge server to accept"
echo "   updates from your Railway app's domain." 