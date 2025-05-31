-- Meal Prep Pro - Initial Database Schema
-- PostgreSQL 15+ with UUID support

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Custom types for better data integrity
CREATE TYPE activity_level AS ENUM ('sedentary', 'lightly_active', 'moderately_active', 'very_active', 'extra_active');
CREATE TYPE difficulty_level AS ENUM ('easy', 'intermediate', 'hard');
CREATE TYPE meal_type AS ENUM ('breakfast', 'lunch', 'dinner', 'snack');
CREATE TYPE goal_type AS ENUM ('weight_loss', 'weight_gain', 'muscle_gain', 'maintenance', 'health_improvement');

-- Users table with comprehensive profile support
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    date_of_birth DATE,
    height_cm DECIMAL(5,2),
    weight_kg DECIMAL(5,2),
    activity_level activity_level DEFAULT 'moderately_active',
    email_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    role VARCHAR(20) DEFAULT 'user',
    preferences JSONB DEFAULT '{}',
    dietary_restrictions TEXT[] DEFAULT '{}',
    allergies TEXT[] DEFAULT '{}',
    timezone VARCHAR(50) DEFAULT 'UTC',
    language_preference VARCHAR(10) DEFAULT 'en',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipes table with comprehensive recipe data
CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    cuisine_type VARCHAR(50),
    difficulty_level difficulty_level DEFAULT 'intermediate',
    prep_time_minutes INTEGER NOT NULL CHECK (prep_time_minutes >= 0),
    cook_time_minutes INTEGER NOT NULL CHECK (cook_time_minutes >= 0),
    total_time_minutes INTEGER GENERATED ALWAYS AS (prep_time_minutes + cook_time_minutes) STORED,
    servings INTEGER NOT NULL DEFAULT 1 CHECK (servings > 0),
    ingredients JSONB NOT NULL,
    instructions JSONB NOT NULL,
    nutrition_per_serving JSONB,
    tags TEXT[] DEFAULT '{}',
    rating DECIMAL(3,2) CHECK (rating >= 0 AND rating <= 5),
    rating_count INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT FALSE,
    source_url TEXT,
    image_url TEXT,
    cost_estimate DECIMAL(8,2),
    carbon_footprint DECIMAL(8,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Meal plans for weekly planning
CREATE TABLE meal_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    week_start DATE NOT NULL,
    week_end DATE GENERATED ALWAYS AS (week_start + INTERVAL '6 days') STORED,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, week_start)
);

-- Individual planned meals
CREATE TABLE planned_meals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    meal_plan_id UUID REFERENCES meal_plans(id) ON DELETE CASCADE,
    recipe_id UUID REFERENCES recipes(id) ON DELETE CASCADE,
    day_of_week INTEGER NOT NULL CHECK (day_of_week >= 1 AND day_of_week <= 7),
    meal_type meal_type NOT NULL,
    planned_date DATE NOT NULL,
    servings INTEGER DEFAULT 1 CHECK (servings > 0),
    is_prepared BOOLEAN DEFAULT FALSE,
    is_consumed BOOLEAN DEFAULT FALSE,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Nutrition goals for users
CREATE TABLE nutrition_goals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    goal_type goal_type NOT NULL,
    target_calories DECIMAL(8,2) NOT NULL CHECK (target_calories > 0),
    target_protein_g DECIMAL(8,2) NOT NULL CHECK (target_protein_g >= 0),
    target_carbs_g DECIMAL(8,2) NOT NULL CHECK (target_carbs_g >= 0),
    target_fat_g DECIMAL(8,2) NOT NULL CHECK (target_fat_g >= 0),
    target_fiber_g DECIMAL(8,2) DEFAULT 25.0,
    target_sodium_mg DECIMAL(8,2) DEFAULT 2300.0,
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_by_ai BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Daily nutrition tracking
CREATE TABLE daily_nutrition (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    calories_consumed DECIMAL(8,2) DEFAULT 0,
    protein_g DECIMAL(8,2) DEFAULT 0,
    carbs_g DECIMAL(8,2) DEFAULT 0,
    fat_g DECIMAL(8,2) DEFAULT 0,
    fiber_g DECIMAL(8,2) DEFAULT 0,
    sodium_mg DECIMAL(8,2) DEFAULT 0,
    water_ml DECIMAL(8,2) DEFAULT 0,
    additional_nutrients JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, date)
);

-- Meal consumption logs
CREATE TABLE meal_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    recipe_id UUID REFERENCES recipes(id),
    planned_meal_id UUID REFERENCES planned_meals(id),
    meal_type meal_type NOT NULL,
    logged_at TIMESTAMPTZ DEFAULT NOW(),
    consumed_date DATE NOT NULL,
    servings_consumed DECIMAL(4,2) NOT NULL DEFAULT 1.0,
    nutrition_consumed JSONB,
    satisfaction_rating INTEGER CHECK (satisfaction_rating >= 1 AND satisfaction_rating <= 5),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Shopping lists
