'use client';

import { useState } from 'react';
import ShipCard from '../components/ShipCard';
import StarSystemCard from '../components/StarSystemCard';
import MissionCard from '../components/MissionCard';
import TradeCard from '../components/TradeCard';
import WalletConnect from '../components/WalletConnect';
import { config } from '../config';

// Mock data for demonstration
const mockShips = [
  {
    name: 'Explorer I',
    type: 'EXPLORER',
    health: 85,
    cargo: 750,
    resources: {
      ENERGY: 500,
      IRON: 200,
      WATER: 300,
    },
  },
  {
    name: 'Miner Alpha',
    type: 'MINER',
    health: 95,
    cargo: 900,
    resources: {
      ENERGY: 400,
      IRON: 600,
      WATER: 200,
    },
  },
];

const mockStarSystems = [
  {
    name: 'Alpha Centauri',
    distance: 4.37,
    discovered: true,
    resources: {
      ENERGY: 1000,
      IRON: 500,
      WATER: 800,
    },
  },
  {
    name: 'Proxima Centauri',
    distance: 4.24,
    discovered: false,
    resources: {
      ENERGY: 1200,
      IRON: 600,
      WATER: 900,
    },
  },
];

const mockMissions = [
  {
    id: 'FIRST_STEPS',
    name: 'First Steps',
    description: 'Discover your first star system',
    reward: {
      resource: 'ENERGY',
      amount: 100,
    },
    progress: {
      stars: 1,
      requiredStars: 1,
      resources: {},
      requiredResources: {},
    },
    completed: false,
  },
  {
    id: 'RESOURCE_COLLECTOR',
    name: 'Resource Collector',
    description: 'Collect 1000 units of resources',
    reward: {
      resource: 'IRON',
      amount: 500,
    },
    progress: {
      stars: 1,
      requiredStars: 1,
      resources: {
        WATER: 800,
      },
      requiredResources: {
        WATER: 1000,
      },
    },
    completed: false,
  },
];

const mockTrades = [
  {
    id: 1,
    seller: '0x1234...5678',
    sellResource: 'ENERGY',
    sellAmount: 500,
    buyResource: 'IRON',
    buyAmount: 200,
  },
  {
    id: 2,
    seller: '0x8765...4321',
    sellResource: 'WATER',
    sellAmount: 300,
    buyResource: 'ENERGY',
    buyAmount: 400,
  },
];

export default function Home() {
  const [selectedTab, setSelectedTab] = useState('ships');
  const [showCreateShip, setShowCreateShip] = useState(false);
  const [showCreateTrade, setShowCreateTrade] = useState(false);

  return (
    <div className="space-y-8">
      <div className="flex justify-between items-center">
        <div className="flex space-x-4">
          <button
            onClick={() => setSelectedTab('ships')}
            className={`px-4 py-2 rounded ${
              selectedTab === 'ships' ? 'bg-blue-600' : 'bg-gray-700'
            }`}
          >
            Ships
          </button>
          <button
            onClick={() => setSelectedTab('stars')}
            className={`px-4 py-2 rounded ${
              selectedTab === 'stars' ? 'bg-blue-600' : 'bg-gray-700'
            }`}
          >
            Star Systems
          </button>
          <button
            onClick={() => setSelectedTab('missions')}
            className={`px-4 py-2 rounded ${
              selectedTab === 'missions' ? 'bg-blue-600' : 'bg-gray-700'
            }`}
          >
            Missions
          </button>
          <button
            onClick={() => setSelectedTab('trading')}
            className={`px-4 py-2 rounded ${
              selectedTab === 'trading' ? 'bg-blue-600' : 'bg-gray-700'
            }`}
          >
            Trading
          </button>
        </div>
        <WalletConnect />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {selectedTab === 'ships' && (
          <>
            <div className="p-6 bg-gray-800 rounded-lg">
              <h3 className="text-xl font-bold mb-4">Create New Ship</h3>
              <div className="space-y-4">
                {config.game.shipTypes.map((shipType) => (
                  <button
                    key={shipType.id}
                    onClick={() => setShowCreateShip(true)}
                    className="w-full p-4 bg-gray-700 rounded hover:bg-gray-600 text-left"
                  >
                    <div className="font-bold">{shipType.name}</div>
                    <div className="text-sm text-gray-400">Cost: {shipType.cost} ENERGY</div>
                  </button>
                ))}
              </div>
            </div>
            {mockShips.map((ship) => (
              <ShipCard
                key={ship.name}
                {...ship}
                onUpgrade={() => console.log('Upgrade ship:', ship.name)}
                onRepair={() => console.log('Repair ship:', ship.name)}
              />
            ))}
          </>
        )}

        {selectedTab === 'stars' && (
          <>
            <div className="p-6 bg-gray-800 rounded-lg">
              <h3 className="text-xl font-bold mb-4">Star Map</h3>
              <div className="aspect-square bg-gray-700 rounded relative">
                {/* Star map visualization would go here */}
                <div className="absolute inset-0 flex items-center justify-center text-gray-400">
                  Star Map Coming Soon
                </div>
              </div>
            </div>
            {mockStarSystems.map((star) => (
              <StarSystemCard
                key={star.name}
                {...star}
                onExplore={() => console.log('Explore star:', star.name)}
                onCollect={() => console.log('Collect from star:', star.name)}
              />
            ))}
          </>
        )}

        {selectedTab === 'missions' && (
          <>
            <div className="p-6 bg-gray-800 rounded-lg">
              <h3 className="text-xl font-bold mb-4">Mission Log</h3>
              <div className="space-y-4">
                <div className="p-4 bg-gray-700 rounded">
                  <h4 className="font-bold">Completed Missions</h4>
                  <p className="text-sm text-gray-400">None yet</p>
                </div>
                <div className="p-4 bg-gray-700 rounded">
                  <h4 className="font-bold">Mission Statistics</h4>
                  <div className="grid grid-cols-2 gap-2 mt-2">
                    <div>
                      <span className="text-sm text-gray-400">Total Completed:</span>
                      <span className="ml-2 text-sm text-white">0</span>
                    </div>
                    <div>
                      <span className="text-sm text-gray-400">Rewards Earned:</span>
                      <span className="ml-2 text-sm text-white">0</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            {mockMissions.map((mission) => (
              <MissionCard
                key={mission.id}
                {...mission}
                onComplete={() => console.log('Complete mission:', mission.id)}
              />
            ))}
          </>
        )}

        {selectedTab === 'trading' && (
          <>
            <div className="p-6 bg-gray-800 rounded-lg">
              <h3 className="text-xl font-bold mb-4">Create Trade Offer</h3>
              <button
                onClick={() => setShowCreateTrade(true)}
                className="w-full px-4 py-2 bg-green-600 rounded hover:bg-green-700"
              >
                New Trade
              </button>
            </div>
            {mockTrades.map((trade) => (
              <TradeCard
                key={trade.id}
                {...trade}
                onAccept={() => console.log('Accept trade:', trade.id)}
              />
            ))}
          </>
        )}
      </div>
    </div>
  );
} 