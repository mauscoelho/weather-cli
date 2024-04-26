fn main() {
    let _api_token = std::env::var("API_TOKEN").expect("expected API_TOKEN in env");

    let mut arg_iterator = std::env::args();
    arg_iterator.next();
    let args: String = arg_iterator.collect();

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.waqi.info/search/")
        .query(&[("token", _api_token), ("keyword", args)])
        .send()
        .expect("request failed")
        .json::<serde_json::Value>()
        .expect("json parsing failed");

    dbg!(response);
}
