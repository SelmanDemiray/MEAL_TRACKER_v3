import React from 'react';
import { Container, Typography, Paper, Button, Box } from '@mui/material';
import { useAuth } from '../hooks/useAuth';

const Login: React.FC = () => {
  const { login } = useAuth();

  const handleLogin = () => {
    // Mock login
    login('mock-token');
  };

  return (
    <Container maxWidth="sm" sx={{ mt: 8 }}>
      <Paper sx={{ p: 4 }}>
        <Typography variant="h4" align="center" gutterBottom>
          Login
        </Typography>
        <Box display="flex" justifyContent="center" mt={2}>
          <Button variant="contained" onClick={handleLogin}>
            Mock Login
          </Button>
        </Box>
      </Paper>
    </Container>
  );
};

export default Login;
