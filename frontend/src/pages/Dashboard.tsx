import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  LinearProgress,
  Avatar,
  Stack,
  IconButton,
  Paper,
} from '@mui/material';
import {
  CalendarMonth,
  Restaurant,
  Analytics,
  ShoppingCart,
  TrendingUp,
  Notifications,
  Add as AddIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';

// Mock data - replace with actual API calls
const mockData = {
  nutritionProgress: {
    calories: { current: 1450, target: 2000, percentage: 72.5 },
    protein: { current: 85, target: 120, percentage: 70.8 },
    carbs: { current: 180, target: 250, percentage: 72 },
    fat: { current: 48, target: 65, percentage: 73.8 },
  },
  todaysMeals: [
    { type: 'Breakfast', name: 'Overnight Oats with Berries', calories: 320, status: 'completed' },
    { type: 'Lunch', name: 'Grilled Chicken Salad', calories: 480, status: 'completed' },
    { type: 'Dinner', name: 'Salmon with Quinoa', calories: 650, status: 'planned' },
  ],
  weeklyStats: {
    mealsPrepped: 12,
    recipesCreated: 3,
    nutritionGoalsHit: 5,
    moneySaved: 47.50,
  },
  recentActivities: [
    { type: 'meal_logged', description: 'Logged lunch: Grilled Chicken Salad', time: '2 hours ago' },
    { type: 'recipe_created', description: 'Created new recipe: Protein Smoothie Bowl', time: '1 day ago' },
    { type: 'goal_achieved', description: 'Hit daily protein target!', time: '1 day ago' },
  ],
};

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate loading
    const timer = setTimeout(() => setLoading(false), 1000);
    return () => clearTimeout(timer);
  }, []);

  const QuickActionCard = ({ 
    title, 
    description, 
    icon, 
    color, 
    onClick 
  }: {
    title: string;
    description: string;
    icon: React.ReactNode;
    color: string;
    onClick: () => void;
  }) => (
    <Card 
      sx={{ 
        cursor: 'pointer',
        transition: 'transform 0.2s, box-shadow 0.2s',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow: 3,
        }
      }}
      onClick={onClick}
    >
      <CardContent>
        <Box display="flex" alignItems="center" gap={2}>
          <Avatar sx={{ bgcolor: color, width: 48, height: 48 }}>
            {icon}
          </Avatar>
          <Box>
            <Typography variant="h6">{title}</Typography>
            <Typography variant="body2" color="text.secondary">
              {description}
            </Typography>
          </Box>
        </Box>
      </CardContent>
    </Card>
  );

  const NutritionProgressCard = () => (
    <Card>
      <CardContent>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
          <Typography variant="h6">Today's Nutrition</Typography>
          <IconButton size="small">
            <RefreshIcon />
          </IconButton>
        </Box>
        
        <Stack spacing={3}>
          {Object.entries(mockData.nutritionProgress).map(([key, value]) => (
            <Box key={key}>
              <Box display="flex" justifyContent="space-between" mb={1}>
                <Typography variant="body2" sx={{ textTransform: 'capitalize' }}>
                  {key}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                  {value.current}/{value.target} {key === 'calories' ? 'kcal' : 'g'}
                </Typography>
              </Box>
              <LinearProgress 
                variant="determinate" 
                value={Math.min(value.percentage, 100)}
                sx={{
                  height: 8,
                  borderRadius: 4,
                  backgroundColor: 'grey.200',
                  '& .MuiLinearProgress-bar': {
                    borderRadius: 4,
                    backgroundColor: value.percentage >= 100 ? 'success.main' : 'primary.main',
                  }
                }}
              />
            </Box>
          ))}
        </Stack>
      </CardContent>
    </Card>
  );

  const TodaysMealsCard = () => (
    <Card>
      <CardContent>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
          <Typography variant="h6">Today's Meals</Typography>
          <Button 
            size="small" 
            startIcon={<AddIcon />}
            onClick={() => navigate('/meal-planning')}
          >
            Add Meal
          </Button>
        </Box>
        
        <Stack spacing={2}>
          {mockData.todaysMeals.map((meal, index) => (
            <Paper 
              key={index} 
              variant="outlined" 
              sx={{ p: 2, backgroundColor: meal.status === 'completed' ? 'success.50' : 'grey.50' }}
            >
              <Box display="flex" justifyContent="space-between" alignItems="center">
                <Box>
                  <Typography variant="subtitle2">{meal.type}</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {meal.name}
                  </Typography>
                </Box>
                <Box textAlign="right">
                  <Typography variant="body2">{meal.calories} cal</Typography>
                  <Chip 
                    size="small" 
                    label={meal.status}
                    color={meal.status === 'completed' ? 'success' : 'default'}
                    variant={meal.status === 'completed' ? 'filled' : 'outlined'}
                  />
                </Box>
              </Box>
            </Paper>
          ))}
        </Stack>
      </CardContent>
    </Card>
  );

  const WeeklyStatsCard = () => (
    <Card>
      <CardContent>
        <Typography variant="h6" mb={2}>This Week</Typography>
        
        <Grid container spacing={2}>
          <Grid item xs={6}>
            <Box textAlign="center">
              <Typography variant="h4" color="primary.main">
                {mockData.weeklyStats.mealsPrepped}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Meals Prepped
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={6}>
            <Box textAlign="center">
              <Typography variant="h4" color="secondary.main">
                {mockData.weeklyStats.recipesCreated}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Recipes Created
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={6}>
            <Box textAlign="center">
              <Typography variant="h4" color="success.main">
                {mockData.weeklyStats.nutritionGoalsHit}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Goals Hit
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={6}>
            <Box textAlign="center">
              <Typography variant="h4" color="warning.main">
                ${mockData.weeklyStats.moneySaved}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Money Saved
              </Typography>
            </Box>
          </Grid>
        </Grid>
      </CardContent>
    </Card>
  );

  if (loading) {
    return (
      <Box p={3}>
        <Typography variant="h4" mb={3}>Dashboard</Typography>
        <Grid container spacing={3}>
          {[...Array(6)].map((_, i) => (
            <Grid item xs={12} md={6} lg={4} key={i}>
              <Card>
                <CardContent>
                  <Box height={120} bgcolor="grey.100" borderRadius={1} />
                </CardContent>
              </Card>
            </Grid>
          ))}
        </Grid>
      </Box>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <Box p={3}>
        <Typography variant="h4" mb={1}>
          Good morning! 👋
        </Typography>
        <Typography variant="body1" color="text.secondary" mb={4}>
          Here's your meal prep overview for today
        </Typography>

        <Grid container spacing={3}>
          {/* Quick Actions */}
          <Grid item xs={12} md={6} lg={3}>
            <QuickActionCard
              title="Plan Meals"
              description="Create your weekly meal plan"
              icon={<CalendarMonth />}
              color="primary.main"
              onClick={() => navigate('/meal-planning')}
            />
          </Grid>
          
          <Grid item xs={12} md={6} lg={3}>
            <QuickActionCard
              title="Browse Recipes"
              description="Discover new healthy recipes"
              icon={<Restaurant />}
              color="secondary.main"
              onClick={() => navigate('/recipes')}
            />
          </Grid>
          
          <Grid item xs={12} md={6} lg={3}>
            <QuickActionCard
              title="Track Nutrition"
              description="Log your meals and nutrients"
              icon={<Analytics />}
              color="success.main"
              onClick={() => navigate('/nutrition')}
            />
          </Grid>
          
          <Grid item xs={12} md={6} lg={3}>
            <QuickActionCard
              title="Shopping List"
              description="Generate your grocery list"
              icon={<ShoppingCart />}
              color="warning.main"
              onClick={() => navigate('/shopping')}
            />
          </Grid>

          {/* Nutrition Progress */}
          <Grid item xs={12} md={6}>
            <NutritionProgressCard />
          </Grid>

          {/* Today's Meals */}
          <Grid item xs={12} md={6}>
            <TodaysMealsCard />
          </Grid>

          {/* Weekly Stats */}
          <Grid item xs={12} md={6}>
            <WeeklyStatsCard />
          </Grid>

          {/* Recent Activities */}
          <Grid item xs={12} md={6}>
            <Card>
              <CardContent>
                <Typography variant="h6" mb={2}>Recent Activity</Typography>
                <Stack spacing={2}>
                  {mockData.recentActivities.map((activity, index) => (
                    <Box key={index} display="flex" alignItems="center" gap={2}>
                      <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.50' }}>
                        <TrendingUp fontSize="small" color="primary" />
                      </Avatar>
                      <Box flex={1}>
                        <Typography variant="body2">{activity.description}</Typography>
                        <Typography variant="caption" color="text.secondary">
                          {activity.time}
                        </Typography>
                      </Box>
                    </Box>
                  ))}
                </Stack>
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </Box>
    </motion.div>
  );
};

export default Dashboard;
