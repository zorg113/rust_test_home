use smart_home::{smart_devices, smart_house, smart_house_errors};

#[macro_use]
extern crate rocket;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
}
