use rocket::get;

#[get("/")]              // <- route attribute
pub fn world() -> &'static str {  // <- request handler
    "hello, world!"
}