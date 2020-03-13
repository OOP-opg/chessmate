use rocket_contrib::serve::StaticFiles;
use rocket;

#[rocket::get("/")]
fn world() -> &'static str {
    "Hello, world"
}

pub fn run_server() {
    rocket::ignite()
        .mount("/hello", rocket::routes![world])
        .mount("/", StaticFiles::from("./static"))
        .launch();
}
