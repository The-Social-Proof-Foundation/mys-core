// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./interfaces/IMySocialToken.sol";
import "./interfaces/IMySocialTokenBridgeAdapter.sol";

/// @title MySocialTokenBridgeAdapter
/// @notice Adapter contract that enables the bridge to interact with MySocialToken
/// @dev This contract acts as an intermediary between the bridge and MySocialToken,
/// allowing the bridge to mint/burn tokens while preserving the token owner's control.
/// The adapter is owned by the MySocialToken owner and only allows authorized bridge
/// addresses to call mint/burn functions.
contract MySocialTokenBridgeAdapter is IMySocialTokenBridgeAdapter, Ownable, ReentrancyGuard {
    /* ========== STATE VARIABLES ========== */

    /// @notice The MySocialToken contract (immutable)
    IMySocialToken private immutable _mySocialToken;

    /// @notice Mapping of authorized bridge addresses
    mapping(address => bool) public authorizedBridges;

    /// @notice Array to track all authorized bridges (for enumeration)
    address[] private _authorizedBridgesList;

    /* ========== CONSTRUCTOR ========== */

    /// @notice Constructs the adapter contract
    /// @param mySocialTokenAddress The address of the MySocialToken contract
    /// @param initialOwner The address that will own this adapter (should be token owner)
    constructor(address mySocialTokenAddress, address initialOwner) Ownable(initialOwner) {
        require(mySocialTokenAddress != address(0), "MySocialTokenBridgeAdapter: Invalid token address");
        require(initialOwner != address(0), "MySocialTokenBridgeAdapter: Invalid owner address");
        
        _mySocialToken = IMySocialToken(mySocialTokenAddress);
    }

    /* ========== EXTERNAL FUNCTIONS ========== */

    /// @notice Mints tokens to a recipient address
    /// @dev Can only be called by authorized bridge addresses
    /// @param recipient The address to receive the minted tokens
    /// @param amount The amount of tokens to mint
    function mint(address recipient, uint256 amount) external nonReentrant {
        require(authorizedBridges[msg.sender], "MySocialTokenBridgeAdapter: Caller is not authorized");
        require(recipient != address(0), "MySocialTokenBridgeAdapter: Invalid recipient");
        require(amount > 0, "MySocialTokenBridgeAdapter: Amount must be greater than 0");

        _mySocialToken.mint(recipient, amount);

        emit TokensMinted(recipient, amount);
    }

    /// @notice Burns tokens from a specific address
    /// @dev Can only be called by authorized bridge addresses
    /// The bridge must ensure the user has approved the adapter to spend their tokens
    /// @param from The address to burn tokens from
    /// @param amount The amount of tokens to burn
    function burnFrom(address from, uint256 amount) external nonReentrant {
        require(authorizedBridges[msg.sender], "MySocialTokenBridgeAdapter: Caller is not authorized");
        require(from != address(0), "MySocialTokenBridgeAdapter: Invalid address");
        require(amount > 0, "MySocialTokenBridgeAdapter: Amount must be greater than 0");

        _mySocialToken.burnFrom(from, amount);

        emit TokensBurned(from, amount);
    }

    /// @notice Authorizes or deauthorizes a bridge address
    /// @dev Can only be called by the contract owner
    /// @param bridge The bridge address to authorize/deauthorize
    /// @param authorized True to authorize, false to deauthorize
    function setAuthorizedBridge(address bridge, bool authorized) external onlyOwner {
        require(bridge != address(0), "MySocialTokenBridgeAdapter: Invalid bridge address");
        
        bool currentlyAuthorized = authorizedBridges[bridge];
        
        if (authorized && !currentlyAuthorized) {
            // Authorize the bridge
            authorizedBridges[bridge] = true;
            _authorizedBridgesList.push(bridge);
            emit BridgeAuthorized(bridge);
        } else if (!authorized && currentlyAuthorized) {
            // Deauthorize the bridge
            authorizedBridges[bridge] = false;
            _removeFromBridgesList(bridge);
            emit BridgeDeauthorized(bridge);
        }
        // If state is already as requested, do nothing
    }

    /// @notice Emergency function to revoke all bridge authorizations
    /// @dev Can only be called by the contract owner
    /// This immediately stops all bridge operations and can be used in case of compromise
    function revokeAllAuthorizations() external onlyOwner {
        // Deauthorize all bridges
        for (uint256 i = 0; i < _authorizedBridgesList.length; i++) {
            authorizedBridges[_authorizedBridgesList[i]] = false;
        }
        
        // Clear the list
        delete _authorizedBridgesList;
        
        emit AllAuthorizationsRevoked();
    }

    /* ========== VIEW FUNCTIONS ========== */

    /// @notice Returns the list of all authorized bridge addresses
    /// @return An array of authorized bridge addresses
    function getAuthorizedBridges() external view returns (address[] memory) {
        // Filter out any deauthorized addresses (in case some were removed)
        uint256 count = 0;
        for (uint256 i = 0; i < _authorizedBridgesList.length; i++) {
            if (authorizedBridges[_authorizedBridgesList[i]]) {
                count++;
            }
        }
        
        address[] memory activeBridges = new address[](count);
        uint256 index = 0;
        for (uint256 i = 0; i < _authorizedBridgesList.length; i++) {
            if (authorizedBridges[_authorizedBridgesList[i]]) {
                activeBridges[index] = _authorizedBridgesList[i];
                index++;
            }
        }
        
        return activeBridges;
    }

    /// @notice Checks if a specific address is an authorized bridge
    /// @param bridge The address to check
    /// @return True if the address is authorized, false otherwise
    function isAuthorizedBridge(address bridge) external view returns (bool) {
        return authorizedBridges[bridge];
    }

    /// @notice Returns the MySocialToken contract address
    /// @return The address of the MySocialToken contract
    function mySocialToken() external view returns (address) {
        return address(_mySocialToken);
    }

    /// @notice Returns the number of decimals the token uses
    /// @dev Required for IERC20Metadata compatibility
    /// @return The number of decimals (delegates to underlying MySocialToken)
    function decimals() external view returns (uint8) {
        return _mySocialToken.decimals();
    }

    /// @notice Returns the name of the token
    /// @dev Required for IERC20Metadata compatibility
    /// @return The token name (delegates to underlying MySocialToken)
    function name() external view returns (string memory) {
        return _mySocialToken.name();
    }

    /// @notice Returns the symbol of the token
    /// @dev Required for IERC20Metadata compatibility
    /// @return The token symbol (delegates to underlying MySocialToken)
    function symbol() external view returns (string memory) {
        return _mySocialToken.symbol();
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    /// @notice Removes a bridge address from the authorized bridges list
    /// @param bridge The bridge address to remove
    function _removeFromBridgesList(address bridge) private {
        for (uint256 i = 0; i < _authorizedBridgesList.length; i++) {
            if (_authorizedBridgesList[i] == bridge) {
                // Move the last element to this position and pop
                _authorizedBridgesList[i] = _authorizedBridgesList[_authorizedBridgesList.length - 1];
                _authorizedBridgesList.pop();
                break;
            }
        }
    }
}
