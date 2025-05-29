import React, { useState } from 'react';
import {
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  Button,
  Chip,
  IconButton,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Avatar,
  Stack,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  ShoppingCart,
  CalendarToday,
  Restaurant,
  Timer,
} from '@mui/icons-material';
import { motion } from 'framer-motion';

// Define proper types for meal plan
interface Meal {
  name: string;
  calories: number;
  prepTime: number;
  difficulty?: string;
  cuisine?: string;
}

interface DayMeals {
  Breakfast: Meal;
  Lunch: Meal;
  Dinner: Meal;
}

interface MealPlan {
  [key: string]: DayMeals;
}

// Updated mock data with proper typing
const mockMealPlan: MealPlan = {
  Monday: {
    Breakfast: { name: 'Greek Yogurt Parfait', calories: 320, prepTime: 10, difficulty: 'Easy', cuisine: 'Mediterranean' },
    Lunch: { name: 'Quinoa Buddha Bowl', calories: 450, prepTime: 25, difficulty: 'Medium', cuisine: 'Healthy' },
    Dinner: { name: 'Grilled Salmon & Veggies', calories: 520, prepTime: 35, difficulty: 'Medium', cuisine: 'American' },
  },
  Tuesday: {
    Breakfast: { name: 'Avocado Toast', calories: 280, prepTime: 8, difficulty: 'Easy', cuisine: 'Modern' },
    Lunch: { name: 'Thai Chicken Salad', calories: 380, prepTime: 20, difficulty: 'Easy', cuisine: 'Thai' },
    Dinner: { name: 'Beef Stir Fry', calories: 480, prepTime: 30, difficulty: 'Medium', cuisine: 'Asian' },
  },
  Wednesday: {
    Breakfast: { name: 'Protein Smoothie', calories: 350, prepTime: 5, difficulty: 'Easy', cuisine: 'Healthy' },
    Lunch: { name: 'Mediterranean Wrap', calories: 420, prepTime: 15, difficulty: 'Easy', cuisine: 'Mediterranean' },
    Dinner: { name: 'Chicken Tikka Masala', calories: 550, prepTime: 45, difficulty: 'Hard', cuisine: 'Indian' },
  },
  Thursday: {
    Breakfast: { name: 'Overnight Oats', calories: 300, prepTime: 5, difficulty: 'Easy', cuisine: 'Healthy' },
    Lunch: { name: 'Caesar Salad', calories: 360, prepTime: 15, difficulty: 'Easy', cuisine: 'American' },
    Dinner: { name: 'Pasta Primavera', calories: 480, prepTime: 25, difficulty: 'Medium', cuisine: 'Italian' },
  },
  Friday: {
    Breakfast: { name: 'Scrambled Eggs & Toast', calories: 340, prepTime: 12, difficulty: 'Easy', cuisine: 'American' },
    Lunch: { name: 'Sushi Bowl', calories: 400, prepTime: 20, difficulty: 'Medium', cuisine: 'Japanese' },
    Dinner: { name: 'BBQ Pulled Pork', calories: 580, prepTime: 40, difficulty: 'Medium', cuisine: 'American' },
  },
  Saturday: {
    Breakfast: { name: 'Pancakes & Berries', calories: 420, prepTime: 20, difficulty: 'Medium', cuisine: 'American' },
    Lunch: { name: 'Fish Tacos', calories: 440, prepTime: 25, difficulty: 'Medium', cuisine: 'Mexican' },
    Dinner: { name: 'Ribeye Steak Dinner', calories: 650, prepTime: 35, difficulty: 'Hard', cuisine: 'American' },
  },
  Sunday: {
    Breakfast: { name: 'French Toast', calories: 380, prepTime: 18, difficulty: 'Medium', cuisine: 'French' },
    Lunch: { name: 'Chicken Pho', calories: 390, prepTime: 30, difficulty: 'Medium', cuisine: 'Vietnamese' },
    Dinner: { name: 'Vegetable Curry', calories: 450, prepTime: 35, difficulty: 'Medium', cuisine: 'Indian' },
  },
};

