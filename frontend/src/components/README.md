# 🧩 React Components Library

Reusable UI components for the Meal Prep Pro frontend application.

## 📁 Component Structure

```
components/
├── AI/                     # AI-powered components
├── ErrorBoundary/          # Error handling components
├── Navigation/             # Navigation and routing
├── Notifications/          # Real-time notifications
├── UI/                     # Generic UI components
├── Forms/                  # Form components
├── Charts/                 # Data visualization
├── Layout/                 # Layout components
└── Domain/                 # Domain-specific components
    ├── Recipe/             # Recipe-related components
    ├── Nutrition/          # Nutrition tracking
    ├── MealPlan/          # Meal planning
    └── Shopping/          # Shopping lists
```

## 🤖 AI Components

### AIFloatingAssistant
Floating AI assistant button with chat interface.

```tsx
import AIFloatingAssistant from 'components/AI/AIFloatingAssistant';

function App() {
  return (
    <div>
      {/* Your app content */}
      <AIFloatingAssistant />
    </div>
  );
}
```

**Features:**
- Floating action button
- Chat interface overlay
- AI-powered responses
- Context-aware suggestions
- Voice input support

### AIRecipeOptimizer
AI-powered recipe optimization suggestions.

```tsx
import AIRecipeOptimizer from 'components/AI/AIRecipeOptimizer';

function RecipeEditor({ recipe }) {
  return (
    <div>
      <RecipeForm recipe={recipe} />
      <AIRecipeOptimizer
        recipe={recipe}
        onOptimize={(optimizedRecipe) => {
          updateRecipe(optimizedRecipe);
        }}
      />
    </div>
  );
}
```

## 🚫 Error Handling

### ErrorBoundary
React error boundary for graceful error handling.

```tsx
import ErrorBoundary from 'components/ErrorBoundary/ErrorBoundary';

function App() {
  return (
    <ErrorBoundary>
      <Router>
        <Routes>
          {/* Your routes */}
        </Routes>
      </Router>
    </ErrorBoundary>
  );
}
```

**Features:**
- Catches JavaScript errors anywhere in component tree
- Logs error details
- Shows fallback UI
- Recovery options

## 🧭 Navigation Components

### NavigationBar
Main application navigation bar.

```tsx
import NavigationBar from 'components/Navigation/NavigationBar';

function AppLayout() {
  return (
    <div>
      <NavigationBar />
      <main>{/* Page content */}</main>
    </div>
  );
}
```

**Features:**
- Responsive design
- Active route highlighting
- User menu dropdown
- Mobile hamburger menu
- Search integration

### Breadcrumbs
Hierarchical navigation breadcrumbs.

```tsx
import Breadcrumbs from 'components/Navigation/Breadcrumbs';

function RecipeDetailPage() {
  const breadcrumbs = [
    { label: 'Home', href: '/' },
    { label: 'Recipes', href: '/recipes' },
    { label: 'Italian Pasta', href: '/recipes/123' }
  ];

  return (
    <div>
      <Breadcrumbs items={breadcrumbs} />
      {/* Page content */}
    </div>
  );
}
```

## 🔔 Notification Components

### NotificationCenter
Centralized notification management.

```tsx
import NotificationCenter from 'components/Notifications/NotificationCenter';

function App() {
  return (
    <div>
      {/* App content */}
      <NotificationCenter />
    </div>
  );
}
```

**Features:**
- Toast notifications
- Real-time updates
- Notification history
- Action buttons
- Dismissal controls

### NotificationToast
Individual notification toast component.

```tsx
import NotificationToast from 'components/Notifications/NotificationToast';

function CustomNotifications() {
  const [notifications, setNotifications] = useState([]);

  return (
    <div>
      {notifications.map(notification => (
        <NotificationToast
          key={notification.id}
          type={notification.type}
          message={notification.message}
          onClose={() => removeNotification(notification.id)}
        />
      ))}
    </div>
  );
}
```

## 🎨 UI Components

### LoadingScreen
Full-screen loading indicator.

```tsx
import LoadingScreen from 'components/UI/LoadingScreen';

function App() {
  if (isLoading) {
    return <LoadingScreen />;
  }

  return <MainApplication />;
}
```

### LoadingSkeleton
Content skeleton for loading states.

```tsx
import LoadingSkeleton from 'components/UI/LoadingSkeleton';

function RecipeCard({ recipe, loading }) {
  if (loading) {
    return <LoadingSkeleton variant="recipe-card" />;
  }

  return <RecipeCardContent recipe={recipe} />;
}
```

**Variants:**
- `recipe-card`
- `nutrition-chart`
- `meal-plan-grid`
- `shopping-list`
- `text-lines`

### Button
Enhanced button component with loading and variant support.

