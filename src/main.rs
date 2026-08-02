use serde::Serialize;

#[derive(Serialize)]
struct SystemState {
    app: String,
    category: String,
    tech: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let state = SystemState {
        app: "retail-pos-rfid-register-rust-axum-v2026-b51".to_string(),
        category: "Retail Smart POS Register & RFID Scanner".to_string(),
        tech: "Rust / Tokio & Axum Asynchronous Core".to_string(),
        status: "OPERATIONAL".to_string(),
    };
    println!("--- High-Performance Rust Microservice ---");
    println!("{}", serde_json::to_string_pretty(&state).unwrap());
}
