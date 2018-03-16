#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate serde_derive;

#[derive(Deserialize)]
// Temporarily disable these warnings (TODO)
#[allow(dead_code)]
#[allow(non_snake_case)]
struct CspReport {
    documentURI: String,
    referrer: String,
    blockedURI: String,
    violatedDirective: String,
    effectiveDirective: String,
    originalPolicy: String,
    disposition: String,
    sourceFile: String,
    statusCode: u16,
    lineNumber: i32,
    columnNumber: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