```tsx
import Button from 'components/UI/Button';

function RecipeActions() {
  return (
    <div>
      <Button
        variant="primary"
        loading={saving}
        onClick={saveRecipe}
        startIcon={<SaveIcon />}
      >
        Save Recipe
      </Button>
      
      <Button
        variant="outline"
        color="danger"
        onClick={deleteRecipe}
        confirmMessage="Are you sure you want to delete this recipe?"
      >
        Delete
      </Button>
    </div>
  );
}
```

### Modal
Flexible modal component with animations.

```tsx
import Modal from 'components/UI/Modal';

function RecipeEditModal({ open, recipe, onClose }) {
  return (
    <Modal
      open={open}
      onClose={onClose}
      title="Edit Recipe"
      size="large"
      showCloseButton
    >
      <RecipeForm
        recipe={recipe}
        onSave={(updatedRecipe) => {
          saveRecipe(updatedRecipe);
          onClose();
        }}
      />
    </Modal>
  );
}
```

## 📊 Chart Components

### NutritionChart
Interactive nutrition visualization.

```tsx
import NutritionChart from 'components/Charts/NutritionChart';

function NutritionDashboard({ nutritionData }) {
  return (
    <div>
      <NutritionChart
        data={nutritionData}
        type="donut"
        showGoals
        interactive
        onNutrientClick={(nutrient) => {
          showNutrientDetails(nutrient);
        }}
      />
    </div>
  );
}
```

**Chart Types:**
- `donut` - Macronutrient breakdown
- `bar` - Daily comparisons
- `line` - Trend analysis
- `radar` - Nutrient profile

### TrendChart
Time-series data visualization.

```tsx
import TrendChart from 'components/Charts/TrendChart';

function AnalyticsDashboard({ data }) {
  return (
    <TrendChart
      data={data}
      xAxis="date"
      yAxis="calories"
      showGoalLine
      goalValue={2000}
      timeRange="week"
      interactive
    />
  );
}
```

## 📝 Form Components

### FormField
Standardized form field wrapper.

```tsx
import FormField from 'components/Forms/FormField';

function ProfileForm() {
  return (
    <form>
      <FormField
        label="Full Name"
        name="fullName"
        required
        error={errors.fullName}
        helperText="Enter your full name as it appears on your ID"
      >
        <TextField />
      </FormField>
    </form>
  );
}
```

### RecipeForm
Comprehensive recipe creation/editing form.

```tsx
import RecipeForm from 'components/Forms/RecipeForm';

function CreateRecipePage() {
  return (
    <RecipeForm
      initialValues={emptyRecipe}
      onSubmit={createRecipe}
      onCancel={() => navigate('/recipes')}
      enableAIAssistance
      showNutritionAnalysis
    />
  );
}
```

### NutritionGoalsForm
Nutrition goal setting and editing.

```tsx
import NutritionGoalsForm from 'components/Forms/NutritionGoalsForm';

function GoalsSettingsPage() {
  return (
    <NutritionGoalsForm
      currentGoals={userGoals}
      onSave={updateGoals}
      showAIRecommendations
      includeAdvancedSettings
    />
  );
}
```

## 🏗️ Layout Components

### PageLayout
Standard page layout with sidebar and header.

```tsx
import PageLayout from 'components/Layout/PageLayout';

function RecipesPage() {
  return (
    <PageLayout
      title="My Recipes"
      subtitle="Manage your recipe collection"
      showBreadcrumbs
      actions={
        <Button onClick={() => setCreateModalOpen(true)}>
          Create Recipe
        </Button>
      }
    >
      <RecipeGrid recipes={recipes} />
    </PageLayout>
  );
}
```

### CardLayout
Reusable card layout for content sections.

```tsx
import CardLayout from 'components/Layout/CardLayout';

function NutritionSection() {
  return (
    <CardLayout
      title="Today's Nutrition"
      icon={<NutritionIcon />}
      actions={<RefreshButton />}
      loading={loading}
    >
      <NutritionSummary data={dailyNutrition} />
    </CardLayout>
  );
}
```

## 🍳 Domain-Specific Components

### Recipe Components

#### RecipeCard
Recipe preview card with actions.

```tsx
import RecipeCard from 'components/Domain/Recipe/RecipeCard';

function RecipeGrid({ recipes }) {
  return (
    <div className="recipe-grid">
      {recipes.map(recipe => (
        <RecipeCard
          key={recipe.id}
          recipe={recipe}
          onView={() => navigate(`/recipes/${recipe.id}`)}
          onEdit={() => setEditingRecipe(recipe)}
          onAddToMealPlan={(date) => addToMealPlan(recipe, date)}
          showNutritionInfo
          showActions
        />
      ))}
    </div>
  );
}
```

#### IngredientList
Interactive ingredient list with scaling.

