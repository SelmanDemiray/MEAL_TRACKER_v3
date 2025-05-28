-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types
CREATE TYPE activity_level_enum AS ENUM (
    'sedentary', 'lightly_active', 'moderately_active', 'very_active', 'extra_active'
);

CREATE TYPE difficulty_enum AS ENUM (
    'beginner', 'intermediate', 'advanced', 'expert'
);

CREATE TYPE meal_type_enum AS ENUM (
    'breakfast', 'lunch', 'dinner', 'snack', 'pre_workout', 'post_workout'
);

CREATE TYPE plan_type_enum AS ENUM (
    'weekly', 'bi_weekly', 'monthly', 'custom'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    role VARCHAR(20) DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User profiles table
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
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipes table
CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    cuisine_type VARCHAR(50),
    difficulty_level difficulty_enum DEFAULT 'intermediate',
    prep_time_minutes INTEGER NOT NULL,
    cook_time_minutes INTEGER NOT NULL,
    servings INTEGER NOT NULL DEFAULT 1,
    calories_per_serving DECIMAL(8,2),
    cost_estimate DECIMAL(8,2),
    rating DECIMAL(3,2) CHECK (rating >= 0 AND rating <= 5),
    is_public BOOLEAN DEFAULT FALSE,
    image_url TEXT,
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipe ingredients table
CREATE TABLE recipe_ingredients (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_id UUID REFERENCES recipes(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    amount DECIMAL(10,3) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    preparation_note TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recipe steps table
CREATE TABLE recipe_steps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    recipe_id UUID REFERENCES recipes(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    instruction TEXT NOT NULL,
    time_estimate_minutes INTEGER,
    temperature VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Meal plans table
CREATE TABLE meal_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    plan_type plan_type_enum NOT NULL,
    total_cost_estimate DECIMAL(10,2),
    ai_generated BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Planned meals table
CREATE TABLE planned_meals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    meal_plan_id UUID REFERENCES meal_plans(id) ON DELETE CASCADE,
    recipe_id UUID REFERENCES recipes(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    meal_type meal_type_enum NOT NULL,
    servings INTEGER NOT NULL DEFAULT 1,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Nutrition goals table
CREATE TABLE nutrition_goals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    daily_calories DECIMAL(8,2),
    daily_protein DECIMAL(8,2),
    daily_carbs DECIMAL(8,2),
    daily_fat DECIMAL(8,2),
    daily_fiber DECIMAL(8,2),
    start_date DATE NOT NULL,
    end_date DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Daily nutrition logs table
CREATE TABLE daily_nutrition_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    total_calories DECIMAL(8,2) DEFAULT 0,
    total_protein DECIMAL(8,2) DEFAULT 0,
    total_carbs DECIMAL(8,2) DEFAULT 0,
    total_fat DECIMAL(8,2) DEFAULT 0,
    total_fiber DECIMAL(8,2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, date)
);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_recipes_user_id ON recipes(user_id);
CREATE INDEX idx_recipes_cuisine_type ON recipes(cuisine_type);
CREATE INDEX idx_recipes_difficulty ON recipes(difficulty_level);
CREATE INDEX idx_recipe_ingredients_recipe_id ON recipe_ingredients(recipe_id);
CREATE INDEX idx_meal_plans_user_id ON meal_plans(user_id);
CREATE INDEX idx_planned_meals_plan_id ON planned_meals(meal_plan_id);
CREATE INDEX idx_planned_meals_date ON planned_meals(date);
CREATE INDEX idx_nutrition_goals_user_id ON nutrition_goals(user_id);
CREATE INDEX idx_daily_nutrition_user_date ON daily_nutrition_logs(user_id, date);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Add updated_at triggers
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_user_profiles_updated_at BEFORE UPDATE ON user_profiles 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_recipes_updated_at BEFORE UPDATE ON recipes 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_meal_plans_updated_at BEFORE UPDATE ON meal_plans 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_nutrition_goals_updated_at BEFORE UPDATE ON nutrition_goals 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_daily_nutrition_logs_updated_at BEFORE UPDATE ON daily_nutrition_logs 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
