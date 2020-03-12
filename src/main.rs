#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn world() -> &'static str {
    "Hello, world"
}

fn main() {
    rocket::ignite().mount("/", routes![world]).launch();
}
