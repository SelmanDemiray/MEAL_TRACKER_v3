# 🧠 Nutrition Service - AI-Powered Nutrition Intelligence

Advanced AI-driven nutrition analysis and recommendation engine for the Meal Prep Pro platform.

## 🎯 Purpose

The Nutrition Service provides intelligent nutrition analysis and personalized recommendations through:
- **Real-time Nutritional Analysis**: Comprehensive analysis of meals and ingredients
- **AI-Powered Recommendations**: Machine learning-driven meal and supplement suggestions
- **Predictive Health Analytics**: Health trend analysis and deficiency prediction
- **Personalized Optimization**: Custom nutrition goals and meal optimization
- **Micronutrient Tracking**: Advanced nutrient analysis with bioavailability calculations

## 🧬 AI Capabilities

### Machine Learning Models
- **Nutrition Optimization**: Multi-objective optimization for nutrition, taste, and cost
- **Recommendation Engine**: Collaborative filtering + content-based recommendations
- **Deficiency Prediction**: Time-series analysis for nutrient deficiency prediction
- **Health Insights**: Pattern recognition for health trend analysis
- **Meal Pairing**: Smart ingredient and meal combination suggestions

### Data Science Stack
- **ndarray**: High-performance numerical computing
- **polars**: Fast DataFrame operations for large datasets
- **rayon**: Parallel processing for compute-intensive operations
- **Custom ML**: Rust-native machine learning implementations

## 🏗️ Architecture

```
┌─────────────────┐    ┌───────────────────┐    ┌──────────────────┐
│   API Gateway   │───►│ Nutrition Service │───►│    AI Engine     │
└─────────────────┘    └───────────────────┘    └──────────────────┘
                                │                         │
                                ▼                         ▼
                       ┌─────────────────┐    ┌──────────────────┐
                       │ Nutrition       │    │ Recommendation   │
                       │ Analyzer        │    │ Engine          │
                       └─────────────────┘    └──────────────────┘
                                │                         │
                                ▼                         ▼
                       ┌─────────────────┐    ┌──────────────────┐
                       │   Database      │    │      Cache       │
                       │  (Nutrition     │    │   (Fast Access   │
                       │     Data)       │    │     Results)     │
                       └─────────────────┘    └──────────────────┘
```

## 📁 Project Structure

```
nutrition-service/
├── src/
│   ├── main.rs                 # Service entry point & API routes
│   ├── ai_engine.rs           # Core AI/ML engine
│   ├── nutrition_analyzer.rs  # Nutritional analysis logic
│   ├── recommendation_engine.rs # AI recommendation system
│   ├── models.rs              # Data models and types
│   └── database.rs            # Database operations
├── models/                    # AI model files (when applicable)
├── data/                      # Training data and datasets
├── tests/                     # Test files
├── Cargo.toml                 # Dependencies
├── Dockerfile                 # Container definition
└── README.md                  # This file
```

## 🚀 Installation & Setup

### Prerequisites
- Rust 1.82+
- PostgreSQL 15+
- Redis 7+
- Optional: CUDA for GPU acceleration

### Local Development
```bash
# 1. Navigate to service directory
cd services/nutrition-service

# 2. Install dependencies
cargo build

# 3. Set up environment
cp .env.example .env
# Configure database and AI model paths

# 4. Download AI models (if applicable)
./scripts/download-models.sh

# 5. Start the service
cargo run
```

### Environment Variables
```bash
# Database & Cache
DATABASE_URL=postgresql://mealprep:mealprep_secure_2024@localhost:5432/mealprep
REDIS_URL=redis://localhost:6379

# AI Configuration
AI_MODEL_PATH=/app/models
ENABLE_GPU=false
MAX_BATCH_SIZE=32

# Performance
RUST_LOG=info
WORKER_THREADS=4
```

## 📡 API Endpoints

### Meal Analysis
```http
POST /analyze/meal
Content-Type: application/json

{
  "ingredients": [
    {
      "name": "chicken breast",
      "amount": 150,
      "unit": "g",
      "preparation": "grilled"
    }
  ],
  "portion_size": 1.0,
  "cooking_method": "grilled",
  "user_id": "uuid"
}
```

**Response:**
```json
{
  "basic_nutrition": {
    "calories": 231,
    "protein": 43.5,
    "carbohydrates": 0,
    "fat": 5.0,
    "fiber": 0,
    "sugar": 0,
    "sodium": 74
  },
  "micronutrients": [
    {
      "name": "Vitamin B6",
      "amount": 0.9,
      "unit": "mg",
      "daily_value_percentage": 45,
      "bioavailability": 0.85
    }
  ],
  "health_score": 87.5,
  "dietary_compliance": {
    "keto_friendly": true,
    "vegan": false,
    "gluten_free": true,
    "anti_inflammatory_score": 8.2
  },
  "optimization_suggestions": [
    {
      "category": "micronutrients",
      "suggestion": "Add spinach for iron and folate",
      "impact": "high",
      "estimated_improvement": 15.5
    }
  ],
  "environmental_impact": {
    "carbon_footprint": 6.1,
    "water_usage": 4325,
    "sustainability_score": 6.8
  }
}
```

