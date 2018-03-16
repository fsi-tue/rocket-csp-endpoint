#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate serde_derive;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
// Temporarily disable these warnings (TODO)
#[allow(dead_code)]
struct JsonBody {
    csp_report: CspReport
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
// Temporarily disable these warnings (TODO)
#[allow(dead_code)]
struct CspReport {
    document_uri: String,
    referrer: String,
    blocked_uri: String,
    effective_directive: String,
    violated_directive: String,
    original_policy: String,
    disposition: String,
    status_code: u16,
    source_file: Option<String>,
    line_number: Option<i32>,
    column_number: Option<i32>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