CREATE TABLE shopping_lists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    meal_plan_id UUID REFERENCES meal_plans(id),
    is_completed BOOLEAN DEFAULT FALSE,
    total_estimated_cost DECIMAL(10,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Shopping list items
CREATE TABLE shopping_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    shopping_list_id UUID REFERENCES shopping_lists(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    quantity DECIMAL(8,2) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    category VARCHAR(100),
    estimated_price DECIMAL(8,2),
    is_purchased BOOLEAN DEFAULT FALSE,
    purchased_price DECIMAL(8,2),
    purchased_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User sessions for authentication tracking
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    refresh_token VARCHAR(255) UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Performance indexes for optimal query performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = TRUE;

CREATE INDEX idx_recipes_user_id ON recipes(user_id);
CREATE INDEX idx_recipes_public ON recipes(is_public) WHERE is_public = TRUE;
CREATE INDEX idx_recipes_cuisine ON recipes(cuisine_type);
CREATE INDEX idx_recipes_difficulty ON recipes(difficulty_level);
CREATE INDEX idx_recipes_prep_time ON recipes(prep_time_minutes);
CREATE INDEX idx_recipes_tags ON recipes USING GIN(tags);
CREATE INDEX idx_recipes_created ON recipes(created_at DESC);

CREATE INDEX idx_meal_plans_user_week ON meal_plans(user_id, week_start);
CREATE INDEX idx_meal_plans_active ON meal_plans(is_active) WHERE is_active = TRUE;

CREATE INDEX idx_planned_meals_plan ON planned_meals(meal_plan_id);
CREATE INDEX idx_planned_meals_recipe ON planned_meals(recipe_id);
CREATE INDEX idx_planned_meals_date ON planned_meals(planned_date);
CREATE INDEX idx_planned_meals_user_date ON planned_meals((SELECT user_id FROM meal_plans WHERE id = meal_plan_id), planned_date);

CREATE INDEX idx_nutrition_goals_user ON nutrition_goals(user_id);
CREATE INDEX idx_nutrition_goals_active ON nutrition_goals(is_active) WHERE is_active = TRUE;

CREATE INDEX idx_daily_nutrition_user_date ON daily_nutrition(user_id, date);
CREATE INDEX idx_daily_nutrition_date ON daily_nutrition(date DESC);

CREATE INDEX idx_meal_logs_user_date ON meal_logs(user_id, consumed_date);
CREATE INDEX idx_meal_logs_recipe ON meal_logs(recipe_id);
CREATE INDEX idx_meal_logs_planned ON meal_logs(planned_meal_id);

CREATE INDEX idx_shopping_lists_user ON shopping_lists(user_id);
CREATE INDEX idx_shopping_lists_meal_plan ON shopping_lists(meal_plan_id);

CREATE INDEX idx_shopping_items_list ON shopping_items(shopping_list_id);
CREATE INDEX idx_shopping_items_category ON shopping_items(category);
CREATE INDEX idx_shopping_items_purchased ON shopping_items(is_purchased);

CREATE INDEX idx_user_sessions_user ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX idx_user_sessions_active ON user_sessions(is_active, expires_at) WHERE is_active = TRUE;

-- Triggers for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_recipes_updated_at BEFORE UPDATE ON recipes FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_meal_plans_updated_at BEFORE UPDATE ON meal_plans FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_nutrition_goals_updated_at BEFORE UPDATE ON nutrition_goals FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_daily_nutrition_updated_at BEFORE UPDATE ON daily_nutrition FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_shopping_lists_updated_at BEFORE UPDATE ON shopping_lists FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample data for development
INSERT INTO users (id, username, email, password_hash, full_name) VALUES 
('550e8400-e29b-41d4-a716-446655440001', 'demo_user', 'demo@mealprep.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewNxAhcEHJY7u5jy', 'Demo User');

-- Sample recipe for testing
INSERT INTO recipes (user_id, name, description, prep_time_minutes, cook_time_minutes, servings, ingredients, instructions, nutrition_per_serving) VALUES 
('550e8400-e29b-41d4-a716-446655440001', 'Grilled Chicken Breast', 'Simple and healthy grilled chicken', 10, 15, 4, 
'[{"name": "chicken breast", "amount": 500, "unit": "g"}, {"name": "olive oil", "amount": 2, "unit": "tbsp"}, {"name": "salt", "amount": 1, "unit": "tsp"}, {"name": "pepper", "amount": 0.5, "unit": "tsp"}]',
'[{"step": 1, "instruction": "Season chicken with salt and pepper"}, {"step": 2, "instruction": "Heat grill to medium-high"}, {"step": 3, "instruction": "Grill chicken 6-7 minutes per side"}]',
'{"calories": 165, "protein": 31, "carbs": 0, "fat": 3.6, "fiber": 0, "sodium": 74}');
