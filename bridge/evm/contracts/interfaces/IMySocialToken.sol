// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";

/// @title IMySocialToken
/// @notice Interface for the MySocialToken ERC20 contract deployed on Base
/// @dev This interface defines the mint and burn functions that the adapter needs to call
interface IMySocialToken is IERC20Metadata {
    /// @notice Mints tokens to a specified address
    /// @dev Can only be called by the owner or presale contract
    /// @param to The address to mint tokens to
    /// @param amount The amount of tokens to mint
    function mint(address to, uint256 amount) external;

    /// @notice Burns tokens from a specified address
    /// @dev Can only be called by the owner or presale contract
    /// @param from The address to burn tokens from
    /// @param amount The amount of tokens to burn
    function burnFrom(address from, uint256 amount) external;

    /// @notice Returns the total supply cap of the token
    /// @return The maximum supply cap (1 billion tokens)
    function getTotalSupplyCap() external pure returns (uint256);
}

