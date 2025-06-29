import React, { useState } from 'react';
import { Container, Typography, Paper, Button, Box, TextField, Alert } from '@mui/material';
import { useAuth } from '../hooks/useAuth';
import { useNavigate } from 'react-router-dom';
import { apiClient } from '../hooks/useApi';

const Login: React.FC = () => {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      const res = await apiClient.post('/auth/login', { email, password });
      login(res.data.token, res.data.user);
      navigate('/dashboard');
    } catch (err: any) {
      setError(err?.response?.data?.message || 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  const handleDemoLogin = async () => {
    setError(null);
    setLoading(true);
    try {
      const res = await apiClient.post('/auth/login', {
        email: 'demo@mealprep.com',
        password: 'demopass'
      });
      login(res.data.token, res.data.user);
      navigate('/dashboard');
    } catch {
      // fallback: just set a demo token and user
      login('demo-token', {
        id: 'demo',
        username: 'Demo User',
        email: 'demo@mealprep.com'
      });
      navigate('/dashboard');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container maxWidth="sm" sx={{ mt: 8 }}>
      <Paper sx={{ p: 4 }}>
        <Typography variant="h4" align="center" gutterBottom>
          Login
        </Typography>
        <Box component="form" onSubmit={handleLogin} mt={2}>
          <TextField
            label="Email"
            type="email"
            fullWidth
            margin="normal"
            required
            value={email}
            onChange={e => setEmail(e.target.value)}
            autoComplete="email"
          />
          <TextField
            label="Password"
            type="password"
            fullWidth
            margin="normal"
            required
            value={password}
            onChange={e => setPassword(e.target.value)}
            autoComplete="current-password"
          />
          {error && <Alert severity="error" sx={{ mt: 2 }}>{error}</Alert>}
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            sx={{ mt: 2 }}
            disabled={loading}
          >
            {loading ? 'Logging in...' : 'Login'}
          </Button>
          <Button
            variant="outlined"
            color="secondary"
            fullWidth
            sx={{ mt: 2 }}
            onClick={handleDemoLogin}
            disabled={loading}
          >
            {loading ? 'Loading Demo...' : 'Try Free Demo'}
          </Button>
          <Button
            fullWidth
            sx={{ mt: 2 }}
            onClick={() => navigate('/register')}
          >
            Don't have an account? Register
          </Button>
        </Box>
      </Paper>
    </Container>
  );
};

export default Login;
