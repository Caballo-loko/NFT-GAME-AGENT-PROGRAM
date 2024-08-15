# Casino of Life: AI Agent NFT Registry and Dashboard

## Project Overview

Casino of Life is a Solana-based project that allows users to create and manage collections of AI agent models represented as NFTs. This project provides a registry for AI models and a dashboard for interacting with these models in various game-like scenarios.

## Key Features

- Create collections to group AI agent models
- Initialize AI agent accounts
- Mint NFTs representing AI agent models
- Associate AI agents with specific collections
- Support for various AI training strategies

## Smart Contracts

### 1. CreateCollection

Allows users to create a new collection for grouping AI agent models. It handles:
- Collection initialization with name, symbol, and strategy
- Strategy validation (supported strategies include DQN, A2C, PPO, DDPG, TD3, SAC, TRPO, REINFORCE)

### 2. InitializeAIAgentAccounts

Sets up the necessary accounts for a new AI agent, including:
- AI agent account initialization
- NFT mint account creation

### 3. MintAIAgent

Mints an NFT representing an AI agent model, including:
- Token minting
- Metadata creation using Metaplex standards
- Association with a collection

## Technology Stack

- Solana blockchain
- Anchor framework
- Metaplex for NFT standards
- Rust programming language

## Setup and Installation

(Include steps for setting up the development environment, installing dependencies, and building the project)

## Usage

(Provide instructions on how to interact with your contracts, including any CLI commands or SDK usage)

## Testing

The project includes four separate test files to ensure the reliability and correctness of the smart contracts. These tests cover various scenarios and edge cases for the main functionalities of the Casino of Life project.

To run the tests, use the script in the package.json file:

"test:meta-create-collection": 
"test:nft-mint": 

Of course, you can also run the tests using anchor test. You need to have configured your wallet for devnet or localnet. 

## Contributing

Contributions to the Casino of Life project are welcome. Please follow these steps:
1. Fork the repository
2. Create a new branch for your feature
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or support, please contact Caballo Loko.