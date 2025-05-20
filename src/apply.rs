use crate::utils::get_current_time_stamp;
use fantoccini::{Client, Locator};
use rand::Rng;

pub struct Application {
    pub client: Client,
}

impl Application {
    pub fn new(client: Client) -> Self {
        Application { client }
    }

    pub async fn submit_application(
        &self,
        application_url: &str,
        domain: &str,
        first_name: &str,
        last_name: &str,
        address: &str,
        city: &str,
        state: &str,
        postcode: &str,
        phone: &str,
    ) -> Result<(), fantoccini::error::CmdError> {
        self.client.goto(application_url).await?;

        println!("✅ 页面加载完成");

        let cookie_button = self
            .client
            .wait()
            .for_element(Locator::XPath(
                "//a[.//span[contains(text(), 'Accept all cookies')]]",
            ))
            .await?;

        println!("✅ 找到 'Accept all cookies'");
        let mut rng = rand::rng();
        let wait_ms = rng.random_range(1500..=3000);

        println!("⏳ 随机等待 {} 毫秒", wait_ms);

        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        cookie_button.click().await?;
        println!("✅ 已点击 'Accept all cookies'");

        self.client
            .wait()
            .for_element(Locator::Css("#preselected-simo-order-details-form"))
            .await?;

        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#firstname"))
            .await?
            .send_keys(first_name)
            .await?;

        println!("✅ 填写名字: {}", first_name);

        let mut rng = rand::rng();
        let wait_ms = rng.random_range(3000..=6000);

        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        self.client
            .find(Locator::Css("#lastname"))
            .await?
            .send_keys(last_name)
            .await?;

        println!("✅ 填写姓氏: {}", last_name);
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;
        let timestamp = get_current_time_stamp();
        let email = format!("{}@{}", timestamp, domain);
        self.client
            .find(Locator::Css("#email"))
            .await?
            .send_keys(&email)
            .await?;
        println!("✅ 填写邮箱: {}", email);

        self.client
            .execute("window.scrollBy(0, 400);", vec![])
            .await?;

        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#countrySelector"))
            .await?
            .send_keys("China")
            .await?;
        println!("✅ 填写国家: China");
        println!("✅ 基本表单填写完成");

        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#internationalline1"))
            .await?
            .send_keys(address)
            .await?;
        println!("✅ 填写地址: {}", address);
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#internationalline2"))
            .await?
            .send_keys(&format!("{} {}", phone, timestamp))
            .await?;
        println!("✅ 填写地址2: {} {}", phone, timestamp);
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#internationalcity"))
            .await?
            .send_keys(city)
            .await?;
        println!("✅ 填写城市: {}", city);
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#internationalstate"))
            .await?
            .send_keys(state)
            .await?;
        println!("✅ 填写省份: {}", state);
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#internationalpostcode"))
            .await?
            .send_keys(postcode)
            .await?;
        println!("✅ 填写邮政编码: {}", postcode);
        println!("✅ 地址表单填写完成");
        let wait_ms = rng.random_range(3000..=6000);
        println!("⏳ 随机等待 {} 毫秒", wait_ms);
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        self.client
            .find(Locator::Css("#submitButton"))
            .await?
            .click()
            .await?;
        println!("✅ 已点击 'Order your free SIM' 按钮");

        Ok(())
    }
}
