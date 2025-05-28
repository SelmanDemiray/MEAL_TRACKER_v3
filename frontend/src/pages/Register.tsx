import React from 'react';
import { Container, Typography, Paper } from '@mui/material';

const Register: React.FC = () => {
  return (
    <Container maxWidth="sm" sx={{ mt: 8 }}>
      <Paper sx={{ p: 4 }}>
        <Typography variant="h4" align="center" gutterBottom>
          Register
        </Typography>
        <Typography align="center">
          Registration form coming soon
        </Typography>
      </Paper>
    </Container>
  );
};

export default Register;
