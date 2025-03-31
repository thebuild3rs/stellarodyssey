'use client';

import { useState } from 'react';
import { config } from '../config';

interface MissionCardProps {
  id: string;
  name: string;
  description: string;
  reward: {
    resource: string;
    amount: number;
  };
  progress: {
    stars: number;
    requiredStars: number;
    resources: { [key: string]: number };
    requiredResources: { [key: string]: number };
  };
  completed: boolean;
  onComplete?: () => void;
}

export default function MissionCard({
  name,
  description,
  reward,
  progress,
  completed,
  onComplete,
}: MissionCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const calculateProgress = () => {
    const starProgress = progress.stars / progress.requiredStars;
    const resourceProgress = Object.entries(progress.requiredResources).reduce(
      (acc, [resource, required]) => {
        const current = progress.resources[resource] || 0;
        return acc + (current / required);
      },
      0
    ) / Object.keys(progress.requiredResources).length;

    return (starProgress + resourceProgress) / 2 * 100;
  };

  return (
    <div className="bg-gray-800 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-shadow">
      <div className="p-4">
        <div className="flex justify-between items-start">
          <div>
            <h3 className="text-xl font-bold text-white">{name}</h3>
            <p className="text-sm text-gray-400">{description}</p>
          </div>
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="text-gray-400 hover:text-white"
          >
            {isExpanded ? '▼' : '▶'}
          </button>
        </div>

        <div className="mt-4">
          <div className="flex items-center">
            <div className="w-full h-2 bg-gray-700 rounded-full">
              <div
                className="h-full bg-blue-600 rounded-full"
                style={{ width: `${calculateProgress()}%` }}
              />
            </div>
            <span className="ml-2 text-sm text-gray-300">
              {Math.round(calculateProgress())}%
            </span>
          </div>
        </div>

        {isExpanded && (
          <div className="mt-4 pt-4 border-t border-gray-700">
            <div className="space-y-4">
              <div>
                <h4 className="text-sm font-semibold text-gray-300 mb-2">Star Progress</h4>
                <div className="flex items-center">
                  <div className="w-full h-2 bg-gray-700 rounded-full">
                    <div
                      className="h-full bg-green-600 rounded-full"
                      style={{ width: `${(progress.stars / progress.requiredStars) * 100}%` }}
                    />
                  </div>
                  <span className="ml-2 text-sm text-gray-300">
                    {progress.stars}/{progress.requiredStars}
                  </span>
                </div>
              </div>

              <div>
                <h4 className="text-sm font-semibold text-gray-300 mb-2">Resource Progress</h4>
                <div className="space-y-2">
                  {Object.entries(progress.requiredResources).map(([resource, required]) => {
                    const current = progress.resources[resource] || 0;
                    return (
                      <div key={resource} className="flex items-center">
                        <span className="text-sm text-gray-400 w-24">
                          {config.game.resources.find(r => r.id === resource)?.name || resource}
                        </span>
                        <div className="flex-1 h-2 bg-gray-700 rounded-full mx-2">
                          <div
                            className="h-full bg-green-600 rounded-full"
                            style={{ width: `${(current / required) * 100}%` }}
                          />
                        </div>
                        <span className="text-sm text-gray-300 w-16 text-right">
                          {current}/{required}
                        </span>
                      </div>
                    );
                  })}
                </div>
              </div>

              <div className="flex justify-between items-center">
                <div>
                  <span className="text-sm text-gray-400">Reward:</span>
                  <span className="ml-2 text-sm text-white">
                    {reward.amount} {config.game.resources.find(r => r.id === reward.resource)?.name || reward.resource}
                  </span>
                </div>
                {!completed && (
                  <button
                    onClick={onComplete}
                    className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 text-sm"
                  >
                    Complete Mission
                  </button>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
} 