use anyhow::Result;
use std::collections::HashMap;
use crate::{MealIngredient, BasicNutrition, Micronutrient, DietaryCompliance, EnvironmentalImpact};
use crate::models::NutritionAnalysisInternal;

pub struct NutritionalAnalyzer {
    nutrition_database: HashMap<String, IngredientNutrition>,
}

#[derive(Debug, Clone)]
struct IngredientNutrition {
    calories_per_100g: f32,
    protein_per_100g: f32,
    carbs_per_100g: f32,
    fat_per_100g: f32,
    fiber_per_100g: f32,
    sugar_per_100g: f32,
    sodium_per_100g: f32,
    micronutrients: Vec<MicronutrientData>,
    allergens: Vec<String>,
    dietary_flags: DietaryFlags,
    carbon_footprint_per_100g: f32,
    water_usage_per_100g: f32,
}

#[derive(Debug, Clone)]
struct MicronutrientData {
    name: String,
    amount_per_100g: f32,
    unit: String,
    daily_value: f32,
    bioavailability_factor: f32,
}

#[derive(Debug, Clone)]
struct DietaryFlags {
    vegan: bool,
    vegetarian: bool,
    gluten_free: bool,
    dairy_free: bool,
    low_sodium: bool,
    anti_inflammatory_score: f32,
}

impl NutritionalAnalyzer {
    pub fn new() -> Self {
        let mut nutrition_database = HashMap::new();
        
        // Initialize comprehensive nutrition database
        nutrition_database.insert("chicken breast".to_string(), IngredientNutrition {
            calories_per_100g: 165.0,
            protein_per_100g: 31.0,
            carbs_per_100g: 0.0,
            fat_per_100g: 3.6,
            fiber_per_100g: 0.0,
            sugar_per_100g: 0.0,
            sodium_per_100g: 74.0,
            micronutrients: vec![
                MicronutrientData {
                    name: "Vitamin B6".to_string(),
                    amount_per_100g: 0.89,
                    unit: "mg".to_string(),
                    daily_value: 1.3,
                    bioavailability_factor: 0.9,
                },
            ],
            allergens: vec![],
            dietary_flags: DietaryFlags {
                vegan: false,
                vegetarian: false,
                gluten_free: true,
                dairy_free: true,
                low_sodium: true,
                anti_inflammatory_score: 7.5,
            },
            carbon_footprint_per_100g: 6.1,
            water_usage_per_100g: 4325.0,
        });

        nutrition_database.insert("brown rice".to_string(), IngredientNutrition {
            calories_per_100g: 112.0,
            protein_per_100g: 2.6,
            carbs_per_100g: 23.0,
            fat_per_100g: 0.9,
            fiber_per_100g: 1.8,
            sugar_per_100g: 0.4,
            sodium_per_100g: 5.0,
            micronutrients: vec![
                MicronutrientData {
                    name: "Magnesium".to_string(),
                    amount_per_100g: 43.0,
                    unit: "mg".to_string(),
                    daily_value: 400.0,
                    bioavailability_factor: 0.8,
                },
            ],
            allergens: vec![],
            dietary_flags: DietaryFlags {
                vegan: true,
                vegetarian: true,
                gluten_free: true,
                dairy_free: true,
                low_sodium: true,
                anti_inflammatory_score: 8.2,
            },
            carbon_footprint_per_100g: 2.3,
            water_usage_per_100g: 2497.0,
        });

        // Add more ingredients...
        Self::add_common_ingredients(&mut nutrition_database);

        Self { nutrition_database }
    }

