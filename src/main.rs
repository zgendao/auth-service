#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

mod core;
mod models;
mod utils;

use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::json::Json;

/// Login endpoint
///
/// Accept a login request with eth_address and signed message and returns
/// the user entity with groups and permissions on that group, also add the
/// internal permissions as well.
/// Token added what can be used for introspection.
#[post("/login", format = "application/json", data = "<login>")]
fn login(conn: utils::connection::DbConn, login: Json<core::request::Login>) -> String {
    core::endpoints::login(&*conn.0, login.0)
}

/// Introspection endpoint
///
/// Accepts a token and validate whether it's an authenticated user or not. Returns details
/// about the owner of the token like permissions on groups and internal permissions
#[get("/introspection")]
fn introspection(conn: utils::connection::DbConn, auth: Authorization) -> String {
    core::endpoints::introspection(&*conn.0, &auth.0)
}

/// Create registration token endpoint
///
/// Creates a registration token for adding new users. Requires `manage_users` permission.
#[post("/register_token")]
fn register_token(conn: utils::connection::DbConn, auth: Authorization) -> String {
    core::endpoints::register_token(&*conn.0, &auth.0)
}

/// Register endpoint
///
/// Creates a new user in the system, it's done by the register token sent with the register
/// request. Register token can be created by a user who has `manage_users` internal permission.
#[post("/register", format = "application/json", data = "<register>")]
fn register(conn: utils::connection::DbConn, register: Json<core::request::Register>) -> String {
    core::endpoints::register(&*conn.0, register.0)
}

/// Create permission endpoint
///
/// Creates a new permission, requires `manage_permissions` internal permission.
#[post("/permissions", format = "application/json", data = "<permission>")]
fn create_permission(
    conn: utils::connection::DbConn,
    permission: Json<core::request::Permission>,
    auth: Authorization,
) -> String {
    core::endpoints::create_permission(&*conn.0, permission.0, &auth.0)
}

/// Create group endpoint
///
/// Creates a new group, requires `manage_groups` internal permission.
/// TODO Should have owner and the owner must be the creator
#[post("/groups", format = "application/json", data = "<group>")]
fn create_group(
    conn: utils::connection::DbConn,
    group: Json<core::request::Group>,
    auth: Authorization,
) -> String {
    core::endpoints::create_group(&*conn.0, group.0, &auth.0)
}

/// Add user_group endpoint
///
/// user_group is an entity where we add a certain permission to a user in a certain group. Requires
/// `manage_users` internal permission.
#[put("/users/permissions", format = "application/json", data = "<ug>")]
fn add_user_group(
    conn: utils::connection::DbConn,
    ug: Json<core::request::UserGroup>,
    auth: Authorization,
) -> String {
    core::endpoints::add_user_group(&*conn.0, ug.0, &auth.0)
}

/// Add internal permission endpoint
///
/// Add one of the internal permissions to a certain user. Requires `set_internal_permissions`
/// internal permission.
#[put(
    "/users/internal-permissions",
    format = "application/json",
    data = "<ug>"
)]
fn add_user_internal_permission(
    conn: utils::connection::DbConn,
    ug: Json<core::request::UserInternalPermission>,
    auth: Authorization,
) -> String {
    core::endpoints::add_user_internal_permission(&*conn.0, ug.0, &auth.0)
}

/// Get permissions endpoint
///
/// Returns the available permissions in the system.
/// Requires `get_permissions` internal permission.
#[get("/permissions")]
fn get_permissions() -> &'static str {
    "Hello, world!" // TODO
}

/// Create permissions endpoint
///
/// Creates a new permission in the system.
/// Requires `manage_permissions` internal permission.
#[post("/permissions")]
fn add_permissions() -> &'static str {
    "Hello, world!" // TODO
}

/// Get user endpoint
///
/// TODO
#[get("/users")]
fn get_user_permissions() -> &'static str {
    "Hello, world!" // TODO
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
                register_token,
                register,
                create_permission,
                create_group,
                add_user_group,
                add_user_internal_permission,
            ],
        )
        .launch();
}

struct Authorization(String);

impl<'a, 'r> FromRequest<'a, 'r> for Authorization {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Authorization, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        return Outcome::Success(Authorization(keys[0].to_string()));
    }
}
