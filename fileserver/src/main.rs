use rocket::fs::{FileServer, relative};


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("public")))
        .launch()
        .await?;

    Ok(())
}
