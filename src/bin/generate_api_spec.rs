#![allow(warnings)]

fn main() {
    // Construct the OpenAPI service the same way as in routes (with server "/api")
    let service = data_dance::web::routes::api::api_service().server("/api");

    // Print pretty JSON spec to stdout
    let spec_json = service.spec();
    println!("{}", spec_json);
}


