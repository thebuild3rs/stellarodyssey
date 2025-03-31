'use client';

import { useState, useEffect } from 'react';
import { getWalletAddress } from '../utils/contracts';

export default function WalletConnect() {
  const [address, setAddress] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);

  const connectWallet = async () => {
    try {
      setIsConnecting(true);
      const walletAddress = await getWalletAddress();
      setAddress(walletAddress);
    } catch (error) {
      console.error('Error connecting wallet:', error);
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div>
      {!address ? (
        <button
          onClick={connectWallet}
          disabled={isConnecting}
          className="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400"
        >
          {isConnecting ? 'Connecting...' : 'Connect Wallet'}
        </button>
      ) : (
        <div className="flex items-center space-x-2">
          <span className="text-sm text-gray-300">
            {address.slice(0, 6)}...{address.slice(-4)}
          </span>
          <button
            onClick={() => setAddress(null)}
            className="px-2 py-1 text-sm rounded bg-red-600 hover:bg-red-700"
          >
            Disconnect
          </button>
        </div>
      )}
    </div>
  );
} 