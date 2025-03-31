import React, { useState, useEffect } from 'react';
import {
  Box,
  Container,
  Grid,
  Paper,
  Typography,
  Button,
  Card,
  CardContent,
} from '@mui/material';
import { Server, Keypair, Asset } from 'stellar-sdk';

// Initialize Stellar testnet server
const server = new Server('https://horizon-testnet.stellar.org');

interface Ship {
  id: string;
  name: string;
  type: string;
  resources: {
    fuel: number;
    cargo: number;
  };
}

interface StarSystem {
  id: string;
  name: string;
  resources: string[];
  distance: number;
}

const Game = () => {
  const [playerShip, setPlayerShip] = useState<Ship | null>(null);
  const [currentSystem, setCurrentSystem] = useState<StarSystem | null>(null);
  const [nearbySystems, setNearbySystems] = useState<StarSystem[]>([]);
  const [stellarBalance, setStellarBalance] = useState<string>('0');

  useEffect(() => {
    // Initialize game state
    initializeGame();
  }, []);

  const initializeGame = async () => {
    try {
      // Create a new Stellar account for the player (in a real game, this would be handled securely)
      const keypair = Keypair.random();
      
      // Initialize player's ship
      setPlayerShip({
        id: '1',
        name: 'Explorer I',
        type: 'Scout',
        resources: {
          fuel: 100,
          cargo: 50,
        },
      });

      // Generate initial star system
      setCurrentSystem({
        id: '1',
        name: 'Alpha Centauri',
        resources: ['Iron', 'Water', 'Energy'],
        distance: 0,
      });

      // Generate nearby systems
      setNearbySystems([
        {
          id: '2',
          name: 'Proxima Centauri',
          resources: ['Gold', 'Water'],
          distance: 4.2,
        },
        {
          id: '3',
          name: 'Barnard\'s Star',
          resources: ['Iron', 'Energy'],
          distance: 5.9,
        },
      ]);

      // Get Stellar balance (in a real game, this would be the player's actual balance)
      setStellarBalance('1000');
    } catch (error) {
      console.error('Error initializing game:', error);
    }
  };

  const travelToSystem = (system: StarSystem) => {
    if (playerShip && playerShip.resources.fuel >= system.distance) {
      setCurrentSystem(system);
      setPlayerShip({
        ...playerShip,
        resources: {
          ...playerShip.resources,
          fuel: playerShip.resources.fuel - system.distance,
        },
      });
    }
  };

  return (
    <Container maxWidth="lg">
      <Box sx={{ mt: 4, mb: 4 }}>
        <Typography variant="h4" gutterBottom>
          Space Explorer
        </Typography>
        <Grid container spacing={3}>
          {/* Player Ship Status */}
          <Grid item xs={12} md={4}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Your Ship
              </Typography>
              {playerShip && (
                <>
                  <Typography>Name: {playerShip.name}</Typography>
                  <Typography>Type: {playerShip.type}</Typography>
                  <Typography>Fuel: {playerShip.resources.fuel}</Typography>
                  <Typography>Cargo: {playerShip.resources.cargo}</Typography>
                </>
              )}
            </Paper>
          </Grid>

          {/* Current System */}
          <Grid item xs={12} md={4}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Current System
              </Typography>
              {currentSystem && (
                <>
                  <Typography>Name: {currentSystem.name}</Typography>
                  <Typography>Resources:</Typography>
                  <ul>
                    {currentSystem.resources.map((resource) => (
                      <li key={resource}>{resource}</li>
                    ))}
                  </ul>
                </>
              )}
            </Paper>
          </Grid>

          {/* Stellar Balance */}
          <Grid item xs={12} md={4}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Stellar Balance
              </Typography>
              <Typography>{stellarBalance} XLM</Typography>
            </Paper>
          </Grid>

          {/* Nearby Systems */}
          <Grid item xs={12}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom>
                Nearby Systems
              </Typography>
              <Grid container spacing={2}>
                {nearbySystems.map((system) => (
                  <Grid item xs={12} sm={6} md={4} key={system.id}>
                    <Card>
                      <CardContent>
                        <Typography variant="h6">{system.name}</Typography>
                        <Typography>Distance: {system.distance} light years</Typography>
                        <Typography>Resources:</Typography>
                        <ul>
                          {system.resources.map((resource) => (
                            <li key={resource}>{resource}</li>
                          ))}
                        </ul>
                        <Button
                          variant="contained"
                          color="primary"
                          onClick={() => travelToSystem(system)}
                          disabled={!playerShip || playerShip.resources.fuel < system.distance}
                        >
                          Travel
                        </Button>
                      </CardContent>
                    </Card>
                  </Grid>
                ))}
              </Grid>
            </Paper>
          </Grid>
        </Grid>
      </Box>
    </Container>
  );
};

export default Game; 