-- Initial schema for Meal Prep Pro platform
-- This creates the foundational tables for users, authentication, and core data

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Custom types
CREATE TYPE activity_level_enum AS ENUM (
    'sedentary',
    'lightly_active', 
    'moderately_active',
    'very_active',
    'extra_active'
);

CREATE TYPE difficulty_enum AS ENUM (
    'beginner',
    'intermediate', 
    'advanced',
    'expert'
);

CREATE TYPE goal_type_enum AS ENUM (
    'weight_loss',
    'weight_gain',
    'muscle_gain',
    'maintenance',
    'improved_energy',
    'better_digestion',
    'reduced_inflammation'
);

CREATE TYPE meal_type_enum AS ENUM (
    'breakfast',
    'lunch', 
    'dinner',
    'snack',
    'pre_workout',
    'post_workout'
);

CREATE TYPE prep_status_enum AS ENUM (
    'not_started',
    'in_progress',
    'completed',
    'skipped'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    role VARCHAR(20) DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User profiles
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    full_name VARCHAR(255),
    date_of_birth DATE,
    height_cm DECIMAL(5,2),
    weight_kg DECIMAL(5,2),
    activity_level activity_level_enum DEFAULT 'moderately_active',
    dietary_restrictions TEXT[],
    allergies TEXT[],
    health_goals TEXT[],
    timezone VARCHAR(50) DEFAULT 'UTC',
    language_preference VARCHAR(10) DEFAULT 'en',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User preferences
CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    preferred_cuisines TEXT[],
    cooking_time_preference VARCHAR(20) DEFAULT 'medium',
    meal_prep_frequency VARCHAR(20) DEFAULT 'weekly',
    budget_min_per_meal DECIMAL(8,2),
    budget_max_per_meal DECIMAL(8,2),
    weekly_budget_limit DECIMAL(8,2),
    kitchen_equipment TEXT[],
    notification_meal_reminders BOOLEAN DEFAULT TRUE,
    notification_prep_reminders BOOLEAN DEFAULT TRUE,
    notification_shopping_reminders BOOLEAN DEFAULT TRUE,
    notification_goal_updates BOOLEAN DEFAULT TRUE,
    notification_weekly_summary BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Refresh tokens for JWT
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    is_revoked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User sessions
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_accessed TIMESTAMPTZ DEFAULT NOW()
);

-- Recipes
CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    cuisine_type VARCHAR(50),
    difficulty_level difficulty_enum DEFAULT 'intermediate',
    prep_time_minutes INTEGER NOT NULL,
    cook_time_minutes INTEGER NOT NULL,
    total_time_minutes INTEGER GENERATED ALWAYS AS (prep_time_minutes + cook_time_minutes) STORED,
    servings INTEGER NOT NULL DEFAULT 1,
    calories_per_serving DECIMAL(8,2),
    cost_estimate DECIMAL(8,2),
    rating DECIMAL(3,2) CHECK (rating >= 0 AND rating <= 5),
    rating_count INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT FALSE,
    source_url TEXT,
    image_url TEXT,
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipe ingredients
CREATE TABLE recipe_ingredients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    ingredient_name VARCHAR(255) NOT NULL,
    amount DECIMAL(10,3) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    preparation_note TEXT,
    substitutions TEXT[],
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipe steps
CREATE TABLE recipe_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    instruction TEXT NOT NULL,
    time_estimate_minutes INTEGER,
    temperature VARCHAR(50),
    equipment_needed TEXT[],
    tips TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipe nutrition info
