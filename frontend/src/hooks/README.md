# 🎣 React Hooks - Custom Hooks Library

Reusable custom hooks for the Meal Prep Pro frontend application.

## 📋 Available Hooks

### 🔐 Authentication Hooks

#### `useAuth`
Comprehensive authentication management hook.

```typescript
import { useAuth } from './useAuth';

function MyComponent() {
  const { user, isAuthenticated, login, logout, loading } = useAuth();
  
  if (loading) return <Loading />;
  
  return (
    <div>
      {isAuthenticated ? (
        <div>
          <p>Welcome, {user?.username}!</p>
          <button onClick={logout}>Logout</button>
        </div>
      ) : (
        <button onClick={() => login('token')}>Login</button>
      )}
    </div>
  );
}
```

**Features:**
- JWT token management
- Automatic token refresh
- User session persistence
- Loading states
- Error handling

### 📊 Data Fetching Hooks

#### `useApi`
Generic API request hook with caching and error handling.

```typescript
import { useApi } from './useApi';

function RecipeList() {
  const { data: recipes, loading, error, refetch } = useApi('/api/recipes');
  
  if (loading) return <Loading />;
  if (error) return <Error message={error.message} />;
  
  return (
    <div>
      {recipes?.map(recipe => (
        <RecipeCard key={recipe.id} recipe={recipe} />
      ))}
      <button onClick={refetch}>Refresh</button>
    </div>
  );
}
```

#### `useQuery`
Advanced data fetching with React Query integration.

```typescript
import { useQuery } from './useQuery';

function NutritionDashboard() {
  const { 
    data, 
    isLoading, 
    error, 
    refetch,
    isStale 
  } = useQuery('nutrition-data', '/api/nutrition/daily');
  
  return (
    <div>
      {isStale && <RefreshIndicator />}
      <NutritionChart data={data} />
    </div>
  );
}
```

#### `useMutation`
Form submissions and data mutations.

```typescript
import { useMutation } from './useMutation';

function CreateRecipeForm() {
  const { mutate: createRecipe, loading, error } = useMutation(
    '/api/recipes',
    {
      onSuccess: (recipe) => {
        navigate(`/recipes/${recipe.id}`);
      },
      onError: (error) => {
        showNotification(error.message, 'error');
      }
    }
  );
  
  const handleSubmit = (formData) => {
    createRecipe(formData);
  };
  
  return (
    <form onSubmit={handleSubmit}>
      {/* form fields */}
      <button type="submit" disabled={loading}>
        {loading ? 'Creating...' : 'Create Recipe'}
      </button>
    </form>
  );
}
```

### 🎨 UI/UX Hooks

#### `useTheme`
Theme management and customization.

```typescript
import { useTheme } from './useTheme';

function ThemeToggle() {
  const { theme, toggleTheme, setTheme } = useTheme();
  
  return (
    <div>
      <button onClick={toggleTheme}>
        Switch to {theme === 'light' ? 'dark' : 'light'} mode
      </button>
    </div>
  );
}
```

#### `useLocalStorage`
Persistent local storage with serialization.

```typescript
import { useLocalStorage } from './useLocalStorage';

function UserPreferences() {
  const [preferences, setPreferences] = useLocalStorage('userPrefs', {
    notifications: true,
    autoSave: true
  });
  
  return (
    <div>
      <label>
        <input
          type="checkbox"
          checked={preferences.notifications}
          onChange={(e) => setPreferences({
            ...preferences,
            notifications: e.target.checked
          })}
        />
        Enable Notifications
      </label>
    </div>
  );
}
```

#### `useDebounce`
Debounced value updates for search and input optimization.

