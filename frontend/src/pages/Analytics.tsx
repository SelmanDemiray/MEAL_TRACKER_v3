import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const Analytics: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Analytics
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>View your analytics here</Typography>
      </Paper>
    </Container>
  );
};

export default Analytics;
