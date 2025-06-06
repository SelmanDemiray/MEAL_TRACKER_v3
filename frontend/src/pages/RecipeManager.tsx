import React, { useState, useEffect } from 'react';
import {
  Container,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  TextField,
  InputAdornment,
  Menu,
  MenuItem,
  Tabs,
  Tab,
  Paper,
  Box,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Rating,
  Stack,
  Alert,
  CircularProgress,
  Fab,
} from '@mui/material';
import {
  Search as SearchIcon,
  FilterList as FilterIcon,
  Add as AddIcon,
  Favorite as FavoriteIcon,
  FavoriteBorder as FavoriteBorderIcon,
  AccessTime as TimeIcon,
  Restaurant as RestaurantIcon,
} from '@mui/icons-material';
import { motion, AnimatePresence } from 'framer-motion';
import { useRecipeSearch, useRecipeImport, useImportStatus, useRecipeFavorites } from '../hooks/useRecipes';

// Mock recipe data
const mockRecipes = [
  {
    id: '1',
    name: 'Mediterranean Quinoa Bowl',
    description: 'A healthy and delicious quinoa bowl with fresh vegetables, olives, and feta cheese.',
    prepTime: 15,
    cookTime: 20,
    totalTime: 35,
    servings: 4,
    difficulty: 'Easy',
    cuisine: 'Mediterranean',
    rating: 4.5,
    reviewCount: 23,
    calories: 420,
    tags: ['Healthy', 'Vegetarian', 'Quick', 'Protein-Rich'],
    ingredients: [
      '1 cup quinoa',
      '2 cups vegetable broth',
      '1 cucumber, diced',
      '2 tomatoes, chopped',
      '1/2 red onion, sliced',
      '1/4 cup olives',
      '100g feta cheese',
      '2 tbsp olive oil',
      '1 lemon, juiced',
      'Salt and pepper to taste'
    ],
    instructions: [
      'Cook quinoa in vegetable broth according to package instructions.',
      'While quinoa cooks, prepare vegetables.',
      'Mix olive oil and lemon juice for dressing.',
      'Combine all ingredients and serve.'
    ],
    image: '/api/placeholder/400/300',
    isFavorite: false,
    createdBy: 'Chef Maria',
    createdAt: '2024-01-15'
  },
  // Add more mock recipes...
];

interface Recipe {
  id: string;
  name: string;
  description: string;
  prepTime: number;
  cookTime: number;
  totalTime: number;
  servings: number;
  difficulty: string;
  cuisine: string;
  rating: number;
  reviewCount: number;
  calories: number;
  tags: string[];
  ingredients: string[];
  instructions: string[];
  image: string;
  isFavorite: boolean;
  createdBy: string;
  createdAt: string;
}

