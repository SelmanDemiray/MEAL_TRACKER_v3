import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface MealPlan {
  id: string;
  name: string;
  startDate: string;
  endDate: string;
  meals: Record<string, any>;
}

interface MealPlanState {
  currentPlan: MealPlan | null;
  plans: MealPlan[];
  loading: boolean;
  error: string | null;
}

const initialState: MealPlanState = {
  currentPlan: null,
  plans: [],
  loading: false,
  error: null,
};

const mealPlanSlice = createSlice({
  name: 'mealPlan',
  initialState,
  reducers: {
    setCurrentPlan: (state, action: PayloadAction<MealPlan>) => {
      state.currentPlan = action.payload;
    },
    setPlans: (state, action: PayloadAction<MealPlan[]>) => {
      state.plans = action.payload;
    },
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    setError: (state, action: PayloadAction<string | null>) => {
      state.error = action.payload;
    },
  },
});

export const { setCurrentPlan, setPlans, setLoading, setError } = mealPlanSlice.actions;
export default mealPlanSlice.reducer;
