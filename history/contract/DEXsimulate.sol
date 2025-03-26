// SPDX-License-Identifier: MIT
//This is a simple contract that simulates a DEX (Decentralized Exchange) for swapping different tokens,
// where the value ratio of token A to token B is fixed at 1:2.
pragma solidity ^0.8.0;

interface IERC20 {
    function totalSupply() external view returns (uint256);

    function balanceOf(address account) external view returns (uint256);

    function transfer(
        address recipient,
        uint256 amount
    ) external returns (bool);

    function approve(address spender, uint256 amount) external returns (bool);

    function allowance(
        address owner,
        address spender
    ) external view returns (uint256);

    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) external returns (bool success);
}

contract DEXsimulate {
    address public owner;
    address public tokenA;
    address public tokenB;
    uint256 public totalSupplyA;
    uint256 public totalSupplyB;

    mapping(address => uint256) public liquidityA;
    mapping(address => uint256) public liquidityB;

    modifier onlyOwner() {
        require(msg.sender == owner, "permission denied");
        _;
    }

    // Events for logging
    event LiquidityAdded(
        address indexed user,
        uint256 amountA,
        uint256 amountB
    );
    event TokensSwapped(
        address indexed user,
        uint256 amountIn,
        uint256 amountOut
    );

    constructor(address _tokenA, address _tokenB) {
        owner = msg.sender;
        operation_state = true;
        tokenA = _tokenA;
        tokenB = _tokenB;
    }

    // Manage ownership
    event OwnerChanged(address oldOwner, address newOwner);

    function ChangeOwnership(address newOwner) public onlyOwner {
        emit OwnerChanged(owner, newOwner);
        owner = newOwner;
    }

    // Manage the running status.
    event No_longer_in_operation();
    event Restart_operation();

    bool public operation_state;

    function ChangeState() public onlyOwner {
        if (operation_state) {
            emit No_longer_in_operation();
        } else {
            emit Restart_operation();
        }
        operation_state = !operation_state;
    }

    modifier isOperating() {
        require(operation_state == true, "Contract is not in operation");
        _;
    }

    // Function to add liquidity
    function addLiquidity(uint256 amountA, uint256 amountB) public isOperating {
        // Ensure user has approved tokens to this contract
        require(
            IERC20(tokenA).transferFrom(msg.sender, address(this), amountA),
            "Failed to transfer tokenA for liquidity"
        );
        require(
            IERC20(tokenB).transferFrom(msg.sender, address(this), amountB),
            "Failed to transfer tokenB for liquidity"
        );

        liquidityA[msg.sender] += amountA;
        liquidityB[msg.sender] += amountB;
        totalSupplyA += amountA;
        totalSupplyB += amountB;

        emit LiquidityAdded(msg.sender, amountA, amountB);
    }

    // Function to swap tokenA for tokenB
    function swapAForB(uint256 amountA) public isOperating {
        require(amountA > 0, "Amount should be greater than zero");
        uint256 amountB = amountA * 2;

        // Ensure the contract has enough tokenB to fulfill the swap
        require(
            IERC20(tokenB).balanceOf(address(this)) >= amountB,
            "Not enough liquidity for swap"
        );

        // Transfer tokenA from user and send tokenB to user
        require(
            IERC20(tokenA).transferFrom(msg.sender, address(this), amountA),
            "Failed to transfer tokenA for swap"
        );
        require(
            IERC20(tokenB).transfer(msg.sender, amountB),
            "Failed to transfer tokenB for swap"
        );

        // Update liquidity and total supply
        liquidityA[msg.sender] += amountA;
        liquidityB[msg.sender] -= amountB;
        totalSupplyA += amountA;
        totalSupplyB -= amountB;

        emit TokensSwapped(msg.sender, amountA, amountB);
    }

    // Function to swap tokenB for tokenA
    function swapBForA(uint256 amountB) public isOperating {
        require(amountB > 0, "Amount should be greater than zero");
        uint256 amountA = amountB / 2;

        // Ensure the contract has enough tokenA to fulfill the swap
        require(
            IERC20(tokenA).balanceOf(address(this)) >= amountA,
            "Not enough liquidity for swap"
        );

        // Transfer tokenB from user and send tokenA to user
        require(
            IERC20(tokenB).transferFrom(msg.sender, address(this), amountB),
            "Failed to transfer tokenB for swap"
        );
        require(
            IERC20(tokenA).transfer(msg.sender, amountA),
            "Failed to transfer tokenA for swap"
        );

        // Update liquidity and total supply
        liquidityB[msg.sender] += amountB;
        liquidityA[msg.sender] -= amountA;
        totalSupplyB += amountB;
        totalSupplyA -= amountA;

        emit TokensSwapped(msg.sender, amountB, amountA);
    }
}
