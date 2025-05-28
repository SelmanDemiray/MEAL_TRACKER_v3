import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const NutritionTracking: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Nutrition Tracking
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Track your nutrition here</Typography>
      </Paper>
    </Container>
  );
};

export default NutritionTracking;
