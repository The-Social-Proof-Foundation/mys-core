// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../contracts/MySocialTokenBridgeAdapter.sol";

/// @title DeployMySocialTokenAdapter
/// @notice Deployment script for MySocialToken Bridge Adapter
/// @dev This script deploys the adapter contract that enables the bridge to interact with MySocialToken
contract DeployMySocialTokenAdapter is Script {
    
    /// @notice Main deployment function
    /// @dev Reads configuration from environment variables:
    /// - PRIVATE_KEY: Deployer private key (must be the token owner)
    /// - MYSOCIAL_TOKEN_ADDRESS: Address of the MySocialToken contract (default: Base mainnet address)
    /// - BRIDGE_ADDRESS: (Optional) Bridge address to authorize immediately after deployment
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // MySocialToken address - defaults to Base mainnet deployment
        address mySocialTokenAddress = vm.envOr(
            "MYSOCIAL_TOKEN_ADDRESS",
            address(0xFdD6013Bf2757018D8c087244f03e5a521B2d3B7)
        );
        
        // Optional: Bridge address to authorize immediately
        address bridgeAddress = vm.envOr("BRIDGE_ADDRESS", address(0));
        
        // Get deployer address (will be the adapter owner)
        address deployer = vm.addr(deployerPrivateKey);
        
        console.log("=== MySocialToken Bridge Adapter Deployment ===");
        console.log("Deployer (Token Owner):", deployer);
        console.log("MySocialToken Address:", mySocialTokenAddress);
        console.log("Chain ID:", block.chainid);
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy the adapter
        MySocialTokenBridgeAdapter adapter = new MySocialTokenBridgeAdapter(
            mySocialTokenAddress,
            deployer
        );
        
        console.log("\n[SUCCESS] Adapter deployed at:", address(adapter));
        
        // If bridge address is provided, authorize it immediately
        if (bridgeAddress != address(0)) {
            console.log("\nAuthorizing bridge:", bridgeAddress);
            adapter.setAuthorizedBridge(bridgeAddress, true);
            console.log("[SUCCESS] Bridge authorized");
        }
        
        vm.stopBroadcast();
        
        // Display post-deployment information
        console.log("\n=== Deployment Summary ===");
        console.log("Adapter Address:", address(adapter));
        console.log("MySocialToken:", adapter.mySocialToken());
        console.log("Adapter Owner:", deployer);
        
        if (bridgeAddress != address(0)) {
            console.log("Authorized Bridge:", bridgeAddress);
            console.log("Is Bridge Authorized:", adapter.isAuthorizedBridge(bridgeAddress));
        }
        
        console.log("\n=== Next Steps ===");
        console.log("1. Verify the adapter contract on block explorer");
        console.log("2. If not done during deployment, authorize the bridge:");
        console.log("   adapter.setAuthorizedBridge(BRIDGE_ADDRESS, true)");
        console.log("3. Configure the bridge to use this adapter:");
        console.log("   bridgeConfig.setupMySocialTokenAdapter(ADAPTER_ADDRESS, 9, PRICE)");
        console.log("\nFor detailed setup instructions, see: MYSOCIALTOKEN_BRIDGE_SETUP.md");
    }
}