```typescript
import { useDebounce } from './useDebounce';

function SearchRecipes() {
  const [searchTerm, setSearchTerm] = useState('');
  const debouncedSearchTerm = useDebounce(searchTerm, 300);
  
  const { data: results } = useQuery(
    ['search', debouncedSearchTerm],
    `/api/recipes/search?q=${debouncedSearchTerm}`,
    { enabled: debouncedSearchTerm.length > 2 }
  );
  
  return (
    <div>
      <input
        value={searchTerm}
        onChange={(e) => setSearchTerm(e.target.value)}
        placeholder="Search recipes..."
      />
      <SearchResults results={results} />
    </div>
  );
}
```

### 🌐 Real-time Hooks

#### `useWebSocket`
WebSocket connection management.

```typescript
import { useWebSocket } from './useWebSocket';

function LiveNutritionUpdate() {
  const { sendMessage, lastMessage, connectionStatus } = useWebSocket('/ws/nutrition');
  
  useEffect(() => {
    if (lastMessage?.data) {
      const update = JSON.parse(lastMessage.data);
      updateNutritionData(update);
    }
  }, [lastMessage]);
  
  return (
    <div>
      <ConnectionIndicator status={connectionStatus} />
      <NutritionRealTimeChart />
    </div>
  );
}
```

#### `useNotifications`
Push notification management.

```typescript
import { useNotifications } from './useNotifications';

function NotificationCenter() {
  const { 
    notifications, 
    markAsRead, 
    markAllAsRead,
    subscribe,
    unsubscribe 
  } = useNotifications();
  
  return (
    <div>
      {notifications.map(notification => (
        <NotificationItem
          key={notification.id}
          notification={notification}
          onRead={() => markAsRead(notification.id)}
        />
      ))}
    </div>
  );
}
```

### 📱 Device & Responsive Hooks

#### `useMediaQuery`
Responsive design breakpoint detection.

```typescript
import { useMediaQuery } from './useMediaQuery';

function ResponsiveLayout() {
  const isMobile = useMediaQuery('(max-width: 768px)');
  const isTablet = useMediaQuery('(max-width: 1024px)');
  
  return (
    <div className={`layout ${isMobile ? 'mobile' : isTablet ? 'tablet' : 'desktop'}`}>
      {isMobile ? <MobileLayout /> : <DesktopLayout />}
    </div>
  );
}
```

#### `useGeolocation`
Location services for local grocery stores and restaurants.

```typescript
import { useGeolocation } from './useGeolocation';

function NearbyStores() {
  const { location, error, loading } = useGeolocation();
  
  const { data: stores } = useQuery(
    ['nearby-stores', location],
    `/api/stores/nearby?lat=${location?.latitude}&lng=${location?.longitude}`,
    { enabled: !!location }
  );
  
  return (
    <div>
      {loading && <p>Getting your location...</p>}
      {error && <p>Location access denied</p>}
      {stores && <StoreList stores={stores} />}
    </div>
  );
}
```

### 🍳 Meal Prep Specific Hooks

#### `useMealPlan`
Meal planning and management.

```typescript
import { useMealPlan } from './useMealPlan';

function MealPlanningCalendar() {
  const {
    mealPlan,
    addMeal,
    removeMeal,
    generatePlan,
    savePlan,
    loading
  } = useMealPlan();
  
  const handleDrop = (meal, date, mealType) => {
    addMeal(date, mealType, meal);
  };
  
  return (
    <Calendar
      mealPlan={mealPlan}
      onMealDrop={handleDrop}
      loading={loading}
    />
  );
}
```

#### `useNutritionTracking`
Nutrition goal tracking and analysis.

```typescript
import { useNutritionTracking } from './useNutritionTracking';

function NutritionDashboard() {
  const {
    dailyNutrition,
    goals,
    progress,
    addMeal,
    updateGoals,
    getInsights
  } = useNutritionTracking();
  
  return (
    <div>
      <NutritionProgressBar progress={progress} />
      <GoalsEditor goals={goals} onUpdate={updateGoals} />
      <InsightsPanel insights={getInsights()} />
    </div>
  );
}
```

#### `useRecipeAnalysis`
Recipe nutritional analysis and AI insights.

