import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import Box from '@mui/material/Box';

// Import pages (we'll create these next)
import Home from './pages/Home';
import Game from './pages/Game';
import Tutorial from './pages/Tutorial';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#00B4D8',
    },
    secondary: {
      main: '#FFD700',
    },
    background: {
      default: '#0A1929',
      paper: '#001E3C',
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Router>
        <Box sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/game" element={<Game />} />
            <Route path="/tutorial" element={<Tutorial />} />
          </Routes>
        </Box>
      </Router>
    </ThemeProvider>
  );
}

export default App; 