import React from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Box,
  Button,
  Container,
  Typography,
  Paper,
  Grid,
} from '@mui/material';
import RocketLaunchIcon from '@mui/icons-material/RocketLaunch';
import SchoolIcon from '@mui/icons-material/School';
import StarsIcon from '@mui/icons-material/Stars';

const Home = () => {
  const navigate = useNavigate();

  return (
    <Container maxWidth="lg">
      <Box
        sx={{
          mt: 8,
          mb: 6,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <Typography
          component="h1"
          variant="h2"
          align="center"
          color="primary"
          gutterBottom
          sx={{ fontWeight: 'bold' }}
        >
          Stellar Space Explorer
        </Typography>
        <Typography
          variant="h5"
          align="center"
          color="text.secondary"
          paragraph
          sx={{ mb: 4 }}
        >
          Embark on an epic journey through space while learning about Stellar blockchain development
        </Typography>
      </Box>

      <Grid container spacing={4}>
        <Grid item xs={12} md={4}>
          <Paper
            sx={{
              p: 3,
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              height: '100%',
              cursor: 'pointer',
              '&:hover': {
                transform: 'translateY(-4px)',
                transition: 'transform 0.2s',
              },
            }}
            onClick={() => navigate('/game')}
          >
            <RocketLaunchIcon sx={{ fontSize: 60, color: 'primary.main', mb: 2 }} />
            <Typography variant="h5" component="h2" gutterBottom>
              Start Exploring
            </Typography>
            <Typography variant="body1" align="center" color="text.secondary">
              Begin your space adventure and discover new star systems
            </Typography>
          </Paper>
        </Grid>

        <Grid item xs={12} md={4}>
          <Paper
            sx={{
              p: 3,
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              height: '100%',
              cursor: 'pointer',
              '&:hover': {
                transform: 'translateY(-4px)',
                transition: 'transform 0.2s',
              },
            }}
            onClick={() => navigate('/tutorial')}
          >
            <SchoolIcon sx={{ fontSize: 60, color: 'primary.main', mb: 2 }} />
            <Typography variant="h5" component="h2" gutterBottom>
              Learn Stellar
            </Typography>
            <Typography variant="body1" align="center" color="text.secondary">
              Interactive tutorials to master Stellar blockchain development
            </Typography>
          </Paper>
        </Grid>

        <Grid item xs={12} md={4}>
          <Paper
            sx={{
              p: 3,
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              height: '100%',
              cursor: 'pointer',
              '&:hover': {
                transform: 'translateY(-4px)',
                transition: 'transform 0.2s',
              },
            }}
          >
            <StarsIcon sx={{ fontSize: 60, color: 'primary.main', mb: 2 }} />
            <Typography variant="h5" component="h2" gutterBottom>
              Trade & Build
            </Typography>
            <Typography variant="body1" align="center" color="text.secondary">
              Trade resources and build powerful ships using Stellar assets
            </Typography>
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};

export default Home; 