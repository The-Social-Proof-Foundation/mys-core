// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title IMySocialTokenBridgeAdapter
/// @notice Interface for the MySocialToken Bridge Adapter
/// @dev This adapter sits between the bridge and the MySocialToken contract,
/// providing controlled access to mint/burn functions while preserving token owner control
interface IMySocialTokenBridgeAdapter {
    /* ========== EVENTS ========== */

    /// @notice Emitted when a bridge address is authorized
    /// @param bridge The address of the authorized bridge
    event BridgeAuthorized(address indexed bridge);

    /// @notice Emitted when a bridge address is deauthorized
    /// @param bridge The address of the deauthorized bridge
    event BridgeDeauthorized(address indexed bridge);

    /// @notice Emitted when all bridge authorizations are revoked (emergency)
    event AllAuthorizationsRevoked();

    /// @notice Emitted when tokens are minted through the adapter
    /// @param recipient The address receiving the minted tokens
    /// @param amount The amount of tokens minted
    event TokensMinted(address indexed recipient, uint256 amount);

    /// @notice Emitted when tokens are burned through the adapter
    /// @param from The address whose tokens are burned
    /// @param amount The amount of tokens burned
    event TokensBurned(address indexed from, uint256 amount);

    /* ========== FUNCTIONS ========== */

    /// @notice Mints tokens to a recipient address
    /// @dev Can only be called by authorized bridge addresses
    /// @param recipient The address to receive the minted tokens
    /// @param amount The amount of tokens to mint
    function mint(address recipient, uint256 amount) external;

    /// @notice Burns tokens from a specific address
    /// @dev Can only be called by authorized bridge addresses
    /// @param from The address to burn tokens from
    /// @param amount The amount of tokens to burn
    function burnFrom(address from, uint256 amount) external;

    /// @notice Authorizes or deauthorizes a bridge address
    /// @dev Can only be called by the contract owner
    /// @param bridge The bridge address to authorize/deauthorize
    /// @param authorized True to authorize, false to deauthorize
    function setAuthorizedBridge(address bridge, bool authorized) external;

    /// @notice Emergency function to revoke all bridge authorizations
    /// @dev Can only be called by the contract owner
    function revokeAllAuthorizations() external;

    /// @notice Returns the list of all authorized bridge addresses
    /// @return An array of authorized bridge addresses
    function getAuthorizedBridges() external view returns (address[] memory);

    /// @notice Checks if a specific address is an authorized bridge
    /// @param bridge The address to check
    /// @return True if the address is authorized, false otherwise
    function isAuthorizedBridge(address bridge) external view returns (bool);

    /// @notice Returns the MySocialToken contract address
    /// @return The address of the MySocialToken contract
    function mySocialToken() external view returns (address);
}

