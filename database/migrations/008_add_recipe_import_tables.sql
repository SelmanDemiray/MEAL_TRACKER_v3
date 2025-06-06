-- Add columns to recipes table for imported data
ALTER TABLE recipes ADD COLUMN IF NOT EXISTS source_repository VARCHAR(255);
ALTER TABLE recipes ADD COLUMN IF NOT EXISTS original_filename VARCHAR(255);
ALTER TABLE recipes ADD COLUMN IF NOT EXISTS import_batch_id UUID;

-- Create recipe import tracking table
CREATE TABLE IF NOT EXISTS recipe_import_batches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_url VARCHAR(255) NOT NULL,
    import_status VARCHAR(50) DEFAULT 'pending',
    total_recipes INTEGER DEFAULT 0,
    successful_imports INTEGER DEFAULT 0,
    failed_imports INTEGER DEFAULT 0,
    error_log TEXT[],
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id)
);

-- Update recipe_ingredients table to handle string-based ingredients from JSON
ALTER TABLE recipe_ingredients ADD COLUMN IF NOT EXISTS ingredient_text TEXT;
ALTER TABLE recipe_ingredients ALTER COLUMN ingredient_id DROP NOT NULL;

-- Update recipe_steps to handle JSON directions
ALTER TABLE recipe_steps ADD COLUMN IF NOT EXISTS step_text TEXT;
ALTER TABLE recipe_steps ADD COLUMN IF NOT EXISTS step_order INTEGER;

-- Create indexes for better search performance
CREATE INDEX IF NOT EXISTS idx_recipes_source_repository ON recipes(source_repository);
CREATE INDEX IF NOT EXISTS idx_recipes_tags_gin ON recipes USING GIN(tags);
CREATE INDEX IF NOT EXISTS idx_recipes_fulltext ON recipes USING GIN(to_tsvector('english', name || ' ' || COALESCE(description, '')));

-- Create function to search recipes
CREATE OR REPLACE FUNCTION search_recipes(
    search_term TEXT,
    tag_filter TEXT[] DEFAULT NULL,
    limit_count INTEGER DEFAULT 50
) RETURNS TABLE (
    id UUID,
    name VARCHAR(255),
    description TEXT,
    prep_time_minutes INTEGER,
    cook_time_minutes INTEGER,
    total_time_minutes INTEGER,
    servings INTEGER,
    tags TEXT[],
    source_repository VARCHAR(255),
    similarity_score REAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        r.id,
        r.name,
        r.description,
        r.prep_time_minutes,
        r.cook_time_minutes,
        r.total_time_minutes,
        r.servings,
        r.tags,
        r.source_repository,
        ts_rank(to_tsvector('english', r.name || ' ' || COALESCE(r.description, '')), plainto_tsquery('english', search_term)) as similarity_score
    FROM recipes r
    WHERE 
        (search_term IS NULL OR to_tsvector('english', r.name || ' ' || COALESCE(r.description, '')) @@ plainto_tsquery('english', search_term))
        AND (tag_filter IS NULL OR r.tags && tag_filter)
        AND r.is_public = true
    ORDER BY 
        CASE WHEN search_term IS NOT NULL THEN ts_rank(to_tsvector('english', r.name || ' ' || COALESCE(r.description, '')), plainto_tsquery('english', search_term)) ELSE 0 END DESC,
        r.rating DESC,
        r.created_at DESC
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;
