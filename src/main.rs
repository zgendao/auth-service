#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod core;
mod models;
mod utils;

use rocket_contrib::json::Json;

/// Login endpoint
///
/// Accept a login request with eth_address and signed message and returns
/// the user entity with groups and permissions on that group, also add the
/// internal permissions as well.
/// Token added what can be used for introspection.
#[post("/login", format = "application/json", data = "<login>")]
fn login(conn: utils::connection::DbConn, login: Json<core::request::Login>) -> String {
// #[post("/login")]
// fn login() -> &'static str {
    core::endpoints::login(&*conn.0, login.0)
    // "Hello"
}

/// Introspection endpoint
///
/// Accepts a token and validate whether it's an authenticated user or not. Returns details
/// about the owner of the token like permissions on groups and internal permissions
#[get("/introspection")]
fn introspection() -> &'static str {
    "Hello, world!"
}

/// Create registration token endpoint
///
/// Creates a registration token for adding new users. Requires `manage_users` permission.
#[post("/register_token")]
fn register_token() -> &'static str {
    "Hello"
}

/// Register endpoint
///
/// Creates a new user in the system, it's done by the register token sent with the register
/// request. Register token can be created by a user who has `manage_users` internal permission.
#[get("/register")]
fn register() -> &'static str {
    "Hello"
}

/// Get permissions endpoint
///
/// Returns the available permissions in the system.
/// Requires `get_permissions` internal permission.
#[get("/permissions")]
fn get_permissions() -> &'static str {
    "Hello, world!"
}

/// Create permissions endpoint
///
/// Creates a new permission in the system.
/// Requires `manage_permissions` internal permission.
#[post("/permissions")]
fn add_permissions() -> &'static str {
    "Hello, world!"
}

/// Get user endpoint
///
/// TODO
#[get("/users")]
fn get_user_permissions() -> &'static str {
    "Hello, world!"
}

/// Add permission to user in group endpoint
///
/// Adds a certain permission to the user in a certain group. Requires `manage_permissions` internal
/// permission.
#[post("/users/permissions")]
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
