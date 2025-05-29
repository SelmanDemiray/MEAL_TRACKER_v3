import React, { useState } from 'react';
import {
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  Button,
  LinearProgress,
  Chip,
  Tab,
  Tabs,
  Paper,
  IconButton,
  Stack,
  CircularProgress,
} from '@mui/material';
import {
  Add as AddIcon,
  TrendingUp,
  Restaurant,
  FitnessCenter,
  Water,
  Settings,
} from '@mui/icons-material';
import { motion } from 'framer-motion';

// Mock nutrition data
const mockNutritionData = {
  dailyGoals: {
    calories: { current: 1680, target: 2000 },
    protein: { current: 98, target: 150 },
    carbs: { current: 210, target: 250 },
    fat: { current: 52, target: 70 },
    fiber: { current: 18, target: 25 },
    water: { current: 6, target: 8 },
  },
  todaysMeals: [
    { 
      type: 'Breakfast', 
      name: 'Greek Yogurt with Berries', 
      time: '8:30 AM',
      calories: 280,
      protein: 20,
      carbs: 35,
      fat: 8
    },
    { 
      type: 'Lunch', 
      name: 'Chicken Quinoa Bowl', 
      time: '12:45 PM',
      calories: 520,
      protein: 35,
      carbs: 55,
      fat: 18
    },
    { 
      type: 'Snack', 
      name: 'Apple with Almond Butter', 
      time: '3:20 PM',
      calories: 190,
      protein: 6,
      carbs: 20,
      fat: 12
    },
  ],
  weeklyTrends: [
    { day: 'Mon', calories: 1950, protein: 145, carbs: 240 },
    { day: 'Tue', calories: 2100, protein: 155, carbs: 260 },
    { day: 'Wed', calories: 1850, protein: 140, carbs: 230 },
    { day: 'Thu', calories: 2050, protein: 150, carbs: 250 },
    { day: 'Fri', calories: 1980, protein: 148, carbs: 245 },
    { day: 'Sat', calories: 2200, protein: 160, carbs: 270 },
    { day: 'Sun', calories: 1680, protein: 98, carbs: 210 },
  ],
};

