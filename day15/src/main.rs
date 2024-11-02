use std::{
    collections::HashMap,
    ops::{Add, Deref, DerefMut, Mul},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let part_1_result = solve_part1(INPUT);
    println!("Part 1: {}", part_1_result);

    let part_2_result = solve_part2(INPUT);
    println!("Part 2: {}", part_2_result);
}

fn solve_part1(input: &str) -> u64 {
    let available_ingredients = AvailableIngredients::from(parse_input(input));
    find_best::<Part1Stragegy>(&available_ingredients)
}

fn find_best<S: Strategy>(available_ingredients: &AvailableIngredients) -> u64 {
    // This holds ingredient name paired with its amounts
    let mut recipe: Recipe = available_ingredients
        .ingredients
        .keys()
        .map(|key| (*key, 0))
        .collect::<Vec<_>>()
        .into();

    // The number of combinations is 100^ingredients
    // We'll use each two digits to represent the amount of each ingredient,
    // e.g. 100234 would be 10 of the first ingredient, 2 of the second, and 34 of the third
    let num_combinations = 100usize.pow(recipe.len() as u32);

    let mut best = 0;

    // We'll then iterate up to the maximum number of combinations
    for i in 0..num_combinations {
        for (index, (_, count)) in recipe.iter_mut().enumerate() {
            // Assign ingredient amount from the digit pairs
            // This is basically a bit shift but using base 100 instead of 2
            *count = (i / 100usize.pow(index as u32) % 100) as u32;
        }

        if !S::accept(available_ingredients, &recipe) {
            continue;
        }

        best = best.max(available_ingredients.calculate_score(&recipe));
    }

    best as u64
}

fn parse_input(input: &str) -> impl Iterator<Item = Ingredient> {
    input.lines().map(Ingredient::from)
}

#[derive(Debug, PartialEq, Eq)]
struct Ingredient<'a> {
    name: &'a str,
    properties: IngredientProperties,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
struct IngredientProperties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl<'a> From<&'a str> for Ingredient<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rest) = value.split_once(": capacity ").unwrap();
        let (capacity, rest) = rest.split_once(", durability ").unwrap();
        let (durability, rest) = rest.split_once(", flavor ").unwrap();
        let (flavor, rest) = rest.split_once(", texture ").unwrap();
        let (texture, calories) = rest.split_once(", calories ").unwrap();

        let capacity = capacity.parse::<i32>().unwrap();
        let durability = durability.parse::<i32>().unwrap();
        let flavor = flavor.parse::<i32>().unwrap();
        let texture = texture.parse::<i32>().unwrap();
        let calories = calories.parse::<i32>().unwrap();

        Self {
            name,
            properties: IngredientProperties {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            },
        }
    }
}

impl Mul<i32> for IngredientProperties {
    type Output = IngredientProperties;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.capacity *= rhs;
        self.durability *= rhs;
        self.flavor *= rhs;
        self.texture *= rhs;
        self.calories *= rhs;
        self
    }
}

impl Mul<i32> for &IngredientProperties {
    type Output = IngredientProperties;

    fn mul(self, rhs: i32) -> Self::Output {
        *self * rhs
    }
}

impl Add<IngredientProperties> for IngredientProperties {
    type Output = IngredientProperties;

    fn add(self, rhs: IngredientProperties) -> Self::Output {
        Self::Output {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

struct AvailableIngredients<'a> {
    ingredients: HashMap<&'a str, IngredientProperties>,
}

impl<'a, I> From<I> for AvailableIngredients<'a>
where
    I: IntoIterator<Item = Ingredient<'a>>,
{
    fn from(value: I) -> Self {
        let ingredients = value
            .into_iter()
            .map(|ingredient| (ingredient.name, ingredient.properties))
            .collect();

        Self { ingredients }
    }
}

struct Recipe<'a>(Vec<(&'a str, u32)>);

impl<'a> Deref for Recipe<'a> {
    type Target = Vec<(&'a str, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Recipe<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Vec<(&'a str, u32)>> for Recipe<'a> {
    fn from(value: Vec<(&'a str, u32)>) -> Self {
        Self(value)
    }
}

impl AvailableIngredients<'_> {
    fn calculate_properties(&self, recipe: &Recipe) -> IngredientProperties {
        let mut properties = IngredientProperties::default();

        for (ingredient, amount) in recipe.iter() {
            let ingredient_properties = self.ingredients.get(ingredient).unwrap();
            properties = properties + ingredient_properties * *amount as i32;
        }

        properties
    }

    fn calculate_score(&self, recipe: &Recipe) -> i32 {
        let properties = self.calculate_properties(recipe);

        properties.capacity.max(0)
            * properties.durability.max(0)
            * properties.flavor.max(0)
            * properties.texture.max(0)
    }

    fn calculate_calories(&self, recipe: &Recipe) -> i32 {
        let properties = self.calculate_properties(recipe);

        properties.calories.max(0)
    }
}

trait Strategy {
    fn accept(available_ingredients: &AvailableIngredients, recipt: &Recipe) -> bool;
}

struct Part1Stragegy;

impl Strategy for Part1Stragegy {
    fn accept(_: &AvailableIngredients, recipe: &Recipe) -> bool {
        recipe.iter().map(|(_, count)| count).sum::<u32>() == 100u32
    }
}

fn solve_part2(input: &str) -> u64 {
    let available_ingredients = AvailableIngredients::from(parse_input(input));
    find_best::<Part2Stragegy>(&available_ingredients)
}

struct Part2Stragegy;

impl Strategy for Part2Stragegy {
    fn accept(available_ingredients: &AvailableIngredients, recipe: &Recipe) -> bool {
        Part1Stragegy::accept(available_ingredients, recipe)
            && available_ingredients.calculate_calories(recipe) == 500
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredient_parser() {
        for (input, expected) in [
            (
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
                Ingredient {
                    name: "Butterscotch",
                    properties: IngredientProperties {
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8,
                    },
                },
            ),
            (
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
                Ingredient {
                    name: "Cinnamon",
                    properties: IngredientProperties {
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3,
                    },
                },
            ),
        ] {
            assert_eq!(Ingredient::from(input), expected);
        }
    }

    #[test]
    fn test_part1() {
        let available_ingredients = AvailableIngredients::from([
            Ingredient {
                name: "Butterscotch",
                properties: IngredientProperties {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                },
            },
            Ingredient {
                name: "Cinnamon",
                properties: IngredientProperties {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3,
                },
            },
        ]);

        let recipe = vec![("Butterscotch", 44), ("Cinnamon", 56)].into();
        let score = available_ingredients.calculate_score(&recipe);

        assert_eq!(score, 62842880);
    }
}
