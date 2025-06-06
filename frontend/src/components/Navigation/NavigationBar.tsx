import React from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  Button,
  Box,
  IconButton,
  Avatar,
} from '@mui/material';
import {
  Restaurant,
  CalendarMonth,
  Analytics,
  ShoppingCart,
  Person,
} from '@mui/icons-material';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAuth } from '../../hooks/useAuth';

const NavigationBar: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { user, logout } = useAuth();

  const navigationItems = [
    { label: 'Dashboard', path: '/dashboard', icon: <Analytics /> },
    { label: 'Meal Planning', path: '/meal-planning', icon: <CalendarMonth /> },
    { label: 'Recipes', path: '/recipes', icon: <Restaurant /> },
    { label: 'Nutrition', path: '/nutrition', icon: <Analytics /> },
    { label: 'Shopping', path: '/shopping', icon: <ShoppingCart /> },
  ];

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <AppBar position="fixed" elevation={1} sx={{ bgcolor: 'background.paper', color: 'text.primary' }}>
      <Toolbar>
        <Typography variant="h6" component="div" sx={{ flexGrow: 1, color: 'primary.main', fontWeight: 'bold' }}>
          🍽️ Meal Prep Pro
        </Typography>
        
        <Box display="flex" gap={1}>
          {navigationItems.map((item) => (
            <Button
              key={item.path}
              onClick={() => navigate(item.path)}
              startIcon={item.icon}
              variant={location.pathname === item.path ? 'contained' : 'text'}
              size="small"
            >
              {item.label}
            </Button>
          ))}
        </Box>

        <Box ml={2}>
          <IconButton onClick={() => navigate('/profile')}>
            <Avatar sx={{ width: 32, height: 32 }}>
              <Person />
            </Avatar>
          </IconButton>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default NavigationBar;