```tsx
import IngredientList from 'components/Domain/Recipe/IngredientList';

function RecipeDetail({ recipe, servings }) {
  return (
    <div>
      <IngredientList
        ingredients={recipe.ingredients}
        servings={servings}
        originalServings={recipe.servings}
        allowScaling
        showSubstitutions
        onAddToShoppingList={addToShoppingList}
      />
    </div>
  );
}
```

### Nutrition Components

#### NutritionProgress
Goal progress visualization.

```tsx
import NutritionProgress from 'components/Domain/Nutrition/NutritionProgress';

function DashboardNutrition({ progress }) {
  return (
    <NutritionProgress
      goals={userGoals}
      current={dailyIntake}
      showDetails
      showRecommendations
      onGoalClick={(goal) => editGoal(goal)}
    />
  );
}
```

#### MacroBreakdown
Macronutrient breakdown visualization.

```tsx
import MacroBreakdown from 'components/Domain/Nutrition/MacroBreakdown';

function MealAnalysis({ meal }) {
  return (
    <MacroBreakdown
      nutrition={meal.nutrition}
      showPercentages
      showGrams
      interactive
      size="large"
    />
  );
}
```

### Meal Planning Components

#### MealPlanCalendar
Interactive meal planning calendar.

```tsx
import MealPlanCalendar from 'components/Domain/MealPlan/MealPlanCalendar';

function MealPlanningPage() {
  return (
    <MealPlanCalendar
      mealPlan={currentMealPlan}
      onMealDrop={handleMealDrop}
      onMealClick={viewMealDetail}
      showNutritionSummary
      allowDragDrop
      view="week"
    />
  );
}
```

#### MealSlot
Individual meal slot in calendar.

```tsx
import MealSlot from 'components/Domain/MealPlan/MealSlot';

function CalendarDay({ date, meals }) {
  return (
    <div className="calendar-day">
      {['breakfast', 'lunch', 'dinner'].map(mealType => (
        <MealSlot
          key={mealType}
          date={date}
          mealType={mealType}
          meal={meals[mealType]}
          onDrop={handleDrop}
          onRemove={removeMeal}
          showNutrition
        />
      ))}
    </div>
  );
}
```

## 🛒 Shopping Components

#### ShoppingListItem
Individual shopping list item with check-off.

```tsx
import ShoppingListItem from 'components/Domain/Shopping/ShoppingListItem';

function ShoppingList({ items }) {
  return (
    <div>
      {items.map(item => (
        <ShoppingListItem
          key={item.id}
          item={item}
          onCheck={markAsCompleted}
          onEdit={editItem}
          onDelete={deleteItem}
          showPrice
          showStore
        />
      ))}
    </div>
  );
}
```

## 🎨 Styling Guidelines

### Theme Integration
All components use the Material-UI theme system:

```tsx
import { useTheme } from '@mui/material/styles';

function CustomComponent() {
  const theme = useTheme();
  
  return (
    <Box
      sx={{
        backgroundColor: theme.palette.background.paper,
        color: theme.palette.text.primary,
        borderRadius: theme.shape.borderRadius,
      }}
    >
      Content
    </Box>
  );
}
```

### Responsive Design
Components use Material-UI breakpoints:

```tsx
function ResponsiveComponent() {
  return (
    <Box
      sx={{
        display: { xs: 'block', md: 'flex' },
        gap: { xs: 1, md: 2 },
        padding: { xs: 1, sm: 2, md: 3 }
      }}
    >
      Content
    </Box>
  );
}
```

## 🧪 Testing

### Component Testing
```tsx
import { render, screen, fireEvent } from '@testing-library/react';
import RecipeCard from './RecipeCard';

test('renders recipe card with title', () => {
  const mockRecipe = {
    id: '1',
    name: 'Test Recipe',
    description: 'A test recipe'
  };

  render(<RecipeCard recipe={mockRecipe} />);
  
  expect(screen.getByText('Test Recipe')).toBeInTheDocument();
});

test('calls onView when card is clicked', () => {
  const onView = jest.fn();
  const mockRecipe = { id: '1', name: 'Test Recipe' };

  render(<RecipeCard recipe={mockRecipe} onView={onView} />);
  
  fireEvent.click(screen.getByText('Test Recipe'));
  expect(onView).toHaveBeenCalledWith();
});
```

### Storybook Stories
```tsx
// RecipeCard.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import RecipeCard from './RecipeCard';

const meta: Meta<typeof RecipeCard> = {
  title: 'Domain/Recipe/RecipeCard',
  component: RecipeCard,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    recipe: {
      id: '1',
      name: 'Spaghetti Carbonara',
      description: 'Classic Italian pasta dish',
      prepTime: 15,
      cookTime: 20,
      servings: 4,
      difficulty: 'medium'
    }
  }
};

export const WithNutrition: Story = {
  args: {
    ...Default.args,
    showNutritionInfo: true
  }
};
```

---

For detailed component APIs and examples, see the individual component files and their corresponding Storybook stories.
