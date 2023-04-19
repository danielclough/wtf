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
        .mount("/account", routes![services::account::find_by_id])
        .mount("/account", routes![services::account::delete])

        .mount("/argument", routes![services::argument::list])
        .mount("/argument", routes![services::argument::create])
        .mount("/argument", routes![services::argument::find_by_id])
        .mount("/argument", routes![services::argument::delete])

        .mount("/conduct_code", routes![services::conduct_code::list])
        .mount("/conduct_code", routes![services::conduct_code::create])
        .mount("/conduct_code", routes![services::conduct_code::find_by_id])
        .mount("/conduct_code", routes![services::conduct_code::delete])

        .mount("/event", routes![services::event::list])
        .mount("/event", routes![services::event::create])
        .mount("/event", routes![services::event::find_by_id])
        .mount("/event", routes![services::event::delete])

        .mount("/login", routes![services::login::list])
        .mount("/login", routes![services::login::create])
        .mount("/login", routes![services::login::find_by_id])
        .mount("/login", routes![services::login::delete])
        .mount("/login", routes![services::login::find_by_email])

        .mount("/preference", routes![services::preference::list])
        .mount("/preference", routes![services::preference::create])
        .mount("/preference", routes![services::preference::find_by_id])
        .mount("/preference", routes![services::preference::delete])
        
        .mount("/proposition", routes![services::proposition::list])
        .mount("/proposition", routes![services::proposition::create])
        .mount("/proposition", routes![services::proposition::find_by_id])
        .mount("/proposition", routes![services::proposition::delete])

        .mount("/role", routes![services::role::list])
        .mount("/role", routes![services::role::create])
        .mount("/role", routes![services::role::find_by_id])
        .mount("/role", routes![services::role::delete])

        .mount("/sensitivities", routes![services::sensitivities::list])
        .mount("/sensitivities", routes![services::sensitivities::create])
        .mount("/sensitivities", routes![services::sensitivities::find_by_id])
        .mount("/sensitivities", routes![services::sensitivities::delete])

        .mount("/survey_result", routes![services::survey_result::list])
        .mount("/survey_result", routes![services::survey_result::create])        
        .mount("/survey_result", routes![services::survey_result::find_by_id])
        .mount("/survey_result", routes![services::survey_result::delete])

        .mount("/user", routes![services::user::create])
        .mount("/user", routes![services::user::find_by_id])
        .mount("/user", routes![services::user::delete])
        .mount("/user", routes![services::user::list])
}