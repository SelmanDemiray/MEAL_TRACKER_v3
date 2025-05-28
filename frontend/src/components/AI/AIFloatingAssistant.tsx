import React from 'react';
import { Fab } from '@mui/material';
import { SmartToy } from '@mui/icons-material';

const AIFloatingAssistant: React.FC = () => {
  return (
    <Fab
      color="primary"
      sx={{
        position: 'fixed',
        bottom: 16,
        right: 16,
      }}
    >
      <SmartToy />
    </Fab>
  );
};

export default AIFloatingAssistant;
