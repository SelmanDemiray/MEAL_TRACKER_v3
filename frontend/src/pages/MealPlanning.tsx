import React, { useState } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  IconButton,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  AccessTime,
  LocalFireDepartment,
} from '@mui/icons-material';

// Type definitions
interface Meal {
  name: string;
  calories: number;
  prepTime: number;
  protein?: number;
  carbs?: number;
  fat?: number;
}

interface DayMeals {
  Breakfast: Meal;
  Lunch: Meal;
  Dinner: Meal;
}

interface MealPlan {
  [day: string]: DayMeals;
}

type MealType = 'Breakfast' | 'Lunch' | 'Dinner';
type DayOfWeek = 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday';

interface MealCardProps {
  day: DayOfWeek;
  mealType: MealType;
  meal: Meal;
  onEdit: (day: DayOfWeek, mealType: MealType, meal: Meal) => void;
  onDelete: (day: DayOfWeek, mealType: MealType) => void;
}

const mockMealPlan: MealPlan = {
  Monday: {
    Breakfast: { name: 'Overnight Oats with Berries', calories: 320, prepTime: 5 },
    Lunch: { name: 'Grilled Chicken Salad', calories: 450, prepTime: 15 },
    Dinner: { name: 'Salmon with Quinoa', calories: 580, prepTime: 25 }
  },
  Tuesday: {
    Breakfast: { name: 'Greek Yogurt Parfait', calories: 280, prepTime: 5 },
    Lunch: { name: 'Turkey Wrap', calories: 420, prepTime: 10 },
    Dinner: { name: 'Beef Stir-fry', calories: 520, prepTime: 20 }
  },
  Wednesday: {
    Breakfast: { name: 'Protein Smoothie', calories: 300, prepTime: 5 },
    Lunch: { name: 'Buddha Bowl', calories: 480, prepTime: 15 },
    Dinner: { name: 'Chicken Curry', calories: 550, prepTime: 30 }
  },
  Thursday: {
    Breakfast: { name: 'Avocado Toast', calories: 350, prepTime: 8 },
    Lunch: { name: 'Quinoa Salad', calories: 400, prepTime: 12 },
    Dinner: { name: 'Grilled Fish', calories: 480, prepTime: 20 }
  },
  Friday: {
    Breakfast: { name: 'Egg Bowl', calories: 380, prepTime: 10 },
    Lunch: { name: 'Chicken Wrap', calories: 440, prepTime: 10 },
    Dinner: { name: 'Pasta Primavera', calories: 520, prepTime: 25 }
  },
  Saturday: {
    Breakfast: { name: 'Pancakes', calories: 420, prepTime: 15 },
    Lunch: { name: 'Sandwich', calories: 380, prepTime: 8 },
    Dinner: { name: 'Pizza Night', calories: 650, prepTime: 30 }
  },
  Sunday: {
    Breakfast: { name: 'French Toast', calories: 390, prepTime: 12 },
    Lunch: { name: 'Soup & Salad', calories: 320, prepTime: 15 },
    Dinner: { name: 'Roast Chicken', calories: 580, prepTime: 40 }
  }
};

const MealCard: React.FC<MealCardProps> = ({ day, mealType, meal, onEdit, onDelete }) => {
  return (
    <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <CardContent sx={{ flexGrow: 1 }}>
        <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={1}>
          <Typography variant="h6" component="h3" fontSize="1rem">
            {meal.name}
          </Typography>
          <Box>
            <IconButton size="small" onClick={() => onEdit(day, mealType, meal)}>
              <EditIcon fontSize="small" />
            </IconButton>
            <IconButton size="small" onClick={() => onDelete(day, mealType)}>
              <DeleteIcon fontSize="small" />
            </IconButton>
          </Box>
        </Box>
        
        <Box display="flex" gap={1} mb={2}>
          <Chip
            icon={<LocalFireDepartment />}
            label={`${meal.calories} cal`}
            size="small"
            color="primary"
            variant="outlined"
          />
          <Chip
            icon={<AccessTime />}
            label={`${meal.prepTime} min`}
            size="small"
            color="secondary"
            variant="outlined"
          />
        </Box>
      </CardContent>
    </Card>
  );
};

