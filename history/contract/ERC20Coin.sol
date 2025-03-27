// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract GLDToken is ERC20 {
    address public owner;

    constructor(uint256 initialSupply) ERC20("Gold", "GLD") {
        _mint(msg.sender, initialSupply);
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "permission denied");
        _;
    }

    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public override returns (bool) {
        _transfer(from, to, value);
        return true;
    }

    function mint(address user, uint256 amount) public {
        _mint(user, amount);
    }
}

contract SlvToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("Silver", "slv") {
        _mint(msg.sender, initialSupply);
    }

    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public override returns (bool) {
        _transfer(from, to, value);
        return true;
    }
}
