import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const RecipeManager: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Recipe Manager
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Manage your recipes here</Typography>
      </Paper>
    </Container>
  );
};

export default RecipeManager;