const MealPlanning: React.FC = () => {
  const [mealPlan, setMealPlan] = useState<MealPlan>(mockMealPlan);
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [editingMeal, setEditingMeal] = useState<{
    day: DayOfWeek;
    mealType: MealType;
    meal: Meal;
  } | null>(null);
  
  const days: DayOfWeek[] = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
  const mealTypes: MealType[] = ['Breakfast', 'Lunch', 'Dinner'];

  const handleEditMeal = (day: DayOfWeek, mealType: MealType, meal: Meal) => {
    setEditingMeal({ day, mealType, meal });
    setEditDialogOpen(true);
  };

  const handleDeleteMeal = (day: DayOfWeek, mealType: MealType) => {
    setMealPlan(prev => ({
      ...prev,
      [day]: {
        ...prev[day],
        [mealType]: { name: '', calories: 0, prepTime: 0 }
      }
    }));
  };

  const handleSaveMeal = () => {
    if (editingMeal) {
      setMealPlan(prev => ({
        ...prev,
        [editingMeal.day]: {
          ...prev[editingMeal.day],
          [editingMeal.mealType]: editingMeal.meal
        }
      }));
    }
    setEditDialogOpen(false);
    setEditingMeal(null);
  };

  return (
    <Box p={3}>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          Weekly Meal Plan
        </Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => {/* Add new meal plan logic */}}
        >
          Generate New Plan
        </Button>
      </Box>

      <Grid container spacing={3}>
        {days.map((day) => (
          <Grid item xs={12} md={6} lg={4} xl={3} key={day}>
            <Paper sx={{ p: 2 }}>
              <Typography variant="h6" gutterBottom color="primary">
                {day}
              </Typography>
              
              <Box display="flex" flexDirection="column" gap={2}>
                {mealTypes.map((mealType) => (
                  <Box key={mealType}>
                    <Typography variant="subtitle2" color="text.secondary" mb={1}>
                      {mealType}
                    </Typography>
                    {mealPlan[day]?.[mealType]?.name ? (
                      <MealCard
                        day={day}
                        mealType={mealType}
                        meal={mealPlan[day][mealType]}
                        onEdit={handleEditMeal}
                        onDelete={handleDeleteMeal}
                      />
                    ) : (
                      <Card sx={{ minHeight: 120, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                        <Button
                          startIcon={<AddIcon />}
                          onClick={() => handleEditMeal(day, mealType, { name: '', calories: 0, prepTime: 0 })}
                        >
                          Add {mealType}
                        </Button>
                      </Card>
                    )}
                  </Box>
                ))}
              </Box>
            </Paper>
          </Grid>
        ))}
      </Grid>

      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>
          {editingMeal?.meal.name ? 'Edit Meal' : 'Add New Meal'}
        </DialogTitle>
        <DialogContent>
          <TextField
            fullWidth
            label="Meal Name"
            value={editingMeal?.meal.name || ''}
            onChange={(e) => setEditingMeal(prev => prev ? {
              ...prev,
              meal: { ...prev.meal, name: e.target.value }
            } : null)}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Calories"
            type="number"
            value={editingMeal?.meal.calories || ''}
            onChange={(e) => setEditingMeal(prev => prev ? {
              ...prev,
              meal: { ...prev.meal, calories: parseInt(e.target.value) || 0 }
            } : null)}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Prep Time (minutes)"
            type="number"
            value={editingMeal?.meal.prepTime || ''}
            onChange={(e) => setEditingMeal(prev => prev ? {
              ...prev,
              meal: { ...prev.meal, prepTime: parseInt(e.target.value) || 0 }
            } : null)}
            margin="normal"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSaveMeal} variant="contained">Save</Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default MealPlanning;
