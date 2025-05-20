use fantoccini::ClientBuilder;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
mod apply;
mod config;
mod utils;
use apply::Application;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    // 加载配置
    let config = Config::from_file("config.toml").expect("配置文件加载失败");

    let mut caps = serde_json::Map::new();
    let browser_key = match config.browser {
        config::Browser::Chrome => "goog:chromeOptions",
        config::Browser::Edge => "ms:edgeOptions",
        config::Browser::Firefox => "moz:firefoxOptions",
    };
    caps.insert(
        browser_key.to_string(),
        json!({
            "args": config.args
        }),
    );

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&config.webdriver_url)
        .await
        .map_err(|e| fantoccini::error::CmdError::NotW3C(json!(e.to_string())))?;

    let app = Application::new(client);

    loop {
        // 清除所有 cookies
        app.client.delete_all_cookies().await?;

        app.submit_application(
            &config.application_url,
            &config.domain,
            &config.first_name,
            &config.last_name,
            &config.address,
            &config.city,
            &config.state,
            &config.postcode,
            &config.phone,
        )
        .await?;
        println!("✅ 申请已提交，等待 {} 秒后再次申请", config.wait_time);
        sleep(Duration::from_secs(config.wait_time)).await;
    }

}
