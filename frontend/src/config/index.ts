export const config = {
  // Network configuration
  network: {
    sorobanRpcUrl: 'https://soroban-testnet.stellar.org',
    networkPassphrase: 'Test SDF Network ; September 2015',
  },

  // Contract addresses (to be updated after deployment)
  contracts: {
    spaceGame: 'space_game_contract_id',
    starSystem: 'star_system_contract_id',
    missions: 'missions_contract_id',
    trading: 'trading_contract_id',
  },

  // Game configuration
  game: {
    shipTypes: [
      { id: 'EXPLORER', name: 'Explorer', cost: 100 },
      { id: 'MINER', name: 'Miner', cost: 200 },
      { id: 'TRADER', name: 'Trader', cost: 300 },
    ],
    resources: [
      { id: 'ENERGY', name: 'Energy' },
      { id: 'IRON', name: 'Iron' },
      { id: 'WATER', name: 'Water' },
      { id: 'CRYSTAL', name: 'Crystal' },
    ],
  },
}; 