#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/login")]
fn login() -> &'static str {
    "Hello, world!"
}

#[get("/introspection")]
fn introspection() -> &'static str {
    "Hello, world!"
}

#[get("/permissions")]
fn permissions() -> &'static str {
    "Hello, world!"
}

#[get("/user/permissions")]
fn get_user_permissions() -> &'static str {
    "Hello, world!"
}

#[post("/user/permissions")]
fn update_user_permissions() -> &'static str {
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
                permissions,
                get_user_permissions,
                update_user_permissions
            ],
        )
        .launch();
}