```typescript
import { useRecipeAnalysis } from './useRecipeAnalysis';

function RecipeEditor() {
  const {
    analysis,
    loading,
    analyzeRecipe,
    suggestions,
    optimizeRecipe
  } = useRecipeAnalysis();
  
  const handleAnalyze = (recipe) => {
    analyzeRecipe(recipe);
  };
  
  return (
    <div>
      <RecipeForm onAnalyze={handleAnalyze} />
      {loading && <AnalysisSpinner />}
      {analysis && (
        <div>
          <NutritionBreakdown analysis={analysis} />
          <OptimizationSuggestions 
            suggestions={suggestions}
            onOptimize={optimizeRecipe}
          />
        </div>
      )}
    </div>
  );
}
```

## 🛠️ Hook Utilities

### Error Handling
All hooks include comprehensive error handling:

```typescript
const { data, error, retry } = useApi('/api/endpoint');

if (error) {
  return <ErrorBoundary error={error} onRetry={retry} />;
}
```

### Loading States
Consistent loading state management:

```typescript
const { loading, data } = useQuery('key', '/api/endpoint');

return (
  <Suspense fallback={<LoadingSkeleton />}>
    {loading ? <LoadingSpinner /> : <DataComponent data={data} />}
  </Suspense>
);
```

### Caching Strategy
Intelligent caching with React Query:

```typescript
// Automatic background refetch
const { data } = useQuery('meals', '/api/meals', {
  staleTime: 5 * 60 * 1000, // 5 minutes
  cacheTime: 30 * 60 * 1000, // 30 minutes
  refetchOnWindowFocus: true
});
```

## 📝 Best Practices

### 1. Hook Composition
Combine hooks for complex functionality:

```typescript
function useMealPlanningWorkflow() {
  const { user } = useAuth();
  const { mealPlan, addMeal } = useMealPlan();
  const { analyzeNutrition } = useNutritionTracking();
  
  const addMealWithAnalysis = useCallback(async (meal, date) => {
    const nutritionData = await analyzeNutrition(meal);
    return addMeal(date, { ...meal, nutrition: nutritionData });
  }, [addMeal, analyzeNutrition]);
  
  return { addMealWithAnalysis };
}
```

### 2. Optimistic Updates
Improve UX with optimistic updates:

```typescript
const { mutate } = useMutation('/api/meals', {
  onMutate: async (newMeal) => {
    // Cancel outgoing refetches
    await queryClient.cancelQueries('meals');
    
    // Snapshot previous value
    const previousMeals = queryClient.getQueryData('meals');
    
    // Optimistically update
    queryClient.setQueryData('meals', old => [...old, newMeal]);
    
    return { previousMeals };
  },
  onError: (err, newMeal, context) => {
    // Rollback on error
    queryClient.setQueryData('meals', context.previousMeals);
  },
  onSettled: () => {
    // Refetch after mutation
    queryClient.invalidateQueries('meals');
  }
});
```

### 3. Type Safety
Ensure type safety with TypeScript:

```typescript
interface UseApiResult<T> {
  data: T | undefined;
  loading: boolean;
  error: Error | null;
  refetch: () => Promise<void>;
}

function useApi<T>(url: string): UseApiResult<T> {
  // Implementation
}
```

## 🧪 Testing

### Hook Testing with React Testing Library
```typescript
import { renderHook, act } from '@testing-library/react';
import { useAuth } from './useAuth';

test('should login user', async () => {
  const { result } = renderHook(() => useAuth());
  
  await act(async () => {
    result.current.login('test-token');
  });
  
  expect(result.current.isAuthenticated).toBe(true);
});
```

### Mock Hooks for Testing
```typescript
// __mocks__/useAuth.ts
export const useAuth = () => ({
  user: { id: '1', username: 'testuser' },
  isAuthenticated: true,
  login: jest.fn(),
  logout: jest.fn(),
  loading: false
});
```

---

For implementation details of specific hooks, see the individual hook files in this directory.
