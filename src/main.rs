#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn world() -> &'static str {
    "Hello, world"
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![world])
        .mount("/", StaticFiles::from("./static"))
        .launch();
}