CREATE TABLE recipe_nutrition (
    recipe_id UUID PRIMARY KEY REFERENCES recipes(id) ON DELETE CASCADE,
    calories_per_serving DECIMAL(8,2),
    protein_g DECIMAL(8,2),
    carbohydrates_g DECIMAL(8,2),
    fat_g DECIMAL(8,2),
    fiber_g DECIMAL(8,2),
    sugar_g DECIMAL(8,2),
    sodium_mg DECIMAL(8,2),
    vitamins JSONB,
    minerals JSONB,
    nutrition_score DECIMAL(3,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Nutrition goals
CREATE TABLE nutrition_goals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    goal_type goal_type_enum NOT NULL,
    target_calories DECIMAL(8,2),
    target_protein_g DECIMAL(8,2),
    target_carbs_g DECIMAL(8,2),
    target_fat_g DECIMAL(8,2),
    target_fiber_g DECIMAL(8,2),
    target_sodium_mg DECIMAL(8,2),
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_by_ai BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Daily nutrition tracking
CREATE TABLE daily_nutrition (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    total_calories DECIMAL(8,2) DEFAULT 0,
    total_protein_g DECIMAL(8,2) DEFAULT 0,
    total_carbs_g DECIMAL(8,2) DEFAULT 0,
    total_fat_g DECIMAL(8,2) DEFAULT 0,
    total_fiber_g DECIMAL(8,2) DEFAULT 0,
    total_sodium_mg DECIMAL(8,2) DEFAULT 0,
    goal_adherence_score DECIMAL(3,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, date)
);

-- Meal plans
CREATE TABLE meal_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    total_cost_estimate DECIMAL(10,2),
    ai_generated BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Planned meals
CREATE TABLE planned_meals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meal_plan_id UUID NOT NULL REFERENCES meal_plans(id) ON DELETE CASCADE,
    recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    scheduled_date DATE NOT NULL,
    meal_type meal_type_enum NOT NULL,
    servings INTEGER NOT NULL DEFAULT 1,
    prep_status prep_status_enum DEFAULT 'not_started',
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Meal logs (actual consumption)
CREATE TABLE meal_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    recipe_id UUID REFERENCES recipes(id) ON DELETE SET NULL,
    meal_type meal_type_enum NOT NULL,
    servings DECIMAL(4,2) NOT NULL DEFAULT 1,
    logged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Shopping lists
CREATE TABLE shopping_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    meal_plan_id UUID REFERENCES meal_plans(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    is_completed BOOLEAN DEFAULT FALSE,
    total_estimated_cost DECIMAL(10,2),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Shopping list items
CREATE TABLE shopping_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    shopping_list_id UUID NOT NULL REFERENCES shopping_lists(id) ON DELETE CASCADE,
    ingredient_name VARCHAR(255) NOT NULL,
    amount DECIMAL(10,3) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    estimated_cost DECIMAL(8,2),
    is_purchased BOOLEAN DEFAULT FALSE,
    category VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User activity logs
CREATE TABLE user_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity_type VARCHAR(100) NOT NULL,
    activity_data JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create partitions for user activities (current year)
CREATE TABLE user_activities_2024 PARTITION OF user_activities
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');

-- API request logs
CREATE TABLE api_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    method VARCHAR(10) NOT NULL,
    path TEXT NOT NULL,
    status_code INTEGER NOT NULL,
    response_time_ms INTEGER,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create partitions for API requests
CREATE TABLE api_requests_2024 PARTITION OF api_requests
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');

-- Indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);

CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);

CREATE INDEX idx_recipes_user_id ON recipes(user_id);
CREATE INDEX idx_recipes_cuisine_type ON recipes(cuisine_type);
CREATE INDEX idx_recipes_difficulty ON recipes(difficulty_level);
CREATE INDEX idx_recipes_prep_time ON recipes(prep_time_minutes);
CREATE INDEX idx_recipes_tags ON recipes USING GIN(tags);
CREATE INDEX idx_recipes_created_at ON recipes(created_at);

CREATE INDEX idx_recipe_ingredients_recipe_id ON recipe_ingredients(recipe_id);
CREATE INDEX idx_recipe_steps_recipe_id ON recipe_steps(recipe_id);

CREATE INDEX idx_nutrition_goals_user_id ON nutrition_goals(user_id);
CREATE INDEX idx_nutrition_goals_active ON nutrition_goals(user_id, is_active);

CREATE INDEX idx_daily_nutrition_user_date ON daily_nutrition(user_id, date);

CREATE INDEX idx_meal_plans_user_id ON meal_plans(user_id);
CREATE INDEX idx_meal_plans_dates ON meal_plans(start_date, end_date);

CREATE INDEX idx_planned_meals_plan_id ON planned_meals(meal_plan_id);
CREATE INDEX idx_planned_meals_date_type ON planned_meals(scheduled_date, meal_type);

CREATE INDEX idx_meal_logs_user_id ON meal_logs(user_id);
CREATE INDEX idx_meal_logs_logged_at ON meal_logs(logged_at);

CREATE INDEX idx_shopping_lists_user_id ON shopping_lists(user_id);
CREATE INDEX idx_shopping_items_list_id ON shopping_items(shopping_list_id);

CREATE INDEX idx_user_activities_user_type ON user_activities(user_id, activity_type);
CREATE INDEX idx_api_requests_created_at ON api_requests(created_at);

-- Trigger function for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers to tables with updated_at columns
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON user_profiles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_preferences_updated_at BEFORE UPDATE ON user_preferences
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_recipes_updated_at BEFORE UPDATE ON recipes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_recipe_nutrition_updated_at BEFORE UPDATE ON recipe_nutrition
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_nutrition_goals_updated_at BEFORE UPDATE ON nutrition_goals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_daily_nutrition_updated_at BEFORE UPDATE ON daily_nutrition
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_meal_plans_updated_at BEFORE UPDATE ON meal_plans
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_planned_meals_updated_at BEFORE UPDATE ON planned_meals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_shopping_lists_updated_at BEFORE UPDATE ON shopping_lists
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_shopping_items_updated_at BEFORE UPDATE ON shopping_items
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
