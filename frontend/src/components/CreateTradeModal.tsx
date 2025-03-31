'use client';

import { useState } from 'react';
import Modal from './Modal';
import { config } from '../config';

interface CreateTradeModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreateTrade: (
    sellResource: string,
    sellAmount: number,
    buyResource: string,
    buyAmount: number
  ) => void;
}

export default function CreateTradeModal({
  isOpen,
  onClose,
  onCreateTrade,
}: CreateTradeModalProps) {
  const [sellResource, setSellResource] = useState('');
  const [sellAmount, setSellAmount] = useState('');
  const [buyResource, setBuyResource] = useState('');
  const [buyAmount, setBuyAmount] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (sellResource && sellAmount && buyResource && buyAmount) {
      onCreateTrade(
        sellResource,
        parseInt(sellAmount),
        buyResource,
        parseInt(buyAmount)
      );
      setSellResource('');
      setSellAmount('');
      setBuyResource('');
      setBuyAmount('');
      onClose();
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Create Trade Offer">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">
            Selling
          </label>
          <div className="grid grid-cols-2 gap-2">
            <select
              value={sellResource}
              onChange={(e) => setSellResource(e.target.value)}
              className="px-3 py-2 bg-gray-700 rounded text-white"
              required
            >
              <option value="">Select Resource</option>
              {config.game.resources.map((resource) => (
                <option key={resource.id} value={resource.id}>
                  {resource.name}
                </option>
              ))}
            </select>
            <input
              type="number"
              value={sellAmount}
              onChange={(e) => setSellAmount(e.target.value)}
              className="px-3 py-2 bg-gray-700 rounded text-white"
              placeholder="Amount"
              min="1"
              required
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1">
            Wanting
          </label>
          <div className="grid grid-cols-2 gap-2">
            <select
              value={buyResource}
              onChange={(e) => setBuyResource(e.target.value)}
              className="px-3 py-2 bg-gray-700 rounded text-white"
              required
            >
              <option value="">Select Resource</option>
              {config.game.resources.map((resource) => (
                <option key={resource.id} value={resource.id}>
                  {resource.name}
                </option>
              ))}
            </select>
            <input
              type="number"
              value={buyAmount}
              onChange={(e) => setBuyAmount(e.target.value)}
              className="px-3 py-2 bg-gray-700 rounded text-white"
              placeholder="Amount"
              min="1"
              required
            />
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
            Create Offer
          </button>
        </div>
      </form>
    </Modal>
  );
} 