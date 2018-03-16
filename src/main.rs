#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket::response::status;
use rocket_contrib::Json;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct JsonBody {
    csp_report: CspReport
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all = "kebab-case")]
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

#[post("/", format = "application/json", data = "<json_body>")]
fn process_csp_report(json_body: Json<JsonBody>) -> status::NoContent {
    let csp_report = &json_body.csp_report;
    let json_report = serde_json::to_string(&csp_report)
        .expect("Error: Couldn't serialize the CSP report.");
    let log_path = Path::new("csp-log.txt");
    let mut log_file = OpenOptions::new().create(true).append(true).open(&log_path)
        .expect("Error: Couldn't open the log file.");
    let mut log_entry = String::from("Sucessfully received a CSP report:\n");
    log_entry.push_str(&json_report);
    log_entry.push_str("\n\n");
    let _ = log_file.write_all(log_entry.as_bytes());
    status::NoContent
}

fn main() {
    rocket::ignite().mount("/", routes![process_csp_report]).launch();
}
