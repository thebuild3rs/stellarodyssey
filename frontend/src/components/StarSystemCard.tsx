'use client';

import { useState } from 'react';
import { config } from '../config';

interface StarSystemCardProps {
  name: string;
  distance: number;
  discovered: boolean;
  resources: { [key: string]: number };
  onExplore?: () => void;
  onCollect?: () => void;
}

export default function StarSystemCard({
  name,
  distance,
  discovered,
  resources,
  onExplore,
  onCollect,
}: StarSystemCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  return (
    <div className="bg-gray-800 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-shadow">
      <div className="p-4">
        <div className="flex justify-between items-start">
          <div>
            <h3 className="text-xl font-bold text-white">{name}</h3>
            <p className="text-sm text-gray-400">{distance} light years</p>
          </div>
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="text-gray-400 hover:text-white"
          >
            {isExpanded ? '▼' : '▶'}
          </button>
        </div>

        <div className="mt-4">
          <span className={`px-2 py-1 rounded text-xs ${
            discovered
              ? 'bg-green-600 text-white'
              : 'bg-yellow-600 text-white'
          }`}>
            {discovered ? 'Discovered' : 'Unexplored'}
          </span>
        </div>

        {isExpanded && (
          <div className="mt-4 pt-4 border-t border-gray-700">
            <h4 className="text-sm font-semibold text-gray-300 mb-2">Resources</h4>
            <div className="grid grid-cols-2 gap-2">
              {Object.entries(resources).map(([resource, amount]) => (
                <div key={resource} className="flex justify-between items-center">
                  <span className="text-sm text-gray-400">
                    {config.game.resources.find(r => r.id === resource)?.name || resource}
                  </span>
                  <span className="text-sm text-white">{amount}</span>
                </div>
              ))}
            </div>

            <div className="mt-4 flex space-x-2">
              {!discovered && (
                <button
                  onClick={onExplore}
                  className="flex-1 px-3 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm"
                >
                  Explore
                </button>
              )}
              {discovered && (
                <button
                  onClick={onCollect}
                  className="flex-1 px-3 py-2 bg-green-600 text-white rounded hover:bg-green-700 text-sm"
                >
                  Collect Resources
                </button>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
} 