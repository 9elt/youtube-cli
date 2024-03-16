use crate::helper::jun;
use crate::read_config;
use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://suggestqueries-clients6.youtube.com";

pub async fn suggestions(query: &str) -> Option<Vec<String>> {
    let client = Client::new();

    let (gl, hl) = read_config();

    let url = format!(
        "{BASE_URL}/complete/search\
?client=youtube\
&hl={hl}\
&gl={gl}\
&q={query}\
&callback=_"
    );

    let js = &client.get(&url).send().await.ok()?.text().await.ok()?;

    // replace callback
    let res = serde_json::from_str::<Value>(&js.replace("_ && _(", "").replace(')', "")).ok()?;

    jun!(res
        => 1
        => map
        => 0
        => to_string
    )
    .and_then(|vec| {
        Some(
            vec.iter()
                .cloned()
                .filter(|x| !x.trim().is_empty())
                .collect::<Vec<String>>(),
        )
    })
}
