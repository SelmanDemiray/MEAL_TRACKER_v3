use anyhow::{anyhow, Result};
use git2::Repository;
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::path::Path;
use tracing::{info, warn, error, debug};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::models::{ImportBatch, ImportRequest, ImportStatus, Recipe, RecipeFile, FileFormat, ParsedRecipe};

pub struct RecipeImporter {
    temp_dir: String,
}

impl RecipeImporter {
    pub fn new() -> Self {
        Self {
            temp_dir: "/tmp/recipe_imports".to_string(),
        }
    }

    pub async fn import_from_repository(&self, request: ImportRequest) -> Result<ImportBatch> {
        info!("Starting import from repository: {}", request.repository_url);
        
        let batch_id = Uuid::new_v4();
        let mut batch = ImportBatch {
            id: batch_id,
            repository_url: request.repository_url.clone(),
            import_status: ImportStatus::InProgress,
            total_recipes: 0,
            successful_imports: 0,
            failed_imports: 0,
            error_log: Vec::new(),
            started_at: chrono::Utc::now(),
            completed_at: None,
            created_by: None,
        };

        // Clone repository
        let repo_path = format!("{}/{}", self.temp_dir, batch_id);
        
        match self.clone_repository(&request.repository_url, &repo_path).await {
            Ok(_) => {
                info!("Repository cloned successfully to {}", repo_path);
            }
            Err(e) => {
                error!("Failed to clone repository: {}", e);
                batch.import_status = ImportStatus::Failed;
                batch.error_log.push(format!("Repository clone failed: {}", e));
                batch.completed_at = Some(chrono::Utc::now());
                return Ok(batch);
            }
        }

        // Scan for recipe files
        let recipe_files = match self.scan_recipe_files(&repo_path).await {
            Ok(files) => files,
            Err(e) => {
                error!("Failed to scan recipe files: {}", e);
                batch.import_status = ImportStatus::Failed;
                batch.error_log.push(format!("File scanning failed: {}", e));
                batch.completed_at = Some(chrono::Utc::now());
                return Ok(batch);
            }
        };

        batch.total_recipes = recipe_files.len() as i32;
        info!("Found {} potential recipe files", batch.total_recipes);

        // Parse and process recipes
        for recipe_file in recipe_files {
            match self.parse_recipe_file(&recipe_file).await {
                Ok(parsed_recipe) => {
                    // Convert parsed recipe to Recipe model
                    match self.convert_to_recipe(parsed_recipe, &request.repository_url).await {
                        Ok(_recipe) => {
                            batch.successful_imports += 1;
                            debug!("Successfully imported recipe from {}", recipe_file.file_path);
                        }
                        Err(e) => {
                            batch.failed_imports += 1;
                            batch.error_log.push(format!("Failed to convert {}: {}", recipe_file.file_path, e));
                            warn!("Failed to convert recipe from {}: {}", recipe_file.file_path, e);
                        }
                    }
                }
                Err(e) => {
                    batch.failed_imports += 1;
                    batch.error_log.push(format!("Failed to parse {}: {}", recipe_file.file_path, e));
                    warn!("Failed to parse recipe file {}: {}", recipe_file.file_path, e);
                }
            }
        }

        // Clean up temporary directory
        if let Err(e) = fs::remove_dir_all(&repo_path) {
            warn!("Failed to clean up temporary directory {}: {}", repo_path, e);
        }

        batch.import_status = ImportStatus::Completed;
        batch.completed_at = Some(chrono::Utc::now());
        
        info!(
            "Import completed. Success: {}, Failed: {}", 
            batch.successful_imports, 
            batch.failed_imports
        );

        Ok(batch)
    }

    async fn clone_repository(&self, url: &str, target_path: &str) -> Result<()> {
        // Create temp directory if it doesn't exist
        fs::create_dir_all(&self.temp_dir)?;
        
        // Clone the repository
        Repository::clone(url, target_path)
            .map_err(|e| anyhow!("Git clone failed: {}", e))?;
        
        Ok(())
    }

