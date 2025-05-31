import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface Meal {
  id: string;
  name: string;
  calories: number;
  prepTime: number;
  difficulty?: string;
  cuisine?: string;
}

interface DayMeals {
  Breakfast?: Meal;
  Lunch?: Meal;
  Dinner?: Meal;
}

interface MealPlan {
  [day: string]: DayMeals;
}

interface MealPlanState {
  currentWeek: number;
  mealPlan: MealPlan;
  loading: boolean;
  error: string | null;
  nutritionGoals: {
    calories: number;
    protein: number;
    carbs: number;
    fat: number;
  };
}

const initialState: MealPlanState = {
  currentWeek: 0,
  mealPlan: {},
  loading: false,
  error: null,
  nutritionGoals: {
    calories: 2000,
    protein: 150,
    carbs: 250,
    fat: 70,
  },
};

const mealPlanSlice = createSlice({
  name: 'mealPlan',
  initialState,
  reducers: {
    setCurrentWeek: (state, action: PayloadAction<number>) => {
      state.currentWeek = action.payload;
    },
    setMealPlan: (state, action: PayloadAction<MealPlan>) => {
      state.mealPlan = action.payload;
    },
    addMeal: (state, action: PayloadAction<{ day: string; mealType: string; meal: Meal }>) => {
      const { day, mealType, meal } = action.payload;
      if (!state.mealPlan[day]) {
        state.mealPlan[day] = {};
      }
      // Ensure the meal object has the expected structure
      const mealData = {
        name: meal.name || 'Unnamed Meal',
        calories: meal.calories || 0,
        prepTime: meal.prepTime || 0,
        difficulty: meal.difficulty || 'Medium',
        cuisine: meal.cuisine || 'Various',
        ...meal
      };
      state.mealPlan[day][mealType as keyof DayMeals] = mealData;
    },
    removeMeal: (state, action: PayloadAction<{ day: string; mealType: string }>) => {
      const { day, mealType } = action.payload;
      if (state.mealPlan[day]) {
        delete state.mealPlan[day][mealType as keyof DayMeals];
      }
    },
    updateNutritionGoals: (state, action: PayloadAction<Partial<MealPlanState['nutritionGoals']>>) => {
      state.nutritionGoals = { ...state.nutritionGoals, ...action.payload };
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
  },
});

export const {
  setCurrentWeek,
  setMealPlan,
  addMeal,
  removeMeal,
  updateNutritionGoals,
  setLoading,
  setError,
} = mealPlanSlice.actions;

export default mealPlanSlice.reducer;
