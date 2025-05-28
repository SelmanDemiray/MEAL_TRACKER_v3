# 🗄️ Database Layer - PostgreSQL Schema & Management

The database layer for Meal Prep Pro, built on PostgreSQL 15 with comprehensive schemas for all platform features.

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    PostgreSQL Database                     │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │    Users    │  │    Meals    │  │    Nutrition        │ │
│  │ Management  │  │ & Recipes   │  │    Tracking         │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  Shopping   │  │ Analytics   │  │    Real-time        │ │
│  │    Lists    │  │   & Logs    │  │   WebSocket Data    │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 📋 Schema Overview

### Core Tables

#### Users & Authentication
- `users` - User accounts and basic info
- `user_profiles` - Extended user profile data
- `user_preferences` - Dietary preferences and settings
- `user_sessions` - Active user sessions
- `refresh_tokens` - JWT refresh tokens

#### Recipes & Meals
- `recipes` - Recipe definitions and metadata
- `recipe_ingredients` - Recipe ingredient relationships
- `recipe_steps` - Cooking instructions
- `recipe_nutrition` - Nutritional analysis
- `recipe_tags` - Categorization and search tags
- `recipe_ratings` - User ratings and reviews

#### Meal Planning
- `meal_plans` - Meal planning schedules
- `planned_meals` - Individual meal assignments
- `meal_prep_sessions` - Meal preparation tracking
- `meal_logs` - Actual consumption tracking

#### Nutrition & Analytics
- `nutrition_goals` - User nutrition targets
- `daily_nutrition` - Daily nutrition summaries
- `nutrient_targets` - Individual nutrient goals
- `nutrition_insights` - AI-generated insights
- `health_metrics` - User health tracking data

#### Shopping & Inventory
- `shopping_lists` - Shopping list management
- `shopping_items` - Individual shopping items
- `inventory_items` - Pantry/fridge inventory
- `ingredient_prices` - Historical price tracking

#### Analytics & Logging
- `user_activities` - User behavior tracking
- `api_requests` - API usage analytics
- `performance_metrics` - System performance data
- `ai_model_results` - ML model predictions and results

## 🚀 Quick Setup

### Prerequisites
- PostgreSQL 15+
- sqlx-cli (for migrations)

### Database Setup
```bash
# 1. Start PostgreSQL (Docker)
docker run --name mealprep-db \
  -e POSTGRES_DB=mealprep \
  -e POSTGRES_USER=mealprep \
  -e POSTGRES_PASSWORD=mealprep_secure_2024 \
  -p 35432:5432 \
  -d postgres:15-alpine

# 2. Install sqlx-cli
cargo install sqlx-cli

# 3. Set database URL
export DATABASE_URL="postgresql://mealprep:mealprep_secure_2024@localhost:35432/mealprep"

# 4. Run migrations
cd services/api-gateway
sqlx migrate run
```

## 📊 Detailed Schema

### Users Table
```sql
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
```

### User Profiles Table
```sql
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
```

### Recipes Table
```sql
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
```

### Nutrition Goals Table
```sql
CREATE TABLE nutrition_goals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
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
```

## 🔧 Migrations Management

### Creating Migrations
```bash
# Create a new migration
sqlx migrate add create_table_name

# Example: Add user preferences table
sqlx migrate add create_user_preferences_table
```

### Migration File Structure
```
migrations/
├── 001_initial_schema.sql          # Core tables
├── 002_add_nutrition_tracking.sql  # Nutrition features
├── 003_add_meal_planning.sql       # Meal planning
├── 004_add_shopping_lists.sql      # Shopping features
├── 005_add_analytics_tables.sql    # Analytics & reporting
├── 006_add_ai_insights.sql         # AI/ML data tables
└── 007_add_indexes_optimization.sql # Performance indexes
```

### Running Migrations
```bash
# Run all pending migrations
sqlx migrate run

# Check migration status
sqlx migrate info

# Revert last migration
sqlx migrate revert
```

## 📈 Performance Optimization

### Indexes Strategy
```sql
-- User lookup optimization
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Recipe search optimization
CREATE INDEX idx_recipes_cuisine_type ON recipes(cuisine_type);
CREATE INDEX idx_recipes_difficulty ON recipes(difficulty_level);
CREATE INDEX idx_recipes_prep_time ON recipes(prep_time_minutes);
CREATE INDEX idx_recipes_tags ON recipes USING GIN(tags);

-- Nutrition tracking optimization
CREATE INDEX idx_daily_nutrition_user_date ON daily_nutrition(user_id, date);
CREATE INDEX idx_meal_logs_user_timestamp ON meal_logs(user_id, logged_at);

-- Analytics optimization
CREATE INDEX idx_user_activities_user_type ON user_activities(user_id, activity_type);
CREATE INDEX idx_api_requests_timestamp ON api_requests(created_at);
```

### Partitioning Strategy
```sql
-- Partition large analytics tables by date
CREATE TABLE api_requests_2024 PARTITION OF api_requests
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');

CREATE TABLE user_activities_2024 PARTITION OF user_activities
FOR VALUES FROM ('2024-01-01') TO ('2025-01-01');
```

## 🔐 Security & Compliance