const MealPlan: React.FC = () => {
  const [selectedWeek, setSelectedWeek] = useState(0);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [selectedMeal, setSelectedMeal] = useState<{day: string, mealType: keyof DayMeals} | null>(null);

  const daysOfWeek = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'] as const;
  const mealTypes: (keyof DayMeals)[] = ['Breakfast', 'Lunch', 'Dinner'];

  const MealCard: React.FC<{
    day: string;
    mealType: keyof DayMeals;
    meal: Meal;
  }> = ({ day, mealType, meal }) => (
    <Card 
      sx={{ 
        height: '100%',
        transition: 'transform 0.2s, box-shadow 0.2s',
        '&:hover': {
          transform: 'translateY(-2px)',
          boxShadow: 3,
        }
      }}
    >
      <CardContent sx={{ p: 2 }}>
        <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={1}>
          <Chip 
            label={mealType} 
            size="small" 
            color={
              mealType === 'Breakfast' ? 'primary' : 
              mealType === 'Lunch' ? 'secondary' : 'success'
            } 
          />
          <IconButton 
            size="small" 
            onClick={() => {
              setSelectedMeal({ day, mealType });
              setEditDialogOpen(true);
            }}
          >
            <EditIcon fontSize="small" />
          </IconButton>
        </Box>
        
        <Typography variant="h6" sx={{ fontSize: '1rem', fontWeight: 600, mb: 1 }}>
          {meal.name}
        </Typography>
        
        <Stack direction="row" spacing={1} mb={2}>
          <Chip 
            icon={<Restaurant />} 
            label={`${meal.calories} cal`} 
            size="small" 
            variant="outlined" 
          />
          <Chip 
            icon={<Timer />} 
            label={`${meal.prepTime}m`} 
            size="small" 
            variant="outlined" 
          />
        </Stack>
        
        <Box display="flex" justifyContent="space-between" alignItems="center">
          <Typography variant="caption" color="text.secondary">
            {meal.difficulty} • {meal.cuisine}
          </Typography>
          
          <Button size="small" variant="text">
            View Recipe
          </Button>
        </Box>
      </CardContent>
    </Card>
  );

  const WeekSummary = () => {
    const totalCalories = daysOfWeek.reduce((sum, day) => {
      const dayMeals = mockMealPlan[day];
      return sum + dayMeals.Breakfast.calories + dayMeals.Lunch.calories + dayMeals.Dinner.calories;
    }, 0);
    
    const avgCaloriesPerDay = Math.round(totalCalories / 7);
    const totalPrepTime = daysOfWeek.reduce((sum, day) => {
      const dayMeals = mockMealPlan[day];
      return sum + dayMeals.Breakfast.prepTime + dayMeals.Lunch.prepTime + dayMeals.Dinner.prepTime;
    }, 0);

    return (
      <Paper sx={{ p: 3, mb: 3 }}>
        <Typography variant="h6" mb={2}>This Week's Summary</Typography>
        <Grid container spacing={3}>
          <Grid item xs={12} sm={3}>
            <Box textAlign="center">
              <Typography variant="h4" color="primary.main">
                {avgCaloriesPerDay}
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Avg Calories/Day
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={12} sm={3}>
            <Box textAlign="center">
              <Typography variant="h4" color="secondary.main">
                {Math.round(totalPrepTime / 60)}h
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Total Prep Time
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={12} sm={3}>
            <Box textAlign="center">
              <Typography variant="h4" color="success.main">
                21
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Meals Planned
              </Typography>
            </Box>
          </Grid>
          <Grid item xs={12} sm={3}>
            <Box textAlign="center">
              <Typography variant="h4" color="warning.main">
                $85
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Est. Weekly Cost
              </Typography>
            </Box>
          </Grid>
        </Grid>
      </Paper>
    );
  };

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
            <Typography variant="h4">Meal Plan</Typography>
            <Typography variant="body1" color="text.secondary">
              Plan your weekly meals and stay on track with your nutrition goals
            </Typography>
          </Box>
          
          <Box display="flex" gap={2}>
            <Button
              variant="outlined"
              startIcon={<ShoppingCart />}
              onClick={() => console.log('Generate shopping list')}
            >
              Shopping List
            </Button>
            <Button
              variant="contained"
              startIcon={<AddIcon />}
              onClick={() => console.log('Generate AI plan')}
            >
              AI Generate
            </Button>
          </Box>
        </Box>

        {/* Week Summary */}
        <WeekSummary />

        {/* Week Navigation */}
        <Box display="flex" justifyContent="center" mb={3}>
          <Paper sx={{ p: 1, display: 'flex', alignItems: 'center', gap: 2 }}>
            <IconButton onClick={() => setSelectedWeek(prev => prev - 1)}>
              <CalendarToday />
            </IconButton>
            <Typography variant="h6">
              Week of Dec {9 + selectedWeek * 7}, 2024
            </Typography>
            <IconButton onClick={() => setSelectedWeek(prev => prev + 1)}>
              <CalendarToday />
            </IconButton>
          </Paper>
        </Box>

        {/* Meal Plan Grid */}
        <Grid container spacing={2}>
          {daysOfWeek.map((day) => (
            <Grid item xs={12} key={day}>
              <Paper sx={{ p: 2 }}>
                <Typography variant="h6" mb={2} color="primary.main">
                  {day}
                </Typography>
                <Grid container spacing={2}>
                  {mealTypes.map((mealType) => (
                    <Grid item xs={12} md={4} key={mealType}>
                      <Box sx={{ height: '100%' }}>
                        <MealCard
                          day={day}
                          mealType={mealType}
                          meal={mockMealPlan[day][mealType]}
                        />
                      </Box>
                    </Grid>
                  ))}
                </Grid>
              </Paper>
            </Grid>
          ))}
        </Grid>

        {/* Edit Meal Dialog */}
        <Dialog 
          open={editDialogOpen} 
          onClose={() => setEditDialogOpen(false)}
          maxWidth="sm"
          fullWidth
        >
          <DialogTitle>
            Edit {selectedMeal?.mealType} for {selectedMeal?.day}
          </DialogTitle>
          <DialogContent>
            <Box sx={{ pt: 1 }}>
              <TextField
                fullWidth
                label="Meal Name"
                defaultValue={selectedMeal ? mockMealPlan[selectedMeal.day][selectedMeal.mealType].name : ''}
                sx={{ mb: 2 }}
              />
              <Grid container spacing={2} sx={{ mb: 2 }}>
                <Grid item xs={6}>
                  <TextField
                    fullWidth
                    label="Calories"
                    type="number"
                    defaultValue={selectedMeal ? mockMealPlan[selectedMeal.day][selectedMeal.mealType].calories : ''}
                  />
                </Grid>
                <Grid item xs={6}>
                  <TextField
                    fullWidth
                    label="Prep Time (min)"
                    type="number"
                    defaultValue={selectedMeal ? mockMealPlan[selectedMeal.day][selectedMeal.mealType].prepTime : ''}
                  />
                </Grid>
              </Grid>
              <Grid container spacing={2}>
                <Grid item xs={6}>
                  <FormControl fullWidth>
                    <InputLabel>Difficulty</InputLabel>
                    <Select
                      defaultValue={selectedMeal ? mockMealPlan[selectedMeal.day][selectedMeal.mealType].difficulty : 'Easy'}
                      label="Difficulty"
                    >
                      <MenuItem value="Easy">Easy</MenuItem>
                      <MenuItem value="Medium">Medium</MenuItem>
                      <MenuItem value="Hard">Hard</MenuItem>
                    </Select>
                  </FormControl>
                </Grid>
                <Grid item xs={6}>
                  <TextField
                    fullWidth
                    label="Cuisine"
                    defaultValue={selectedMeal ? mockMealPlan[selectedMeal.day][selectedMeal.mealType].cuisine : ''}
                  />
                </Grid>
              </Grid>
            </Box>
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setEditDialogOpen(false)}>
              Cancel
            </Button>
            <Button 
              variant="contained" 
              onClick={() => {
                setEditDialogOpen(false);
                console.log('Save meal changes');
              }}
            >
              Save Changes
            </Button>
          </DialogActions>
        </Dialog>
      </Box>
    </motion.div>
  );
};

export default MealPlan;
