use anyhow::Result;
use std::sync::Arc;
use crate::ai_engine::AIEngine;
use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionData {
    pub calories: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
    pub fiber_g: f64,
    pub sodium_mg: f64,
    pub sugar_g: f64,
}

pub struct NutritionalAnalyzer {
    ai_engine: Arc<AIEngine>,
}

impl NutritionalAnalyzer {
    pub fn new(ai_engine: Arc<AIEngine>) -> Self {
        Self { ai_engine }
    }

    pub async fn analyze_meal(&self, ingredients: &[MealIngredient]) -> Result<BasicNutrition> {
        // Convert MealIngredient to strings for AI analysis
        let ingredient_names: Vec<String> = ingredients
            .iter()
            .map(|ing| format!("{} {} {}", ing.amount, ing.unit, ing.name))
            .collect();
        
        self.ai_engine.analyze_meal_nutrition(&ingredient_names).await
    }

    pub async fn analyze_daily_nutrition(&self, request: &DailyNutritionRequest) -> Result<BasicNutrition> {
        let mut total_nutrition = BasicNutrition::default();
        
        for meal in &request.meals {
            let meal_nutrition = self.analyze_meal(&meal.ingredients).await?;
            
            // Scale by portion size
            let scale_factor = meal.portion_size;
            total_nutrition.calories += meal_nutrition.calories * scale_factor;
            total_nutrition.protein_g += meal_nutrition.protein_g * scale_factor;
            total_nutrition.carbohydrates_g += meal_nutrition.carbohydrates_g * scale_factor;
            total_nutrition.fat_g += meal_nutrition.fat_g * scale_factor;
            total_nutrition.fiber_g += meal_nutrition.fiber_g * scale_factor;
            total_nutrition.sugar_g += meal_nutrition.sugar_g * scale_factor;
            total_nutrition.sodium_mg += meal_nutrition.sodium_mg * scale_factor;
            total_nutrition.cholesterol_mg += meal_nutrition.cholesterol_mg * scale_factor;
        }
        
        Ok(total_nutrition)
    }

    pub async fn calculate_environmental_impact(&self, ingredients: &[MealIngredient]) -> Result<EnvironmentalImpact> {
        let ingredient_names: Vec<String> = ingredients
            .iter()
            .map(|ing| ing.name.clone())
            .collect();
            
        self.ai_engine.calculate_environmental_impact(&ingredient_names).await
    }

    pub async fn get_optimization_suggestions(
        &self,
        current_nutrition: &BasicNutrition,
        goals: &NutritionGoals,
    ) -> Result<Vec<OptimizationSuggestion>> {
        // Convert goals to BasicNutrition for comparison
        let goal_nutrition = BasicNutrition {
            calories: goals.daily_calories,
            protein_g: goals.daily_protein_g,
            carbohydrates_g: goals.daily_carbs_g,
            fat_g: goals.daily_fat_g,
            fiber_g: goals.daily_fiber_g,
            sodium_mg: goals.daily_sodium_mg,
            ..Default::default()
        };
        
        self.ai_engine.get_optimization_suggestions(current_nutrition, &goal_nutrition).await
    }
}
