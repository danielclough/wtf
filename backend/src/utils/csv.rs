use diesel::RunQueryDsl;

use crate::{
    models::recipe::{NewRecipe, Recipe},
    utils::{
        pg::establish_connection_pg,
        uuid::new_random_uuid_v4
    }
};

pub fn parse_csv() -> Result<(), csv::Error> {
    let csv = std::fs::read_to_string("./recipes/recipes.csv").expect("file exists");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: NewRecipe = record?;

        let uuid = new_random_uuid_v4();

        let new_recipe = Recipe {
            id: uuid,
            recipe_name: record.recipe_name.to_string(),
            prep_time: record.prep_time.to_string(),
            cook_time: record.cook_time.to_string(),
            total_time: record.total_time.to_string(),
            servings: record.servings.to_string(),
            total_yield: record.total_yield.to_string(),
            ingredients: record.ingredients.to_string(),
            directions: record.directions.to_string(),
            url: record.url.to_string(),
            rating: record.rating.to_string(),
            cuisine_path: record.cuisine_path.to_string(),
            nutrition: record.nutrition.to_string(),
            timing: record.timing.to_string(),
            img_src: record.img_src.to_string(),
            published: true,
        };

        println!("In {:?}", new_recipe);

        let connection = &mut establish_connection_pg();

        diesel::insert_into(crate::schema::recipes::dsl::recipes)
            .values(&new_recipe)
            .execute(connection)
            .expect("Error saving new recipe");
    }

    Ok(())
}
