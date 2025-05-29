import React, { useState } from 'react';
import {
  Box,
  Container,
  Typography,
  Paper,
  Grid,
  Card,
  CardContent,
  Button,
  Chip,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Tabs,
  Tab,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  CalendarToday as CalendarIcon,
  Restaurant as RestaurantIcon,
  Schedule as ScheduleIcon,
} from '@mui/icons-material';
import { motion } from 'framer-motion';

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
  Monday: DayMeals;
  Tuesday: DayMeals;
  Wednesday: DayMeals;
  Thursday: DayMeals;
  Friday: DayMeals;
  Saturday: DayMeals;
  Sunday: DayMeals;
}

type MealType = 'Breakfast' | 'Lunch' | 'Dinner';
type DayOfWeek = 'Monday' | 'Tuesday' | 'Wednesday' | 'Thursday' | 'Friday' | 'Saturday' | 'Sunday';

interface MealCardProps {
  day: string;
  mealType: MealType;
  meal?: Meal;
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

const MealCard: React.FC<MealCardProps> = ({ day, mealType, meal }) => {
  const [open, setOpen] = useState(false);

  return (
    <>
      <Card 
        sx={{ 
          height: '100%', 
          cursor: 'pointer',
          transition: 'all 0.2s ease-in-out',
          '&:hover': {
            transform: 'translateY(-2px)',
            boxShadow: 4
          }
        }}
        onClick={() => setOpen(true)}
      >
        <CardContent>
          {meal ? (
            <>
              <Typography variant="h6" component="h3" gutterBottom>
                {meal.name}
              </Typography>
              <Box display="flex" gap={1} flexWrap="wrap" mb={1}>
                <Chip 
                  icon={<RestaurantIcon />} 
                  label={`${meal.calories} cal`} 
                  size="small" 
                  color="primary"
                />
                <Chip 
                  icon={<ScheduleIcon />} 
                  label={`${meal.prepTime} min`} 
                  size="small" 
                  color="secondary"
                />
              </Box>
            </>
          ) : (
            <Box 
              display="flex" 
              flexDirection="column" 
              alignItems="center" 
              justifyContent="center"
              minHeight={100}
              color="text.secondary"
            >
              <AddIcon sx={{ fontSize: 48, mb: 1 }} />
              <Typography variant="body2">
                Add {mealType}
              </Typography>
            </Box>
          )}
        </CardContent>
      </Card>

      <Dialog open={open} onClose={() => setOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>
          {meal ? `Edit ${mealType}` : `Add ${mealType}`} for {day}
        </DialogTitle>
        <DialogContent>
          <Box component="form" sx={{ mt: 2 }}>
            <TextField
              fullWidth
              label="Meal Name"
              defaultValue={meal?.name || ''}
              margin="normal"
            />
            <Grid container spacing={2} sx={{ mt: 1 }}>
              <Grid item xs={4}>
                <TextField
                  fullWidth
                  label="Calories"
                  type="number"
                  defaultValue={meal?.calories || ''}
                />
              </Grid>
              <Grid item xs={4}>
                <TextField
                  fullWidth
                  label="Prep Time (min)"
                  type="number"
                  defaultValue={meal?.prepTime || ''}
                />
              </Grid>
              <Grid item xs={4}>
                <FormControl fullWidth>
                  <InputLabel>Difficulty</InputLabel>
                  <Select defaultValue="Medium">
                    <MenuItem value="Easy">Easy</MenuItem>
                    <MenuItem value="Medium">Medium</MenuItem>
                    <MenuItem value="Hard">Hard</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button onClick={() => setOpen(false)} variant="contained">
            Save
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

const MealPlanning: React.FC = () => {
  const [selectedTab, setSelectedTab] = useState(0);
  const [selectedWeek, setSelectedWeek] = useState(0);

  const daysOfWeek: DayOfWeek[] = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
  const mealTypes: MealType[] = ['Breakfast', 'Lunch', 'Dinner'];

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.1
      }
    }
  };

  const itemVariants = {
    hidden: { y: 20, opacity: 0 },
    visible: {
      y: 0,
      opacity: 1
    }
  };

  return (
    <Container maxWidth="xl" sx={{ py: 4 }}>
      <motion.div
        variants={containerVariants}
        initial="hidden"
        animate="visible"
      >
        {/* Header */}
        <motion.div variants={itemVariants}>
          <Box display="flex" justifyContent="space-between" alignItems="center" mb={4}>
            <Box>
              <Typography variant="h3" component="h1" gutterBottom>
                🗓️ Meal Planning
              </Typography>
              <Typography variant="h6" color="text.secondary">
                Plan your weekly meals and optimize your nutrition
              </Typography>
            </Box>
            <Box display="flex" gap={2}>
              <Button
                variant="outlined"
                startIcon={<CalendarIcon />}
                onClick={() => setSelectedWeek(selectedWeek - 1)}
              >
                Previous Week
              </Button>
              <Button
                variant="outlined"
                startIcon={<CalendarIcon />}
                onClick={() => setSelectedWeek(selectedWeek + 1)}
              >
                Next Week
              </Button>
              <Button
                variant="contained"
                startIcon={<AddIcon />}
              >
                Auto-Generate Plan
              </Button>
            </Box>
          </Box>
        </motion.div>

        {/* Tabs */}
        <motion.div variants={itemVariants}>
          <Paper sx={{ mb: 3 }}>
            <Tabs 
              value={selectedTab} 
              onChange={(_, newValue) => setSelectedTab(newValue)}
              variant="fullWidth"
            >
              <Tab label="Weekly View" />
              <Tab label="Calendar View" />
              <Tab label="Shopping List" />
            </Tabs>
          </Paper>
        </motion.div>

        {/* Weekly Meal Plan Grid */}
        {selectedTab === 0 && (
          <motion.div variants={itemVariants}>
            <Paper sx={{ p: 3 }}>
              <Grid container spacing={2}>
                {/* Header Row */}
                <Grid item xs={12} md={1.5}>
                  <Typography variant="h6" textAlign="center">
                    Meal Type
                  </Typography>
                </Grid>
                {daysOfWeek.map((day) => (
                  <Grid item xs={12} md={1.5} key={day}>
                    <Typography variant="h6" textAlign="center">
                      {day}
                    </Typography>
                  </Grid>
                ))}

                {/* Meal Rows */}
                {mealTypes.map((mealType) => (
                  <React.Fragment key={mealType}>
                    <Grid item xs={12} md={1.5}>
                      <Box 
                        display="flex" 
                        alignItems="center" 
                        justifyContent="center"
                        height="100%"
                        sx={{ 
                          backgroundColor: 'primary.main',
                          color: 'primary.contrastText',
                          borderRadius: 1,
                          py: 2
                        }}
                      >
                        <Typography variant="subtitle1" fontWeight="bold">
                          {mealType}
                        </Typography>
                      </Box>
                    </Grid>
                    {daysOfWeek.map((day) => (
                      <Grid item xs={12} md={1.5} key={`${day}-${mealType}`}>
                        <Box sx={{ minHeight: 120 }}>
                          <MealCard
                            day={day}
                            mealType={mealType}
                            meal={mockMealPlan[day]?.[mealType]}
                          />
                        </Box>
                      </Grid>
                    ))}
                  </React.Fragment>
                ))}
              </Grid>
            </Paper>
          </motion.div>
        )}

        {/* Calendar View Placeholder */}
        {selectedTab === 1 && (
          <motion.div variants={itemVariants}>
            <Paper sx={{ p: 4, textAlign: 'center', minHeight: 400 }}>
              <CalendarIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
              <Typography variant="h5" gutterBottom>
                Calendar View
              </Typography>
              <Typography color="text.secondary">
                Monthly calendar view coming soon...
              </Typography>
            </Paper>
          </motion.div>
        )}

        {/* Shopping List Placeholder */}
        {selectedTab === 2 && (
          <motion.div variants={itemVariants}>
            <Paper sx={{ p: 4, textAlign: 'center', minHeight: 400 }}>
              <RestaurantIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
              <Typography variant="h5" gutterBottom>
                Auto-Generated Shopping List
              </Typography>
              <Typography color="text.secondary">
                Shopping list generation based on meal plan coming soon...
              </Typography>
            </Paper>
          </motion.div>
        )}

        {/* Weekly Nutrition Summary */}
        <motion.div variants={itemVariants}>
          <Paper sx={{ p: 3, mt: 3 }}>
            <Typography variant="h5" gutterBottom>
              📊 Weekly Nutrition Summary
            </Typography>
            <Grid container spacing={3}>
              <Grid item xs={12} sm={6} md={3}>
                <Box textAlign="center">
                  <Typography variant="h4" color="primary.main">
                    2,850
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Avg Daily Calories
                  </Typography>
                </Box>
              </Grid>
              <Grid item xs={12} sm={6} md={3}>
                <Box textAlign="center">
                  <Typography variant="h4" color="secondary.main">
                    165g
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Avg Daily Protein
                  </Typography>
                </Box>
              </Grid>
              <Grid item xs={12} sm={6} md={3}>
                <Box textAlign="center">
                  <Typography variant="h4" color="success.main">
                    2.5h
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Total Prep Time
                  </Typography>
                </Box>
              </Grid>
              <Grid item xs={12} sm={6} md={3}>
                <Box textAlign="center">
                  <Typography variant="h4" color="warning.main">
                    $12.50
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Avg Cost per Day
                  </Typography>
                </Box>
              </Grid>
            </Grid>
          </Paper>
        </motion.div>
      </motion.div>
    </Container>
  );
};

export default MealPlanning;