### Daily Nutrition Analysis
```http
POST /analyze/daily
Content-Type: application/json

{
  "user_id": "uuid",
  "date": "2024-01-01T00:00:00Z",
  "meals": [
    {
      "meal_type": "breakfast",
      "ingredients": [...],
      "timestamp": "2024-01-01T08:00:00Z"
    }
  ]
}
```

### Meal Recommendations
```http
POST /recommendations/meals
Content-Type: application/json

{
  "user_id": "uuid",
  "meal_type": "dinner",
  "time_constraint": 30,
  "dietary_preferences": ["vegetarian"],
  "nutrition_targets": {
    "calories": 500,
    "protein": 25
  },
  "mood_state": "energetic",
  "recent_meals": ["pasta", "salad"]
}
```

### Supplement Recommendations
```http
POST /recommendations/supplements
Content-Type: application/json

{
  "user_id": "uuid",
  "current_nutrition": {
    "calories": 1800,
    "protein": 120
  },
  "health_goals": ["muscle_gain", "energy"]
}
```

### Nutrition Goals Calculation
```http
POST /goals/calculate
Content-Type: application/json

{
  "user_id": "uuid",
  "age": 30,
  "weight_kg": 70,
  "height_cm": 175,
  "activity_level": "moderately_active",
  "goals": ["weight_loss", "muscle_gain"]
}
```

### Trend Analysis
```http
GET /analyze/trends?user_id=uuid&timeframe=3months
```

### Deficiency Prediction
```http
POST /predict/deficiencies
Content-Type: application/json

{
  "user_id": "uuid",
  "nutrition_history": [...],
  "health_indicators": ["fatigue", "pale_skin"]
}
```

## 🤖 AI Engine Components

### 1. Nutrition Analyzer
```rust
pub struct NutritionalAnalyzer {
    // Core analysis engine
}

impl NutritionalAnalyzer {
    pub async fn analyze_meal(
        &self,
        ingredients: &[MealIngredient],
        portion_size: f32,
        cooking_method: Option<&str>,
    ) -> Result<NutritionAnalysis> {
        // Advanced nutritional analysis
        // Considers bioavailability, cooking methods, interactions
    }
}
```

### 2. Recommendation Engine
```rust
pub struct RecommendationEngine {
    ai_engine: Arc<NutritionAI>,
}

impl RecommendationEngine {
    pub async fn recommend_meals(
        &self,
        request: &MealRecommendationRequest,
    ) -> Result<Vec<MealRecommendation>> {
        // ML-driven meal recommendations
        // Considers user preferences, nutrition goals, variety
    }
}
```

### 3. AI Engine Core
```rust
pub struct NutritionAI {
    // Core AI/ML models and algorithms
}

impl NutritionAI {
    pub async fn generate_health_insights(
        &self,
        analysis: &NutritionAnalysis,
        user_id: Uuid,
    ) -> Result<HealthInsights> {
        // AI-powered health insights
        // Pattern recognition and predictive analytics
    }
}
```

## 🧮 Advanced Features

### Bioavailability Calculations
The service considers nutrient bioavailability factors:
- **Cooking Methods**: How preparation affects nutrient absorption
- **Food Combinations**: Synergistic and antagonistic nutrient interactions
- **Individual Factors**: User-specific absorption rates
- **Timing**: Meal timing effects on nutrient utilization

### Smart Optimization
Multi-objective optimization considering:
- **Nutritional Completeness**: Meeting all macro and micronutrient needs
- **Taste Preferences**: User taste profile and satisfaction
- **Cost Efficiency**: Budget-conscious meal planning
- **Preparation Time**: Time constraints and cooking complexity
- **Environmental Impact**: Sustainability and carbon footprint

### Predictive Analytics
- **Deficiency Prediction**: Early warning for potential nutrient deficiencies
- **Health Trend Analysis**: Long-term health pattern recognition
- **Goal Achievement Prediction**: Likelihood of reaching nutrition goals
- **Seasonal Adjustments**: Nutrition needs based on seasonal factors

## 📊 Data Models

### Basic Nutrition
```rust
pub struct BasicNutrition {
    pub calories: f32,
    pub protein: f32,
    pub carbohydrates: f32,
    pub fat: f32,
    pub fiber: f32,
    pub sugar: f32,
    pub sodium: f32,
}
```

### Micronutrient
```rust
pub struct Micronutrient {
    pub name: String,
    pub amount: f32,
    pub unit: String,
    pub daily_value_percentage: f32,
    pub bioavailability: f32,
}
```

