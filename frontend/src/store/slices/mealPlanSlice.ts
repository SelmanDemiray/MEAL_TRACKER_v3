import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface Meal {
  id: string;
  name: string;
  calories: number;
  prepTime: number;
  protein?: number;
  carbs?: number;
  fat?: number;
}

interface MealPlanState {
  currentWeek: number;
  meals: { [key: string]: Meal };
  loading: boolean;
  error: string | null;
}

const initialState: MealPlanState = {
  currentWeek: 0,
  meals: {},
  loading: false,
  error: null,
};

const mealPlanSlice = createSlice({
  name: 'mealPlan',
  initialState,
  reducers: {
    setCurrentWeek: (state, action: PayloadAction<number>) => {
      state.currentWeek = action.payload;
    },
    addMeal: (state, action: PayloadAction<{ key: string; meal: Meal }>) => {
      state.meals[action.payload.key] = action.payload.meal;
    },
    removeMeal: (state, action: PayloadAction<string>) => {
      delete state.meals[action.payload];
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
  },
});

export const { setCurrentWeek, addMeal, removeMeal, setLoading, setError } = mealPlanSlice.actions;
export default mealPlanSlice.reducer;