const RecipeManager: React.FC = () => {
  const [recipes, setRecipes] = useState<Recipe[]>(mockRecipes);
  const [filteredRecipes, setFilteredRecipes] = useState<Recipe[]>(mockRecipes);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedTab, setSelectedTab] = useState(0);
  const [filterAnchorEl, setFilterAnchorEl] = useState<null | HTMLElement>(null);
  const [selectedRecipe, setSelectedRecipe] = useState<Recipe | null>(null);
  const [recipeDialogOpen, setRecipeDialogOpen] = useState(false);

  const [importDialogOpen, setImportDialogOpen] = useState(false);
  const [repositoryUrl, setRepositoryUrl] = useState('https://github.com/dpapathanasiou/recipes.git');
  const [currentImportBatch, setCurrentImportBatch] = useState<string>('');

  const { favorites, toggleFavorite, isFavorite } = useRecipeFavorites();
  
  const { 
    data: recipesData = [], 
    isLoading: recipesLoading, 
    error: recipesError,
    refetch: refetchRecipes 
  } = useRecipeSearch({ 
    q: searchTerm, 
    tags: '', //selectedTags,
    limit: 50 
  });

  const { 
    mutate: importRecipes, 
    isLoading: importLoading 
  } = useRecipeImport();

  const { 
    data: importStatus,
    isLoading: statusLoading 
  } = useImportStatus(currentImportBatch);

  // Filter recipes based on search term and selected tab
  useEffect(() => {
    let filtered = recipesData.length > 0 ? recipesData : recipes;

    if (searchTerm) {
      filtered = filtered.filter(recipe =>
        recipe.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        recipe.tags.some(tag => tag.toLowerCase().includes(searchTerm.toLowerCase())) ||
        recipe.cuisine.toLowerCase().includes(searchTerm.toLowerCase())
      );
    }

    if (selectedTab === 1) {
      filtered = filtered.filter(recipe => recipe.isFavorite);
    }

    setFilteredRecipes(filtered);
  }, [searchTerm, selectedTab, recipesData, recipes]);

  const handleImportRecipes = () => {
    importRecipes(
      { repository_url: repositoryUrl },
      {
        onSuccess: (data) => {
          setCurrentImportBatch(data.batch_id);
          setImportDialogOpen(false);
        },
        onError: (error) => {
          console.error('Import failed:', error);
        }
      }
    );
  };

  const RecipeCard: React.FC<{ recipe: Recipe }> = ({ recipe }) => (
    <motion.div
      layout
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.9 }}
      transition={{ duration: 0.3 }}
    >
      <Card
        sx={{
          height: '100%',
          cursor: 'pointer',
          transition: 'transform 0.2s, box-shadow 0.2s',
          '&:hover': {
            transform: 'translateY(-4px)',
            boxShadow: 6,
          }
        }}
        onClick={() => {
          setSelectedRecipe(recipe);
          setRecipeDialogOpen(true);
        }}
      >
        <Box sx={{ position: 'relative' }}>
          <Box
            component="img"
            src={recipe.image}
            alt={recipe.name}
            sx={{
              width: '100%',
              height: 200,
              objectFit: 'cover',
              backgroundColor: 'grey.200'
            }}
          />
          <IconButton
            sx={{
              position: 'absolute',
              top: 8,
              right: 8,
              backgroundColor: 'rgba(255,255,255,0.9)',
              '&:hover': { backgroundColor: 'rgba(255,255,255,1)' }
            }}
            onClick={(e) => {
              e.stopPropagation();
              toggleFavorite(recipe.id);
            }}
          >
            {recipe.isFavorite ? <FavoriteIcon color="error" /> : <FavoriteBorderIcon />}
          </IconButton>
        </Box>

        <CardContent>
          <Typography variant="h6" gutterBottom noWrap>
            {recipe.name}
          </Typography>
          
          <Typography variant="body2" color="text.secondary" sx={{ mb: 2, height: 40, overflow: 'hidden' }}>
            {recipe.description}
          </Typography>

          <Stack direction="row" spacing={1} sx={{ mb: 2, flexWrap: 'wrap' }}>
            {recipe.tags.slice(0, 3).map((tag, index) => (
              <Chip key={index} label={tag} size="small" variant="outlined" />
            ))}
          </Stack>

          <Box display="flex" justifyContent="space-between" alignItems="center" sx={{ mb: 1 }}>
            <Box display="flex" alignItems="center" gap={1}>
              <TimeIcon fontSize="small" color="action" />
              <Typography variant="caption">{recipe.totalTime} min</Typography>
            </Box>
            <Box display="flex" alignItems="center" gap={1}>
              <RestaurantIcon fontSize="small" color="action" />
              <Typography variant="caption">{recipe.servings} servings</Typography>
            </Box>
          </Box>

          <Box display="flex" justifyContent="space-between" alignItems="center">
            <Box display="flex" alignItems="center" gap={0.5}>
              <Rating value={recipe.rating} precision={0.5} size="small" readOnly />
              <Typography variant="caption" color="text.secondary">
                ({recipe.reviewCount})
              </Typography>
            </Box>
            <Chip label={recipe.difficulty} size="small" color="primary" />
          </Box>
        </CardContent>
      </Card>
    </motion.div>
  );

  return (
    <Container maxWidth="xl" sx={{ py: 4 }}>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.5 }}
      >
        {/* Header */}
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={4}>
          <Box>
            <Typography variant="h3" component="h1" gutterBottom>
              🍳 Recipe Manager
            </Typography>
            <Typography variant="h6" color="text.secondary">
              Discover, create, and organize your favorite recipes
            </Typography>
          </Box>
          
          <Button
            variant="contained"
            size="large"
            startIcon={<AddIcon />}
            onClick={() => console.log('Create new recipe')}
          >
            Create Recipe
          </Button>
        </Box>

        {/* Import Status */}
        {currentImportBatch && importStatus && (
          <Alert 
            severity={
              importStatus.import_status === 'completed' ? 'success' :
              importStatus.import_status === 'failed' ? 'error' : 'info'
            }
            sx={{ mb: 3 }}
            action={
              importStatus.import_status === 'in_progress' && (
                <CircularProgress size={20} />
              )
            }
          >
            Import {importStatus.import_status}: {importStatus.successful_imports || 0} recipes imported
            {importStatus.import_status === 'in_progress' && ' (in progress...)'}
          </Alert>
        )}

        {/* Search and Filters */}
        <Paper sx={{ p: 2, mb: 3 }}>
          <Box display="flex" gap={2} alignItems="center">
            <TextField
              fullWidth
              placeholder="Search recipes, tags, or cuisine..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <SearchIcon />
                  </InputAdornment>
                ),
              }}
            />
            
            <Button
              variant="outlined"
              startIcon={<FilterIcon />}
              onClick={(e) => setFilterAnchorEl(e.currentTarget)}
            >
              Filters
            </Button>
          </Box>

          <Menu
            anchorEl={filterAnchorEl}
            open={Boolean(filterAnchorEl)}
            onClose={() => setFilterAnchorEl(null)}
          >
            <MenuItem onClick={() => setFilterAnchorEl(null)}>By Difficulty</MenuItem>
            <MenuItem onClick={() => setFilterAnchorEl(null)}>By Cuisine</MenuItem>
            <MenuItem onClick={() => setFilterAnchorEl(null)}>By Prep Time</MenuItem>
            <MenuItem onClick={() => setFilterAnchorEl(null)}>By Dietary Restrictions</MenuItem>
          </Menu>
        </Paper>

        {/* Tabs */}
        <Tabs value={selectedTab} onChange={(_, newValue) => setSelectedTab(newValue)} sx={{ mb: 3 }}>
          <Tab label={`All Recipes (${recipes.length})`} />
          <Tab label={`Favorites (${recipes.filter(r => r.isFavorite).length})`} />
          <Tab label="My Recipes" />
          <Tab label="Recently Viewed" />
        </Tabs>

        {/* Recipe Grid */}
        <Grid container spacing={3}>
          <AnimatePresence>
            {filteredRecipes.map((recipe) => (
              <Grid item xs={12} sm={6} md={4} lg={3} key={recipe.id}>
                <RecipeCard recipe={recipe} />
              </Grid>
            ))}
          </AnimatePresence>
        </Grid>

        {filteredRecipes.length === 0 && (
          <Paper sx={{ p: 6, textAlign: 'center', mt: 4 }}>
            <RestaurantIcon sx={{ fontSize: 64, color: 'text.secondary', mb: 2 }} />
            <Typography variant="h5" gutterBottom>
              No recipes found
            </Typography>
            <Typography color="text.secondary">
              Try adjusting your search or filters
            </Typography>
          </Paper>
        )}

        {/* Recipe Detail Dialog */}
        <Dialog 
          open={recipeDialogOpen} 
          onClose={() => setRecipeDialogOpen(false)}
          maxWidth="md"
          fullWidth
        >
          {selectedRecipe && (
            <Box p={2}>
              <Typography variant="h4">{selectedRecipe.name}</Typography>
              <Typography variant="body1" color="text.secondary">
                {selectedRecipe.description}
              </Typography>
            </Box>
          )}
        </Dialog>

        {/* Import Dialog */}
        <Dialog 
          open={importDialogOpen} 
          onClose={() => setImportDialogOpen(false)}
          maxWidth="sm"
          fullWidth
        >
          <DialogTitle>Import Recipe Collection</DialogTitle>
          <DialogContent>
            <TextField
              fullWidth
              label="Repository URL"
              value={repositoryUrl}
              onChange={(e) => setRepositoryUrl(e.target.value)}
              placeholder="https://github.com/dpapathanasiou/recipes.git"
              sx={{ mt: 2 }}
              helperText="Import recipes from a Git repository containing JSON recipe files"
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setImportDialogOpen(false)}>
              Cancel
            </Button>
            <Button 
              variant="contained" 
              onClick={handleImportRecipes}
              disabled={importLoading || !repositoryUrl}
            >
              {importLoading ? 'Importing...' : 'Import'}
            </Button>
          </DialogActions>
        </Dialog>

        {/* Floating Action Button */}
        <Fab
          color="primary"
          aria-label="add recipe"
          sx={{
            position: 'fixed',
            bottom: 24,
            right: 24,
          }}
          onClick={() => console.log('Quick add recipe')}
        >
          <AddIcon />
        </Fab>
      </motion.div>
    </Container>
  );
};

export default RecipeManager;
