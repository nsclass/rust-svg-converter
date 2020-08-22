use assert_json_diff::assert_json_include;
use serde_json::json;
use svg_web_service::config::Config;
use svg_web_service::run;

#[actix_rt::test]
async fn health_check_test() {
    let conf = Config {
        host: "localhost".to_string(),
        port: 8080
    };

    spawn_app(conf.clone());
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}:{}/health", conf.host, conf.port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let res: serde_json::Value = response.json().await.expect("failed to get a response");
    assert_json_include!(
        actual: res,
        expected: json!({
            "status": "UP",
        })
    )
}

// all the things.
fn spawn_app(conf: Config) {
    let server = run(conf).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