### Row-Level Security (RLS)
```sql
-- Enable RLS on sensitive tables
ALTER TABLE user_profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE nutrition_goals ENABLE ROW LEVEL SECURITY;
ALTER TABLE meal_plans ENABLE ROW LEVEL SECURITY;

-- Users can only access their own data
CREATE POLICY user_profile_access ON user_profiles
    FOR ALL TO authenticated_user
    USING (user_id = current_user_id());
```

### Data Encryption
- Passwords: bcrypt hashed
- Sensitive fields: Application-level encryption
- API keys: Encrypted at rest
- Database connections: SSL/TLS required

### Audit Logging
```sql
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name VARCHAR(64) NOT NULL,
    operation VARCHAR(10) NOT NULL,
    user_id UUID,
    old_values JSONB,
    new_values JSONB,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);

-- Audit trigger function
CREATE OR REPLACE FUNCTION audit_trigger_function()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit_log (table_name, operation, user_id, old_values, new_values)
    VALUES (TG_TABLE_NAME, TG_OP, current_user_id(), 
            row_to_json(OLD), row_to_json(NEW));
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;
```

## 📊 Monitoring & Analytics

### Database Metrics
```sql
-- Monitor table sizes
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size
FROM pg_tables 
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

-- Monitor query performance
SELECT 
    query,
    calls,
    total_time,
    mean_time,
    rows
FROM pg_stat_statements
ORDER BY total_time DESC
LIMIT 10;
```

### Health Checks
```sql
-- Database health check query
SELECT 
    'healthy' as status,
    version() as postgres_version,
    current_database() as database_name,
    pg_database_size(current_database()) as database_size_bytes,
    (SELECT count(*) FROM users) as total_users,
    now() as timestamp;
```

## 🧪 Testing

### Test Database Setup
```bash
# Create test database
createdb mealprep_test

# Run migrations on test DB
DATABASE_URL="postgresql://mealprep:password@localhost:5432/mealprep_test" sqlx migrate run

# Seed test data
psql mealprep_test < test_data.sql
```

### Data Seeding
```sql
-- Insert test users
INSERT INTO users (id, username, email, password_hash) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'testuser1', 'test1@example.com', '$2b$12$...');

-- Insert test recipes
INSERT INTO recipes (user_id, name, prep_time_minutes, cook_time_minutes, servings) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'Test Recipe', 15, 30, 4);
```

## 🔄 Backup & Recovery

### Automated Backups
```bash
#!/bin/bash
# backup_database.sh
BACKUP_DIR="/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/mealprep_backup_$TIMESTAMP.sql"

pg_dump -h localhost -U mealprep -d mealprep > $BACKUP_FILE
gzip $BACKUP_FILE

# Clean up old backups (keep last 7 days)
find $BACKUP_DIR -name "mealprep_backup_*.sql.gz" -mtime +7 -delete
```

### Recovery Process
```bash
# Restore from backup
gunzip mealprep_backup_20240101_120000.sql.gz
psql -h localhost -U mealprep -d mealprep < mealprep_backup_20240101_120000.sql
```

## 🚀 Production Deployment

### Connection Pooling
```sql
-- Recommended connection pool settings
max_connections = 200
shared_buffers = 256MB
effective_cache_size = 1GB
work_mem = 4MB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
random_page_cost = 1.1
```

### Monitoring Queries
```sql
-- Active connections
SELECT count(*) FROM pg_stat_activity WHERE state = 'active';

-- Long running queries
SELECT 
    pid,
    now() - pg_stat_activity.query_start AS duration,
    query 
FROM pg_stat_activity 
WHERE (now() - pg_stat_activity.query_start) > interval '5 minutes';

-- Database locks
SELECT 
    blocked_locks.pid AS blocked_pid,
    blocked_activity.usename AS blocked_user,
    blocking_locks.pid AS blocking_pid,
    blocking_activity.usename AS blocking_user,
    blocked_activity.query AS blocked_statement,
    blocking_activity.query AS current_statement_in_blocking_process
FROM pg_catalog.pg_locks blocked_locks
JOIN pg_catalog.pg_stat_activity blocked_activity ON blocked_activity.pid = blocked_locks.pid
JOIN pg_catalog.pg_locks blocking_locks ON blocking_locks.locktype = blocked_locks.locktype
JOIN pg_catalog.pg_stat_activity blocking_activity ON blocking_activity.pid = blocking_locks.pid
WHERE NOT blocked_locks.granted;
```

## 📞 Troubleshooting

### Common Issues

**Connection Issues**
```bash
# Check if PostgreSQL is running
pg_isready -h localhost -p 35432

# Check connection limits
psql -c "SELECT count(*) FROM pg_stat_activity;"
```

**Performance Issues**
```bash
# Analyze slow queries
psql -c "SELECT query, mean_time FROM pg_stat_statements ORDER BY mean_time DESC LIMIT 5;"

# Check for missing indexes
psql -c "SELECT schemaname, tablename, attname FROM pg_stats WHERE n_distinct > 100 AND correlation < 0.1;"
```

**Migration Issues**
```bash
# Check migration status
sqlx migrate info

# Force migration version
sqlx migrate run --target-version 5
```

---

For more detailed information about specific tables or procedures, see the individual migration files in the `migrations/` directory.
