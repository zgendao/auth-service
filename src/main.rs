#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod models;
mod permissions;
mod utils;

#[get("/login")]
fn login() -> &'static str {
    "Hello, world!"
}

#[get("/introspection")]
fn introspection() -> &'static str {
    "Hello, world!"
}

#[get("/permissions")]
fn get_permissions() -> &'static str {
    "Hello, world!"
}

#[post("/permissions")]
fn add_permissions() -> &'static str {
    "Hello, world!"
}

#[get("/user/permissions")]
fn get_user_permissions() -> &'static str {
    "Hello, world!"
}

#[post("/user/permissions")]
fn add_user_permissions() -> &'static str {
    // Someone who has admin permission
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount(
            "/auth",
            routes![
                login,
                introspection,
                add_permissions,
                get_user_permissions,
                add_user_permissions
            ],
        )
        .launch();
}