const NutritionTracking: React.FC = () => {
  const [selectedTab, setSelectedTab] = useState(0);

  const MacroProgressCard = ({ 
    label, 
    current, 
    target, 
    unit, 
    color 
  }: {
    label: string;
    current: number;
    target: number;
    unit: string;
    color: string;
  }) => {
    const percentage = Math.min((current / target) * 100, 100);
    
    return (
      <Card>
        <CardContent sx={{ textAlign: 'center' }}>
          <Box position="relative" display="inline-flex" mb={2}>
            <CircularProgress
              variant="determinate"
              value={percentage}
              size={80}
              thickness={6}
              sx={{ color }}
            />
            <Box
              position="absolute"
              top={0}
              left={0}
              bottom={0}
              right={0}
              display="flex"
              alignItems="center"
              justifyContent="center"
            >
              <Typography variant="h6" color="text.secondary">
                {Math.round(percentage)}%
              </Typography>
            </Box>
          </Box>
          
          <Typography variant="h6" color={color}>
            {current}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            of {target} {unit}
          </Typography>
          <Typography variant="caption" sx={{ textTransform: 'uppercase', fontWeight: 600 }}>
            {label}
          </Typography>
        </CardContent>
      </Card>
    );
  };

  const TodaysMealsCard = () => (
    <Card>
      <CardContent>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
          <Typography variant="h6">Today's Meals</Typography>
          <Button size="small" startIcon={<AddIcon />}>
            Log Meal
          </Button>
        </Box>
        
        <Stack spacing={2}>
          {mockNutritionData.todaysMeals.map((meal, index) => (
            <Paper key={index} variant="outlined" sx={{ p: 2 }}>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={1}>
                <Box>
                  <Typography variant="subtitle2">{meal.type}</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {meal.name} • {meal.time}
                  </Typography>
                </Box>
                <Chip label={`${meal.calories} cal`} size="small" color="primary" />
              </Box>
              
              <Grid container spacing={1}>
                <Grid item xs={4}>
                  <Typography variant="caption" color="text.secondary">
                    Protein: {meal.protein}g
                  </Typography>
                </Grid>
                <Grid item xs={4}>
                  <Typography variant="caption" color="text.secondary">
                    Carbs: {meal.carbs}g
                  </Typography>
                </Grid>
                <Grid item xs={4}>
                  <Typography variant="caption" color="text.secondary">
                    Fat: {meal.fat}g
                  </Typography>
                </Grid>
              </Grid>
            </Paper>
          ))}
        </Stack>
      </CardContent>
    </Card>
  );

  const WeeklyTrendsCard = () => (
    <Card>
      <CardContent>
        <Typography variant="h6" mb={2}>Weekly Trends</Typography>
        
        <Box sx={{ overflowX: 'auto' }}>
          <Box display="flex" gap={2} minWidth={400}>
            {mockNutritionData.weeklyTrends.map((day, index) => (
              <Box key={index} textAlign="center" minWidth={80}>
                <Typography variant="caption" color="text.secondary">
                  {day.day}
                </Typography>
                <Box mt={1}>
                  <Typography variant="body2" fontWeight={600}>
                    {day.calories}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    cal
                  </Typography>
                </Box>
                <Box mt={0.5}>
                  <Typography variant="caption" color="secondary.main">
                    {day.protein}g protein
                  </Typography>
                </Box>
              </Box>
            ))}
          </Box>
        </Box>
      </CardContent>
    </Card>
  );

  const QuickStats = () => (
    <Grid container spacing={2}>
      <Grid item xs={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <FitnessCenter color="primary" />
          <Typography variant="h6" mt={1}>5/7</Typography>
          <Typography variant="caption">Goals Hit This Week</Typography>
        </Paper>
      </Grid>
      <Grid item xs={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <TrendingUp color="success" />
          <Typography variant="h6" mt={1}>↑12%</Typography>
          <Typography variant="caption">Protein Improvement</Typography>
        </Paper>
      </Grid>
      <Grid item xs={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Restaurant color="secondary" />
          <Typography variant="h6" mt={1}>23</Typography>
          <Typography variant="caption">Meals Logged</Typography>
        </Paper>
      </Grid>
      <Grid item xs={6} md={3}>
        <Paper sx={{ p: 2, textAlign: 'center' }}>
          <Water color="info" />
          <Typography variant="h6" mt={1}>6.2L</Typography>
          <Typography variant="caption">Avg Daily Water</Typography>
        </Paper>
      </Grid>
    </Grid>
  );

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >
      <Box p={3}>
        {/* Header */}
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
          <Box>
            <Typography variant="h4">Nutrition Tracking</Typography>
            <Typography variant="body1" color="text.secondary">
              Monitor your daily nutrition and reach your health goals
            </Typography>
          </Box>
          
          <Box display="flex" gap={2}>
            <IconButton>
              <Settings />
            </IconButton>
            <Button
              variant="contained"
              startIcon={<AddIcon />}
              onClick={() => console.log('Quick log meal')}
            >
              Quick Log
            </Button>
          </Box>
        </Box>

        {/* Quick Stats */}
        <Box mb={4}>
          <QuickStats />
        </Box>

        {/* Macro Progress */}
        <Typography variant="h6" mb={2}>Today's Progress</Typography>
        <Grid container spacing={2} mb={4}>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Calories"
              current={mockNutritionData.dailyGoals.calories.current}
              target={mockNutritionData.dailyGoals.calories.target}
              unit="kcal"
              color="primary.main"
            />
          </Grid>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Protein"
              current={mockNutritionData.dailyGoals.protein.current}
              target={mockNutritionData.dailyGoals.protein.target}
              unit="g"
              color="secondary.main"
            />
          </Grid>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Carbs"
              current={mockNutritionData.dailyGoals.carbs.current}
              target={mockNutritionData.dailyGoals.carbs.target}
              unit="g"
              color="warning.main"
            />
          </Grid>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Fat"
              current={mockNutritionData.dailyGoals.fat.current}
              target={mockNutritionData.dailyGoals.fat.target}
              unit="g"
              color="success.main"
            />
          </Grid>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Fiber"
              current={mockNutritionData.dailyGoals.fiber.current}
              target={mockNutritionData.dailyGoals.fiber.target}
              unit="g"
              color="info.main"
            />
          </Grid>
          <Grid item xs={6} md={2}>
            <MacroProgressCard
              label="Water"
              current={mockNutritionData.dailyGoals.water.current}
              target={mockNutritionData.dailyGoals.water.target}
              unit="cups"
              color="cyan.500"
            />
          </Grid>
        </Grid>

        {/* Tabs */}
        <Tabs value={selectedTab} onChange={(_, newValue) => setSelectedTab(newValue)} sx={{ mb: 3 }}>
          <Tab label="Daily View" />
          <Tab label="Weekly Trends" />
          <Tab label="Goals & Settings" />
        </Tabs>

        {/* Content */}
        <Grid container spacing={3}>
          {selectedTab === 0 && (
            <>
              <Grid item xs={12} md={8}>
                <TodaysMealsCard />
              </Grid>
              <Grid item xs={12} md={4}>
                <WeeklyTrendsCard />
              </Grid>
            </>
          )}

          {selectedTab === 1 && (
            <Grid item xs={12}>
              <Card>
                <CardContent>
                  <Typography variant="h6" mb={2}>Weekly Nutrition Trends</Typography>
                  <Typography variant="body2" color="text.secondary">
                    Advanced charts and trend analysis coming soon...
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          )}

          {selectedTab === 2 && (
            <Grid item xs={12}>
              <Card>
                <CardContent>
                  <Typography variant="h6" mb={2}>Nutrition Goals & Settings</Typography>
                  <Typography variant="body2" color="text.secondary">
                    Goal management and preferences coming soon...
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          )}
        </Grid>
      </Box>
    </motion.div>
  );
};

export default NutritionTracking;
