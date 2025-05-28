import React from 'react';
import { AppBar, Toolbar, Typography, Button } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';

const NavigationBar: React.FC = () => {
  const navigate = useNavigate();
  const { logout } = useAuth();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <AppBar position="fixed">
      <Toolbar>
        <Typography variant="h6" sx={{ flexGrow: 1 }}>
          Meal Prep Pro
        </Typography>
        <Button color="inherit" onClick={() => navigate('/dashboard')}>
          Dashboard
        </Button>
        <Button color="inherit" onClick={() => navigate('/meal-planning')}>
          Meal Planning
        </Button>
        <Button color="inherit" onClick={() => navigate('/nutrition')}>
          Nutrition
        </Button>
        <Button color="inherit" onClick={() => navigate('/recipes')}>
          Recipes
        </Button>
        <Button color="inherit" onClick={handleLogout}>
          Logout
        </Button>
      </Toolbar>
    </AppBar>
  );
};

export default NavigationBar;
