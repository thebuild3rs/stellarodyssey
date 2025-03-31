'use client';

import { useState } from 'react';
import Modal from './Modal';
import { config } from '../config';

interface CreateShipModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreateShip: (name: string, type: string) => void;
}

export default function CreateShipModal({
  isOpen,
  onClose,
  onCreateShip,
}: CreateShipModalProps) {
  const [name, setName] = useState('');
  const [selectedType, setSelectedType] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name && selectedType) {
      onCreateShip(name, selectedType);
      setName('');
      setSelectedType('');
      onClose();
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Create New Ship">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">
            Ship Name
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-3 py-2 bg-gray-700 rounded text-white"
            placeholder="Enter ship name"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">
            Ship Type
          </label>
          <div className="space-y-2">
            {config.game.shipTypes.map((type) => (
              <label
                key={type.id}
                className="flex items-center p-3 bg-gray-700 rounded cursor-pointer hover:bg-gray-600"
              >
                <input
                  type="radio"
                  name="shipType"
                  value={type.id}
                  checked={selectedType === type.id}
                  onChange={(e) => setSelectedType(e.target.value)}
                  className="mr-3"
                  required
                />
                <div>
                  <div className="font-medium">{type.name}</div>
                  <div className="text-sm text-gray-400">
                    Cost: {type.cost} ENERGY
                  </div>
                </div>
              </label>
            ))}
          </div>
        </div>

        <div className="flex justify-end space-x-3 mt-6">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 bg-gray-700 text-white rounded hover:bg-gray-600"
          >
            Cancel
          </button>
          <button
            type="submit"
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Create Ship
          </button>
        </div>
      </form>
    </Modal>
  );
} 