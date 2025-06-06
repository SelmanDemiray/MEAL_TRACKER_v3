use crate::models::*;
use crate::database::DatabaseService;
use anyhow::Result;

#[derive(Debug)]
pub struct NutritionalAnalyzer {
    db: DatabaseService,
}

impl NutritionalAnalyzer {
    pub fn new(db: DatabaseService) -> Self {
        Self { db }
    }

    pub async fn analyze_meal(
        &self,
        ingredients: &[MealIngredient],
        portion_size: f32,
        cooking_method: Option<&str>,
    ) -> Result<NutritionAnalysis> {
        let mut total_nutrition = BasicNutrition::default();
        let mut micronutrients = Vec::new();
        let mut optimization_suggestions = Vec::new();

        // Process each ingredient
        for ingredient in ingredients {
            if let Some(nutrition_data) = self.db.get_nutrition_data(&ingredient.name).await? {
                // Calculate nutrition based on amount
                let factor = (ingredient.amount / 100.0) * portion_size;
                
                total_nutrition.calories += nutrition_data.calories_per_100g * factor;
                total_nutrition.protein += nutrition_data.protein_g * factor;
                total_nutrition.carbohydrates += nutrition_data.carbs_g * factor;
                total_nutrition.fat += nutrition_data.fat_g * factor;
                total_nutrition.fiber += nutrition_data.fiber_g * factor;

                // Add micronutrients (simplified example)
                micronutrients.push(Micronutrient {
                    name: "Vitamin C".to_string(),
                    amount: 15.0 * factor,
                    unit: "mg".to_string(),
                    daily_value_percentage: 20.0,
                    bioavailability: 0.85,
                });
            }
        }

        // Apply cooking method adjustments
        if let Some(method) = cooking_method {
            self.apply_cooking_adjustments(&mut total_nutrition, &mut micronutrients, method);
        }

        // Generate health score
        let health_score = self.calculate_health_score(&total_nutrition, &micronutrients).await?;

        // Generate optimization suggestions
        if total_nutrition.protein < 20.0 {
            optimization_suggestions.push(OptimizationSuggestion {
                category: "protein".to_string(),
                suggestion: "Consider adding a protein source like chicken, fish, or legumes".to_string(),
                impact: "medium".to_string(),
                difficulty: "easy".to_string(),
                estimated_improvement: 15.0,
            });
        }

        Ok(NutritionAnalysis {
            basic_nutrition: total_nutrition.clone(),
            micronutrients,
            health_score,
            dietary_compliance: DietaryCompliance {
                keto_friendly: total_nutrition.carbohydrates < 20.0,
                vegan: true, // Would need to check ingredients
                gluten_free: true, // Would need to check ingredients
                anti_inflammatory_score: 7.5,
                vegetarian: true,
                dairy_free: true,
                low_sodium: total_nutrition.sodium < 600.0,
            },
            optimization_suggestions,
            environmental_impact: EnvironmentalImpact {
                carbon_footprint: 5.2,
                water_usage: 1500.0,
                sustainability_score: 7.8,
            },
        })
    }

    fn apply_cooking_adjustments(
        &self,
        nutrition: &mut BasicNutrition,
        micronutrients: &mut Vec<Micronutrient>,
        cooking_method: &str,
    ) {
        match cooking_method {
            "grilled" | "baked" => {
                // Minimal nutrient loss
                for micronutrient in micronutrients.iter_mut() {
                    micronutrient.amount *= 0.95; // 5% loss
                }
            }
            "boiled" | "steamed" => {
                // Moderate nutrient loss for water-soluble vitamins
                for micronutrient in micronutrients.iter_mut() {
                    if micronutrient.name.contains("Vitamin C") || micronutrient.name.contains("B") {
                        micronutrient.amount *= 0.80; // 20% loss
                    }
                }
            }
            "fried" => {
                // Add calories from oil, reduce some nutrients
                nutrition.calories += 50.0;
                nutrition.fat += 5.0;
                for micronutrient in micronutrients.iter_mut() {
                    micronutrient.amount *= 0.90; // 10% loss
                }
            }
            _ => {} // No adjustments for unknown methods
        }
    }

    async fn calculate_health_score(
        &self,
        nutrition: &BasicNutrition,
        micronutrients: &Vec<Micronutrient>,
    ) -> Result<f32> {
        // Remove unused variable warning
        let _micronutrient_count = micronutrients.len() as f32;
        
        let mut score: f32 = 50.0; // Base score

        // Protein score (20-40g is good for a meal)
        if nutrition.protein >= 20.0 && nutrition.protein <= 40.0 {
            score += 20.0;
        } else if nutrition.protein >= 15.0 && nutrition.protein <= 50.0 {
            score += 10.0;
        }

        // Fiber score (5-15g is good for a meal)
        if nutrition.fiber >= 5.0 && nutrition.fiber <= 15.0 {
            score += 15.0;
        } else if nutrition.fiber >= 3.0 {
            score += 8.0;
        }

        // Calorie appropriateness (assuming 400-800 cal per meal)
        if nutrition.calories >= 400.0 && nutrition.calories <= 800.0 {
            score += 15.0;
        } else if nutrition.calories >= 300.0 && nutrition.calories <= 1000.0 {
            score += 8.0;
        }

        Ok(score.min(100.0).max(0.0))
    }

    pub async fn health_check(&self) -> Result<()> {
        self.db.health_check().await
    }

    pub async fn quick_performance_test(&self) -> Result<()> {
        // Basic performance test
        let test_ingredients = vec![
            MealIngredient {
                ingredient_id: uuid::Uuid::new_v4(),
                name: "chicken breast".to_string(),
                amount: 100.0,
                unit: "g".to_string(),
                preparation: None,
            }
        ];

        let _result = self.analyze_meal(&test_ingredients, 1.0, None).await?;
        Ok(())
    }
}
