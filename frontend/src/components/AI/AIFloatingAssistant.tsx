import React from 'react';
import { Fab } from '@mui/material';
import { Psychology } from '@mui/icons-material';
import { useAIAssistant } from '../../contexts/AIAssistantContext';

const AIFloatingAssistant: React.FC = () => {
  const { showAssistant } = useAIAssistant();

  return (
    <Fab
      color="secondary"
      aria-label="AI Assistant"
      sx={{
        position: 'fixed',
        bottom: 24,
        right: 24,
        zIndex: 1000,
      }}
      onClick={showAssistant}
    >
      <Psychology />
    </Fab>
  );
};

export default AIFloatingAssistant;