    async fn scan_recipe_files(&self, repo_path: &str) -> Result<Vec<RecipeFile>> {
        let mut recipe_files = Vec::new();
        
        // Define file extensions that might contain recipes
        let recipe_extensions = vec!["json", "yaml", "yml", "md", "txt"];
        
        for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if recipe_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            match self.read_file_content(path) {
                                Ok(content) => {
                                    let format = match ext_str.to_lowercase().as_str() {
                                        "json" => FileFormat::Json,
                                        "yaml" | "yml" => FileFormat::Yaml,
                                        "md" => FileFormat::Markdown,
                                        _ => FileFormat::PlainText,
                                    };
                                    
                                    recipe_files.push(RecipeFile {
                                        file_path: path.to_string_lossy().to_string(),
                                        content,
                                        format,
                                    });
                                }
                                Err(e) => {
                                    warn!("Failed to read file {}: {}", path.display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(recipe_files)
    }

    fn read_file_content(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .map_err(|e| anyhow!("Failed to read file {}: {}", path.display(), e))
    }

    async fn parse_recipe_file(&self, recipe_file: &RecipeFile) -> Result<ParsedRecipe> {
        match recipe_file.format {
            FileFormat::Json => self.parse_json_recipe(recipe_file),
            FileFormat::Yaml => self.parse_yaml_recipe(recipe_file),
            FileFormat::Markdown => self.parse_markdown_recipe(recipe_file),
            FileFormat::PlainText => self.parse_text_recipe(recipe_file),
        }
    }

    fn parse_json_recipe(&self, recipe_file: &RecipeFile) -> Result<ParsedRecipe> {
        let json: Value = serde_json::from_str(&recipe_file.content)?;
        
        let name = json.get("name")
            .or_else(|| json.get("title"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown Recipe")
            .to_string();
            
        let description = json.get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let ingredients = json.get("ingredients")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        let instructions = json.get("instructions")
            .or_else(|| json.get("directions"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        Ok(ParsedRecipe {
            name,
            description,
            prep_time: json.get("prep_time").and_then(|v| v.as_str()).map(|s| s.to_string()),
            cook_time: json.get("cook_time").and_then(|v| v.as_str()).map(|s| s.to_string()),
            servings: json.get("servings").and_then(|v| v.as_str()).map(|s| s.to_string()),
            ingredients,
            instructions,
            tags: json.get("tags").and_then(|v| v.as_array()).map(|arr| {
                arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect()
            }).unwrap_or_default(),
            source_file: recipe_file.file_path.clone(),
        })
    }

    fn parse_yaml_recipe(&self, _recipe_file: &RecipeFile) -> Result<ParsedRecipe> {
        // TODO: Implement YAML parsing
        Err(anyhow!("YAML parsing not yet implemented"))
    }

    fn parse_markdown_recipe(&self, recipe_file: &RecipeFile) -> Result<ParsedRecipe> {
        let content = &recipe_file.content;
        
        // Extract title from first heading
        let title_regex = Regex::new(r"^#\s*(.+)$").unwrap();
        let name = title_regex
            .captures_iter(content)
            .next()
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "Unknown Recipe".to_string());
        
        // Extract ingredients section
        let ingredients = self.extract_markdown_section(content, "ingredients");
        
        // Extract instructions/directions section
        let instructions = self.extract_markdown_section(content, "instructions")
            .or_else(|| self.extract_markdown_section(content, "directions"))
            .unwrap_or_default();
        
        Ok(ParsedRecipe {
            name,
            description: None,
            prep_time: None,
            cook_time: None,
            servings: None,
            ingredients: ingredients.unwrap_or_default(),
            instructions,
            tags: Vec::new(),
            source_file: recipe_file.file_path.clone(),
        })
    }

    fn parse_text_recipe(&self, _recipe_file: &RecipeFile) -> Result<ParsedRecipe> {
        // TODO: Implement text parsing with basic heuristics
        Err(anyhow!("Plain text parsing not yet implemented"))
    }

    fn extract_markdown_section(&self, content: &str, section_name: &str) -> Option<Vec<String>> {
        let section_regex = Regex::new(&format!(r"(?i)##?\s*{}\s*\n(.*?)(?=\n##|\n#|$)", section_name)).unwrap();
        
        if let Some(captures) = section_regex.captures(content) {
            if let Some(section_content) = captures.get(1) {
                let items: Vec<String> = section_content
                    .as_str()
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .filter(|line| line.starts_with('-') || line.starts_with('*') || line.chars().next().unwrap_or(' ').is_numeric())
                    .map(|line| {
                        // Remove markdown list markers
                        line.trim_start_matches('-')
                            .trim_start_matches('*')
                            .trim_start_matches(|c: char| c.is_numeric() || c == '.')
                            .trim()
                            .to_string()
                    })
                    .filter(|line| !line.is_empty())
                    .collect();
                
                if !items.is_empty() {
                    return Some(items);
                }
            }
        }
        
        None
    }

    async fn convert_to_recipe(&self, parsed: ParsedRecipe, repository_url: &str) -> Result<Recipe> {
        let prep_time = self.parse_time_string(parsed.prep_time.as_deref());
        let cook_time = self.parse_time_string(parsed.cook_time.as_deref());
        
        Ok(Recipe {
            id: Uuid::new_v4(),
            name: parsed.name,
            description: parsed.description,
            prep_time_minutes: prep_time,
            cook_time_minutes: cook_time,
            total_time_minutes: prep_time.and_then(|p| cook_time.map(|c| p + c)),
            servings: parsed.servings.and_then(|s| s.parse().ok()),
            tags: if parsed.tags.is_empty() { None } else { Some(parsed.tags) },
            source_repository: Some(repository_url.to_string()),
            original_filename: Some(parsed.source_file),
            ingredients: if parsed.ingredients.is_empty() { None } else { Some(parsed.ingredients) },
            directions: if parsed.instructions.is_empty() { None } else { Some(parsed.instructions) },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    fn parse_time_string(&self, time_str: Option<&str>) -> Option<i32> {
        time_str.and_then(|s| {
            let time_regex = Regex::new(r"(\d+)").unwrap();
            time_regex.captures(s)
                .and_then(|cap| cap.get(1))
                .and_then(|m| m.as_str().parse().ok())
        })
    }
}

impl Default for RecipeImporter {
    fn default() -> Self {
        Self::new()
    }
}
