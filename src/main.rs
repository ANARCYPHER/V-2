#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "V-2!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}