### Optimization Suggestion
```rust
pub struct OptimizationSuggestion {
    pub category: String,
    pub suggestion: String,
    pub impact: String,
    pub difficulty: String,
    pub estimated_improvement: f32,
}
```

## 🧪 Testing

### Unit Tests
```bash
# Run all tests
cargo test

# Test specific modules
cargo test ai_engine
cargo test nutrition_analyzer

# Performance tests
cargo test --release -- --ignored
```

### AI Model Validation
```bash
# Validate recommendation accuracy
cargo test --test recommendation_accuracy

# Benchmark performance
cargo bench
```

### Example Test
```rust
#[tokio::test]
async fn test_meal_analysis_accuracy() {
    let analyzer = NutritionalAnalyzer::new();
    
    let ingredients = vec![
        MealIngredient {
            name: "chicken breast".to_string(),
            amount: 100.0,
            unit: "g".to_string(),
            preparation: Some("grilled".to_string()),
        }
    ];
    
    let analysis = analyzer
        .analyze_meal(&ingredients, 1.0, Some("grilled"))
        .await
        .unwrap();
    
    // Verify nutritional accuracy within acceptable range
    assert!((analysis.basic_nutrition.calories - 165.0).abs() < 10.0);
    assert!((analysis.basic_nutrition.protein - 31.0).abs() < 2.0);
}
```

## 📈 Performance Optimization

### Parallel Processing
```rust
use rayon::prelude::*;

// Parallel nutrient calculations
let nutrient_results: Vec<_> = ingredients
    .par_iter()
    .map(|ingredient| calculate_nutrients(ingredient))
    .collect();
```

### Caching Strategy
- **Ingredient Data**: Cache nutritional data for common ingredients
- **User Profiles**: Cache user preferences and dietary restrictions
- **Analysis Results**: Cache complex analysis results for similar meals
- **Model Predictions**: Cache ML model predictions for similar inputs

### Memory Management
- **Zero-copy Operations**: Minimize data copying where possible
- **Efficient Data Structures**: Use appropriate data structures for each use case
- **Memory Pooling**: Reuse allocations for frequently used objects

## 🔬 AI Model Training

### Data Collection
```bash
# Prepare training data
python scripts/prepare_training_data.py

# Validate data quality
cargo run --bin validate_data
```

### Model Training
```bash
# Train recommendation model
cargo run --bin train_recommendation_model

# Train deficiency prediction model
cargo run --bin train_deficiency_model

# Evaluate model performance
cargo run --bin evaluate_models
```

### Model Deployment
```bash
# Export trained models
cargo run --bin export_models

# Load models into service
# Models are loaded automatically on service startup
```

## 🔧 Configuration

### AI Model Parameters
```toml
[ai_config]
recommendation_model_path = "/models/recommendation.bin"
deficiency_model_path = "/models/deficiency.bin"
batch_size = 32
learning_rate = 0.001
```

### Performance Tuning
```toml
[performance]
worker_threads = 4
max_concurrent_analyses = 100
cache_ttl_seconds = 3600
enable_gpu = false
```

## 🐛 Troubleshooting

### Common Issues

**AI Model Loading Failed**
```bash
# Check model files exist
ls -la /app/models/

# Verify model format
cargo run --bin validate_models
```

**Analysis Accuracy Issues**
```bash
# Update nutrition database
cargo run --bin update_nutrition_db

# Recalibrate models
cargo run --bin recalibrate_models
```

**Performance Issues**
```bash
# Enable performance profiling
RUST_LOG=debug cargo run

# Check system resources
htop
nvidia-smi  # If using GPU
```

### Monitoring

**Key Metrics**
- Analysis request rate and latency
- AI model prediction accuracy
- Cache hit rates
- Memory and CPU usage
- GPU utilization (if applicable)

**Health Checks**
```bash
curl http://localhost:8081/health
```

## 🚀 Future Enhancements

### Planned Features
- **Advanced ML Models**: Deep learning for more accurate predictions
- **Real-time Learning**: Continuous model updates based on user feedback
- **Computer Vision**: Image-based food recognition and analysis
- **Genetic Integration**: Personalized nutrition based on genetic data
- **IoT Integration**: Data from smart kitchen appliances and wearables

### Research Areas
- **Personalized Metabolism**: Individual metabolic rate calculations
- **Gut Microbiome**: Nutrition recommendations based on microbiome data
- **Circadian Nutrition**: Time-based nutrition optimization
- **Social Nutrition**: Community-based recommendation features

---

## 📞 Support

For issues specific to the Nutrition Service:
1. Check AI model loading and paths
2. Verify nutrition database is up-to-date
3. Test individual analysis endpoints
4. Check system resources (CPU, memory, GPU)
5. Review AI model performance metrics

For general platform issues, see the main project README.