    pub async fn analyze_meal(
        &self,
        ingredients: &[MealIngredient],
        portion_size: f32,
        cooking_method: Option<&str>,
    ) -> Result<NutritionAnalysisInternal> {
        let mut total_nutrition = BasicNutrition {
            calories: 0.0,
            protein: 0.0,
            carbohydrates: 0.0,
            fat: 0.0,
            fiber: 0.0,
            sugar: 0.0,
            sodium: 0.0,
        };

        let mut micronutrients: HashMap<String, f32> = HashMap::new();
        let mut allergens: Vec<String> = Vec::new();
        let mut dietary_compliance = DietaryCompliance {
            keto_friendly: true,
            vegan: true,
            vegetarian: true,
            gluten_free: true,
            dairy_free: true,
            low_sodium: true,
            anti_inflammatory_score: 0.0,
        };

        let mut total_carbon_footprint = 0.0;
        let mut total_water_usage = 0.0;
        let mut anti_inflammatory_scores = Vec::new();

        for ingredient in ingredients {
            let ingredient_nutrition = self.get_ingredient_nutrition(&ingredient.name);
            let amount_factor = self.calculate_amount_factor(&ingredient.amount, &ingredient.unit);
            let cooking_factor = self.get_cooking_factor(cooking_method, &ingredient.name);

            // Calculate nutrition with cooking adjustments
            total_nutrition.calories += ingredient_nutrition.calories_per_100g * amount_factor * cooking_factor.calories;
            total_nutrition.protein += ingredient_nutrition.protein_per_100g * amount_factor * cooking_factor.protein;
            total_nutrition.carbohydrates += ingredient_nutrition.carbs_per_100g * amount_factor;
            total_nutrition.fat += ingredient_nutrition.fat_per_100g * amount_factor * cooking_factor.fat;
            total_nutrition.fiber += ingredient_nutrition.fiber_per_100g * amount_factor * cooking_factor.fiber;
            total_nutrition.sugar += ingredient_nutrition.sugar_per_100g * amount_factor;
            total_nutrition.sodium += ingredient_nutrition.sodium_per_100g * amount_factor;

            // Process micronutrients
            for micro in &ingredient_nutrition.micronutrients {
                let amount = micro.amount_per_100g * amount_factor * micro.bioavailability_factor * cooking_factor.micronutrients;
                *micronutrients.entry(micro.name.clone()).or_insert(0.0) += amount;
            }

            // Aggregate allergens
            for allergen in &ingredient_nutrition.allergens {
                if !allergens.contains(allergen) {
                    allergens.push(allergen.clone());
                }
            }

            // Update dietary compliance
            dietary_compliance.vegan &= ingredient_nutrition.dietary_flags.vegan;
            dietary_compliance.vegetarian &= ingredient_nutrition.dietary_flags.vegetarian;
            dietary_compliance.gluten_free &= ingredient_nutrition.dietary_flags.gluten_free;
            dietary_compliance.dairy_free &= ingredient_nutrition.dietary_flags.dairy_free;

            // Environmental impact
            total_carbon_footprint += ingredient_nutrition.carbon_footprint_per_100g * amount_factor;
            total_water_usage += ingredient_nutrition.water_usage_per_100g * amount_factor;
            anti_inflammatory_scores.push(ingredient_nutrition.dietary_flags.anti_inflammatory_score);
        }

        // Apply portion size
        total_nutrition.calories *= portion_size;
        total_nutrition.protein *= portion_size;
        total_nutrition.carbohydrates *= portion_size;
        total_nutrition.fat *= portion_size;
        total_nutrition.fiber *= portion_size;
        total_nutrition.sugar *= portion_size;
        total_nutrition.sodium *= portion_size;

        // Determine keto-friendliness
        dietary_compliance.keto_friendly = total_nutrition.carbohydrates < 20.0;
        dietary_compliance.low_sodium = total_nutrition.sodium < 600.0;
        dietary_compliance.anti_inflammatory_score = anti_inflammatory_scores.iter().sum::<f32>() / anti_inflammatory_scores.len() as f32;

        // Convert micronutrients to final format
        let final_micronutrients: Vec<Micronutrient> = micronutrients
            .into_iter()
            .map(|(name, amount)| Micronutrient {
                name: name.clone(),
                amount: amount * portion_size,
                unit: self.get_nutrient_unit(&name),
                daily_value_percentage: (amount * portion_size / self.get_daily_value(&name)) * 100.0,
            })
            .collect();

        Ok(NutritionAnalysisInternal {
            basic_nutrition: total_nutrition,
            micronutrients: final_micronutrients,
            dietary_compliance,
            environmental_impact: EnvironmentalImpact {
                carbon_footprint: total_carbon_footprint * portion_size,
                water_usage: total_water_usage * portion_size,
                sustainability_score: self.calculate_sustainability_score(total_carbon_footprint, total_water_usage),
            },
        })
    }

    fn get_ingredient_nutrition(&self, ingredient_name: &str) -> IngredientNutrition {
        // Normalize ingredient name for lookup
        let normalized = ingredient_name.to_lowercase().trim().to_string();
        
        self.nutrition_database
            .get(&normalized)
            .cloned()
            .unwrap_or_else(|| self.get_default_nutrition())
    }

