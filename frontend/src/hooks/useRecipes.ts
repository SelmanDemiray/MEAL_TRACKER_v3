import { useState, useEffect } from 'react';
import { useQuery, useMutation, useQueryClient } from 'react-query';
import axios from 'axios';

interface Recipe {
  id: string;
  name: string;
  description?: string;
  prep_time_minutes?: number;
  cook_time_minutes?: number;
  total_time_minutes?: number;
  servings?: number;
  tags?: string[];
  source_repository?: string;
  ingredients?: string[];
  directions?: string[];
  similarity_score?: number;
}

interface ImportRequest {
  repository_url: string;
  user_id?: string;
}

interface ImportResponse {
  batch_id: string;
  message: string;
}

interface ImportStatus {
  id: string;
  repository_url: string;
  import_status: string;
  total_recipes?: number;
  successful_imports?: number;
  failed_imports?: number;
  started_at?: string;
  completed_at?: string;
}

interface SearchParams {
  q?: string;
  tags?: string;
  limit?: number;
  page?: number;
}

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080';

// API functions
const recipeApi = {
  search: async (params: SearchParams): Promise<Recipe[]> => {
    const searchParams = new URLSearchParams();
    if (params.q) searchParams.append('q', params.q);
    if (params.tags) searchParams.append('tags', params.tags);
    if (params.limit) searchParams.append('limit', params.limit.toString());
    if (params.page) searchParams.append('page', params.page.toString());

    const response = await axios.get(`${API_BASE_URL}/api/recipes/search?${searchParams}`);
    return response.data;
  },

  getDetail: async (id: string): Promise<Recipe> => {
    const response = await axios.get(`http://localhost:38083/api/recipes/${id}`);
    return response.data;
  },

  importRecipes: async (request: ImportRequest) => {
    const response = await axios.post(`${API_BASE_URL}/api/recipes/import`, request);
    return response.data;
  },

  getImportStatus: async (batchId: string): Promise<ImportStatus> => {
    const response = await axios.get(`http://localhost:38083/api/recipes/import/${batchId}/status`);
    return response.data;
  },
};

// Custom hooks
export const useRecipeSearch = (params: SearchParams) => {
  return useQuery(
    ['recipes', params],
    () => recipeApi.search(params),
    {
      staleTime: 5 * 60 * 1000, // 5 minutes
      enabled: Boolean(params.q && params.q.length > 2) || Boolean(params.tags) || (!params.q && !params.tags),
    }
  );
};

export const useRecipe = (id: string) => {
  return useQuery(
    ['recipes', id],
    () => recipeApi.getDetail(id),
    {
      enabled: Boolean(id),
      staleTime: 30 * 60 * 1000, // 30 minutes
    }
  );
};

export const useRecipeImport = () => {
  const queryClient = useQueryClient();

  return useMutation(recipeApi.importRecipes, {
    onSuccess: () => {
      // Invalidate recipe searches after import
      queryClient.invalidateQueries(['recipes', 'search']);
    },
  });
};

export const useImportStatus = (batchId?: string) => {
  return useQuery(
    ['import', 'status', batchId],
    () => recipeApi.getImportStatus(batchId!),
    {
      enabled: Boolean(batchId),
      refetchInterval: (data) => {
        // Stop polling if import is complete
        if (data?.import_status === 'completed' || data?.import_status === 'failed') {
          return false;
        }
        return 2000; // Poll every 2 seconds while in progress
      },
    }
  );
};

// Helper hook for managing recipe favorites
export const useRecipeFavorites = () => {
  const [favorites, setFavorites] = useState<Set<string>>(
    new Set(JSON.parse(localStorage.getItem('recipe-favorites') || '[]'))
  );

  useEffect(() => {
    localStorage.setItem('recipe-favorites', JSON.stringify(Array.from(favorites)));
  }, [favorites]);

  const toggleFavorite = (recipeId: string) => {
    // For development, also update local state
    setFavorites(prev => {
      const newFavorites = new Set(prev);
      if (newFavorites.has(recipeId)) {
        newFavorites.delete(recipeId);
      } else {
        newFavorites.add(recipeId);
      }
      return newFavorites;
    });

    // TODO: Replace with real API call
    console.log(`Toggled favorite for recipe ID: ${recipeId}`);
  };

  const isFavorite = (recipeId: string) => favorites.has(recipeId);

  return { favorites, toggleFavorite, isFavorite };
};
