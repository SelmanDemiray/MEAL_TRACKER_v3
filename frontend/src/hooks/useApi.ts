import { useState, useEffect, useCallback } from 'react';
import axios, { AxiosError } from 'axios';

interface UseApiResult<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
}

interface UseApiOptions {
  immediate?: boolean;
  onSuccess?: (data: any) => void;
  onError?: (error: string) => void;
}

const API_BASE_URL = process.env.REACT_APP_API_URL || '/api';

// Create axios instance with interceptors
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
});

// Request interceptor to add auth token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('authToken');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor for error handling
apiClient.interceptors.response.use(
  (response) => response,
  (error: AxiosError) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('authToken');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export function useApi<T>(
  url: string,
  options: UseApiOptions = {}
): UseApiResult<T> {
  const { immediate = true, onSuccess, onError } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(immediate);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const response = await apiClient.get<T>(url);
      setData(response.data);
      
      if (onSuccess) {
        onSuccess(response.data);
      }
    } catch (err) {
      const errorMessage = err instanceof AxiosError 
        ? err.response?.data?.message || err.message 
        : 'An unexpected error occurred';
      
      setError(errorMessage);
      
      if (onError) {
        onError(errorMessage);
      }
    } finally {
      setLoading(false);
    }
  }, [url, onSuccess, onError]);

  useEffect(() => {
    if (immediate) {
      fetchData();
    }
  }, [fetchData, immediate]);

  return {
    data,
    loading,
    error,
    refetch: fetchData,
  };
}

export function useMutation<TData, TVariables = any>() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const mutate = useCallback(async (
    url: string,
    data?: TVariables,
    method: 'POST' | 'PUT' | 'PATCH' | 'DELETE' = 'POST'
  ): Promise<TData> => {
    try {
      setLoading(true);
      setError(null);

      const response = await apiClient.request<TData>({
        url,
        method,
        data,
      });

      return response.data;
    } catch (err) {
      const errorMessage = err instanceof AxiosError 
        ? err.response?.data?.message || err.message 
        : 'An unexpected error occurred';
      
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    mutate,
    loading,
    error,
  };
}

// Specific API hooks for common operations
export function useNutritionAnalysis() {
  const { mutate, loading, error } = useMutation();

  const analyzeMeal = useCallback(async (ingredients: any[], portionSize: number) => {
    return mutate('/nutrition/analyze/meal', {
      ingredients,
      portion_size: portionSize,
      cooking_method: null,
      user_id: localStorage.getItem('userId'),
    });
  }, [mutate]);

  const analyzeDailyNutrition = useCallback(async (meals: any[]) => {
    return mutate('/nutrition/analyze/daily', {
      user_id: localStorage.getItem('userId'),
      date: new Date().toISOString(),
      meals,
    });
  }, [mutate]);

  return {
    analyzeMeal,
    analyzeDailyNutrition,
    loading,
    error,
  };
}

export function useMealRecommendations() {
  const { mutate, loading, error } = useMutation();

  const getRecommendations = useCallback(async (preferences: any) => {
    return mutate('/nutrition/recommendations/meals', preferences);
  }, [mutate]);

  return {
    getRecommendations,
    loading,
    error,
  };
}

export function useNutritionGoals() {
  const { mutate, loading, error } = useMutation();

  const calculateGoals = useCallback(async (userInfo: any) => {
    return mutate('/nutrition/goals/calculate', userInfo);
  }, [mutate]);

  const trackProgress = useCallback(async (currentIntake: any, goals: any) => {
    return mutate('/nutrition/goals/track', {
      user_id: localStorage.getItem('userId'),
      current_intake: currentIntake,
      goals,
    });
  }, [mutate]);

  return {
    calculateGoals,
    trackProgress,
    loading,
    error,
  };
}

export { apiClient };
