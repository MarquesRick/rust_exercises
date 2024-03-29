//for import all globally
#[macro_use]
extern crate rocket;

use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::broadcast::channel,
};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/hello", routes![world])
}
