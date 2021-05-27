#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod core;
mod models;
mod permissions;
mod utils;

// #[post("/login", format = "application/json", data = "<login>")]
//fn login(conn: utils::connection::DbConn, login: core::request::Login) -> &'static str {
#[post("/login")]
fn login() -> &'static str {
    //core::endpoints::login(conn.0, login)
    "Hello"
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
    let p = utils::connection::init_pool();
    rocket::ignite()
        .manage(p)
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
