use crate::read_config;
use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://www.youtube.com/youtubei/v1";

const API_KEY_WEB: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

const CONTEXT_WEB: &str = r#"{
    "context": {
        "client": {
            "clientName": "WEB",
            "clientVersion": "2.20220918",
            "gl": "{GL}",
            "hl": "{HL}"
        }
    },
    {BODY}
}"#;

const API_KEY_IOS: &str = "AIzaSyB-63vPrdThhKuerbB2N_l7Kwwcxj6yUAc";

const CONTEXT_IOS: &str = r#"{
    "context": {
        "client": {
            "clientName": "IOS",
            "clientVersion": "17.36.4",
            "gl": "{GL}",
            "hl": "{HL}"
        }
    },
    {BODY}
}"#;

pub async fn youtubei(endpoint: &str, body: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let (api_key, body) = match endpoint {
        "/player" => (API_KEY_IOS, get_body(CONTEXT_IOS, &body)),
        _ => (API_KEY_WEB, get_body(CONTEXT_WEB, &body)),
    };

    Ok(client
        .post(&format!("{BASE_URL}{endpoint}?key={api_key}"))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?
        .json::<Value>()
        .await?)
}

fn get_body(context: &str, body: &Value) -> String {
    let (gl, hl) = read_config();

    let body_str = body.to_string();
    let mut chars = body_str.chars();

    chars.next();
    chars.next_back();

    context
        .replace("{GL}", gl)
        .replace("{HL}", hl)
        .replace("{BODY}", chars.as_str())
}
