# Stellar Space Game

A space exploration game built on the Stellar blockchain using Soroban smart contracts.

## Overview

This game allows players to:
- Create and manage spaceships
- Explore star systems and discover resources
- Complete missions and earn rewards
- Trade resources with other players

## Project Structure

```
├── frontend/               # Web interface
│   ├── src/
│   │   ├── app/          # Next.js app directory
│   │   ├── components/   # React components
│   │   ├── utils/        # Utility functions
│   │   └── config/       # Configuration files
│   └── package.json
└── src/
    └── contracts/        # Soroban smart contracts
        ├── space_game.rs    # Core game mechanics
        ├── star_system.rs   # Star system management
        ├── missions.rs      # Mission system
        └── trading.rs       # Trading system
```

## Prerequisites

- Node.js 18+ and npm
- Rust and Cargo
- Soroban CLI
- Freighter Wallet (browser extension)

## Setup

1. Install dependencies for smart contracts:
```bash
cargo build --target wasm32-unknown-unknown
```

2. Install dependencies for frontend:
```bash
cd frontend
npm install
```

3. Deploy smart contracts:
```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/space_game.wasm
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/star_system.wasm
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/missions.wasm
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/trading.wasm
```

4. Update contract IDs in `frontend/src/config/index.ts` with the deployed contract addresses.

5. Start the frontend development server:
```bash
cd frontend
npm run dev
```

## Smart Contracts

### Space Game Contract
- Player management
- Ship creation and management
- Resource collection

### Star System Contract
- Star system management
- Resource discovery
- Star exploration

### Missions Contract
- Mission creation and tracking
- Mission completion verification
- Reward distribution

### Trading Contract
- Resource trading
- Offer creation and management
- Trade execution

## Frontend

The web interface is built with:
- Next.js 14
- React
- TypeScript
- Tailwind CSS
- Soroban Client SDK
- Freighter Wallet Integration

## Development

1. Make changes to smart contracts
2. Build and deploy updated contracts
3. Update frontend configuration if needed
4. Test functionality in the web interface

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License 