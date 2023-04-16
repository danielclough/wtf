use rocket::{launch, routes};
pub mod models;
pub mod schema;
pub mod services;
pub mod utils;

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = std::env::args().collect();

    if &args.len() > &1 {
        if args[1] == "init" {
            // let csv = utils::csv::parse_csv();
            println!("{:?}", "csv?");
        }
    }

    rocket::build()

        .mount("/account", routes![services::account::list])
        .mount("/account", routes![services::account::create])

        .mount("/conduct_code", routes![services::conduct_code::list])
        .mount("/conduct_code", routes![services::conduct_code::create])

        .mount("/event", routes![services::event::list])
        .mount("/event", routes![services::event::create])

        .mount("/login", routes![services::login::list])
        .mount("/login", routes![services::login::create])
        .mount("/login", routes![services::login::find_by_id])
        .mount("/login", routes![services::login::find_by_email])
        .mount("/login", routes![services::login::delete])

        .mount("/preference", routes![services::preference::list])
        .mount("/preference", routes![services::preference::create])

        .mount("/role", routes![services::role::list])
        .mount("/role", routes![services::role::create])

        .mount("/sensitivities", routes![services::sensitivities::list])
        .mount("/sensitivities", routes![services::sensitivities::create])

        .mount("/survey_result", routes![services::survey_result::list])
        .mount("/survey_result", routes![services::survey_result::create])        

        .mount("/user", routes![services::user::create])
        .mount("/user", routes![services::user::list])
}