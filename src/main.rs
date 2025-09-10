
#[macro_use] extern crate rocket;

mod handlers;
mod models;
use handlers::{index, ppakcoding, get_posts, get_users, get_user};


#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment().merge(("port", 4000)))
    .mount("/", routes![index, ppakcoding, get_posts, get_users, get_user])
}