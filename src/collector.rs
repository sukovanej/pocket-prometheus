pub async fn collect_metrics(host: &str, port: i32) -> reqwest::Result<String> {
    let response = reqwest::get(format!("{}:{}/metrics", host, port)).await?;
    response.text().await
}
