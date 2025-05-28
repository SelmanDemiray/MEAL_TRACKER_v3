import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const MealPlanning: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Meal Planning
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Plan your meals here</Typography>
      </Paper>
    </Container>
  );
};

export default MealPlanning;
