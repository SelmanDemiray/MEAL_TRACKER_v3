import React, { useState } from 'react';
import {
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  CardMedia,
  Button,
  IconButton,
  Chip,
  TextField,
  InputAdornment,
  Menu,
  MenuItem,
  Rating,
  Paper,
  Stack,
} from '@mui/material';
import {
  Add as AddIcon,
  Search as SearchIcon,
  FilterList,
  Favorite,
  FavoriteBorder,
  Schedule,
  Person,
  MoreVert,
} from '@mui/icons-material';
import { motion } from 'framer-motion';

// Mock recipe data
const mockRecipes = [
  {
    id: '1',
    name: 'Mediterranean Quinoa Bowl',
    description: 'A healthy and delicious bowl packed with Mediterranean flavors',
    image: '/api/placeholder/300/200',
    cookTime: 25,
    servings: 4,
    difficulty: 'Easy',
    rating: 4.8,
    tags: ['Healthy', 'Vegetarian', 'Mediterranean'],
    calories: 420,
    isFavorite: true,
  },
  {
    id: '2',
    name: 'Grilled Salmon with Asparagus',
    description: 'Perfectly grilled salmon with seasoned asparagus spears',
    image: '/api/placeholder/300/200',
    cookTime: 20,
    servings: 2,
    difficulty: 'Medium',
    rating: 4.6,
    tags: ['Healthy', 'Keto', 'High Protein'],
    calories: 380,
    isFavorite: false,
  },
  {
    id: '3',
    name: 'Chicken Stir Fry',
    description: 'Quick and easy chicken stir fry with fresh vegetables',
    image: '/api/placeholder/300/200',
    cookTime: 15,
    servings: 3,
    difficulty: 'Easy',
    rating: 4.4,
    tags: ['Quick', 'Asian', 'High Protein'],
    calories: 350,
    isFavorite: true,
  },
  {
    id: '4',
    name: 'Overnight Oats Parfait',
    description: 'Creamy overnight oats layered with fresh berries',
    image: '/api/placeholder/300/200',
    cookTime: 5,
    servings: 1,
    difficulty: 'Easy',
    rating: 4.9,
    tags: ['Breakfast', 'Healthy', 'Make-ahead'],
    calories: 280,
    isFavorite: false,
  },
];

const difficultyColors = {
  'Easy': 'success',
  'Medium': 'warning',
  'Hard': 'error',
} as const;

