import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const Dashboard: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Dashboard
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Welcome to Meal Prep Pro Dashboard</Typography>
      </Paper>
    </Container>
  );
};

export default Dashboard;