    fn get_default_nutrition(&self) -> IngredientNutrition {
        IngredientNutrition {
            calories_per_100g: 50.0,
            protein_per_100g: 2.0,
            carbs_per_100g: 10.0,
            fat_per_100g: 1.0,
            fiber_per_100g: 2.0,
            sugar_per_100g: 2.0,
            sodium_per_100g: 10.0,
            micronutrients: vec![],
            allergens: vec![],
            dietary_flags: DietaryFlags {
                vegan: true,
                vegetarian: true,
                gluten_free: true,
                dairy_free: true,
                low_sodium: true,
                anti_inflammatory_score: 5.0,
            },
            carbon_footprint_per_100g: 1.0,
            water_usage_per_100g: 500.0,
        }
    }

    fn calculate_amount_factor(&self, amount: &f32, unit: &str) -> f32 {
        match unit.to_lowercase().as_str() {
            "g" | "grams" => amount / 100.0,
            "kg" | "kilograms" => amount * 10.0,
            "oz" | "ounces" => amount * 0.2835,
            "lb" | "pounds" => amount * 4.536,
            "cup" | "cups" => amount * 2.4, // Approximate
            "tbsp" | "tablespoon" => amount * 0.15,
            "tsp" | "teaspoon" => amount * 0.05,
            _ => amount / 100.0, // Default to grams
        }
    }

    fn get_cooking_factor(&self, cooking_method: Option<&str>, _ingredient: &str) -> CookingFactor {
        match cooking_method {
            Some("grilled") => CookingFactor {
                calories: 1.0,
                protein: 0.95,
                fat: 0.9,
                fiber: 1.0,
                micronutrients: 0.85,
            },
            Some("boiled") => CookingFactor {
                calories: 1.0,
                protein: 0.9,
                fat: 1.0,
                fiber: 0.9,
                micronutrients: 0.7,
            },
            Some("steamed") => CookingFactor {
                calories: 1.0,
                protein: 0.98,
                fat: 1.0,
                fiber: 0.95,
                micronutrients: 0.9,
            },
            _ => CookingFactor {
                calories: 1.0,
                protein: 1.0,
                fat: 1.0,
                fiber: 1.0,
                micronutrients: 1.0,
            },
        }
    }

    fn get_daily_value(&self, nutrient_name: &str) -> f32 {
        match nutrient_name.to_lowercase().as_str() {
            "vitamin c" => 90.0,
            "vitamin d" => 20.0,
            "vitamin b6" => 1.3,
            "calcium" => 1000.0,
            "iron" => 18.0,
            "magnesium" => 400.0,
            "zinc" => 11.0,
            _ => 100.0, // Default
        }
    }

    fn get_nutrient_unit(&self, nutrient_name: &str) -> String {
        match nutrient_name.to_lowercase().as_str() {
            "vitamin c" | "vitamin b6" | "calcium" | "iron" | "magnesium" | "zinc" => "mg".to_string(),
            "vitamin d" => "mcg".to_string(),
            _ => "mg".to_string(),
        }
    }

    fn calculate_sustainability_score(&self, carbon_footprint: f32, water_usage: f32) -> f32 {
        // Calculate sustainability score (0-10 scale, higher is better)
        let carbon_score = (20.0 - carbon_footprint.min(20.0)) / 2.0;
        let water_score = (10000.0 - water_usage.min(10000.0)) / 1000.0;
        ((carbon_score + water_score) / 2.0).max(0.0).min(10.0)
    }

    fn add_common_ingredients(database: &mut HashMap<String, IngredientNutrition>) {
        // Add more common ingredients to the database
        database.insert("spinach".to_string(), IngredientNutrition {
            calories_per_100g: 23.0,
            protein_per_100g: 2.9,
            carbs_per_100g: 3.6,
            fat_per_100g: 0.4,
            fiber_per_100g: 2.2,
            sugar_per_100g: 0.4,
            sodium_per_100g: 79.0,
            micronutrients: vec![],
            allergens: vec![],
            dietary_flags: DietaryFlags {
                vegan: true,
                vegetarian: true,
                gluten_free: true,
                dairy_free: true,
                low_sodium: true,
                anti_inflammatory_score: 9.0,
            },
            carbon_footprint_per_100g: 0.7,
            water_usage_per_100g: 322.0,
        });

        // Add more ingredients as needed...
    }
}

#[derive(Debug, Clone)]
struct CookingFactor {
    calories: f32,
    protein: f32,
    fat: f32,
    fiber: f32,
    micronutrients: f32,
}