const RecipeManager: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [filterAnchorEl, setFilterAnchorEl] = useState<null | HTMLElement>(null);
  const [selectedFilter, setSelectedFilter] = useState('all');
  const [recipes, setRecipes] = useState(mockRecipes);

  const handleToggleFavorite = (recipeId: string) => {
    setRecipes(prev => prev.map(recipe => 
      recipe.id === recipeId 
        ? { ...recipe, isFavorite: !recipe.isFavorite }
        : recipe
    ));
  };

  const filteredRecipes = recipes.filter(recipe => {
    const matchesSearch = recipe.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         recipe.description.toLowerCase().includes(searchTerm.toLowerCase());
    
    const matchesFilter = selectedFilter === 'all' || 
                         (selectedFilter === 'favorites' && recipe.isFavorite) ||
                         recipe.tags.some(tag => tag.toLowerCase() === selectedFilter.toLowerCase());
    
    return matchesSearch && matchesFilter;
  });

  const RecipeCard = ({ recipe }: { recipe: typeof mockRecipes[0] }) => {
    const [menuAnchorEl, setMenuAnchorEl] = useState<null | HTMLElement>(null);

    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
      >
        <Card 
          sx={{ 
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
            transition: 'transform 0.2s, box-shadow 0.2s',
            '&:hover': {
              transform: 'translateY(-4px)',
              boxShadow: 4,
            }
          }}
        >
          <Box position="relative">
            <CardMedia
              component="img"
              height="200"
              image={recipe.image}
              alt={recipe.name}
              sx={{ backgroundColor: 'grey.200' }}
            />
            <IconButton
              sx={{ 
                position: 'absolute', 
                top: 8, 
                right: 8,
                backgroundColor: 'rgba(255,255,255,0.9)',
                '&:hover': { backgroundColor: 'rgba(255,255,255,1)' }
              }}
              onClick={() => handleToggleFavorite(recipe.id)}
            >
              {recipe.isFavorite ? (
                <Favorite color="error" />
              ) : (
                <FavoriteBorder />
              )}
            </IconButton>
            
            <Chip
              label={recipe.difficulty}
              size="small"
              color={difficultyColors[recipe.difficulty as keyof typeof difficultyColors]}
              sx={{ 
                position: 'absolute',
                bottom: 8,
                left: 8,
              }}
            />
          </Box>
          
          <CardContent sx={{ flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
            <Box display="flex" justifyContent="space-between" alignItems="flex-start" mb={1}>
              <Typography variant="h6" component="h3" sx={{ flexGrow: 1 }}>
                {recipe.name}
              </Typography>
              <IconButton 
                size="small"
                onClick={(e) => setMenuAnchorEl(e.currentTarget)}
              >
                <MoreVert />
              </IconButton>
            </Box>
            
            <Typography variant="body2" color="text.secondary" sx={{ mb: 2, flexGrow: 1 }}>
              {recipe.description}
            </Typography>
            
            <Box display="flex" alignItems="center" gap={2} mb={2}>
              <Box display="flex" alignItems="center" gap={0.5}>
                <Schedule fontSize="small" color="action" />
                <Typography variant="caption">{recipe.cookTime}min</Typography>
              </Box>
              <Box display="flex" alignItems="center" gap={0.5}>
                <Person fontSize="small" color="action" />
                <Typography variant="caption">{recipe.servings} servings</Typography>
              </Box>
              <Chip 
                label={`${recipe.calories} cal`} 
                size="small" 
                variant="outlined"
                color="primary"
              />
            </Box>
            
            <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
              <Rating value={recipe.rating} precision={0.1} size="small" readOnly />
              <Typography variant="caption" color="text.secondary">
                ({recipe.rating})
              </Typography>
            </Box>
            
            <Stack direction="row" spacing={1} sx={{ mb: 2, flexWrap: 'wrap', gap: 0.5 }}>
              {recipe.tags.slice(0, 3).map((tag, index) => (
                <Chip 
                  key={index}
                  label={tag} 
                  size="small" 
                  variant="outlined"
                  sx={{ fontSize: '0.7rem' }}
                />
              ))}
            </Stack>
            
            <Button
              variant="contained"
              fullWidth
              onClick={() => console.log('View recipe', recipe.id)}
            >
              View Recipe
            </Button>
          </CardContent>
          
          <Menu
            anchorEl={menuAnchorEl}
            open={Boolean(menuAnchorEl)}
            onClose={() => setMenuAnchorEl(null)}
          >
            <MenuItem onClick={() => console.log('Edit recipe')}>Edit Recipe</MenuItem>
            <MenuItem onClick={() => console.log('Add to meal plan')}>Add to Meal Plan</MenuItem>
            <MenuItem onClick={() => console.log('Share recipe')}>Share Recipe</MenuItem>
            <MenuItem onClick={() => console.log('Delete recipe')} sx={{ color: 'error.main' }}>
              Delete Recipe
            </MenuItem>
          </Menu>
        </Card>
      </motion.div>
    );
  };

  return (
    <Box p={3}>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4">Recipe Manager</Typography>
          <Typography variant="body1" color="text.secondary">
            Discover, create, and organize your favorite recipes
          </Typography>
        </Box>
        
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => console.log('Create new recipe')}
        >
          New Recipe
        </Button>
      </Box>

      {/* Search and Filters */}
      <Paper sx={{ p: 2, mb: 3 }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item xs={12} md={6}>
            <TextField
              fullWidth
              placeholder="Search recipes..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <SearchIcon color="action" />
                  </InputAdornment>
                ),
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <Box display="flex" gap={2} justifyContent="flex-end">
              <Button
                variant={selectedFilter === 'all' ? 'contained' : 'outlined'}
                onClick={() => setSelectedFilter('all')}
              >
                All Recipes
              </Button>
              <Button
                variant={selectedFilter === 'favorites' ? 'contained' : 'outlined'}
                onClick={() => setSelectedFilter('favorites')}
                startIcon={<Favorite />}
              >
                Favorites
              </Button>
              <Button
                variant="outlined"
                startIcon={<FilterList />}
                onClick={(e) => setFilterAnchorEl(e.currentTarget)}
              >
                Filter
              </Button>
            </Box>
          </Grid>
        </Grid>
      </Paper>

      {/* Filter Menu */}
      <Menu
        anchorEl={filterAnchorEl}
        open={Boolean(filterAnchorEl)}
        onClose={() => setFilterAnchorEl(null)}
      >
        <MenuItem onClick={() => { setSelectedFilter('healthy'); setFilterAnchorEl(null); }}>
          Healthy
        </MenuItem>
        <MenuItem onClick={() => { setSelectedFilter('quick'); setFilterAnchorEl(null); }}>
          Quick & Easy
        </MenuItem>
        <MenuItem onClick={() => { setSelectedFilter('vegetarian'); setFilterAnchorEl(null); }}>
          Vegetarian
        </MenuItem>
        <MenuItem onClick={() => { setSelectedFilter('high protein'); setFilterAnchorEl(null); }}>
          High Protein
        </MenuItem>
      </Menu>

      {/* Recipe Grid */}
      <Grid container spacing={3}>
        {filteredRecipes.map((recipe) => (
          <Grid item xs={12} sm={6} lg={4} xl={3} key={recipe.id}>
            <RecipeCard recipe={recipe} />
          </Grid>
        ))}
      </Grid>

      {/* Empty State */}
      {filteredRecipes.length === 0 && (
        <Box textAlign="center" py={8}>
          <Typography variant="h6" color="text.secondary" mb={2}>
            No recipes found
          </Typography>
          <Typography variant="body2" color="text.secondary" mb={3}>
            Try adjusting your search or filter criteria
          </Typography>
          <Button
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => console.log('Create first recipe')}
          >
            Create Your First Recipe
          </Button>
        </Box>
      )}
    </Box>
  );
};

export default RecipeManager;
