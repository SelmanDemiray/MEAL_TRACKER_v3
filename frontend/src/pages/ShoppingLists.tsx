import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const ShoppingLists: React.FC = () => {
  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" gutterBottom>
        Shopping Lists
      </Typography>
      <Paper sx={{ p: 2 }}>
        <Typography>Manage your shopping lists here</Typography>
      </Paper>
    </Container>
  );
};

export default ShoppingLists;
