#![allow(non_snake_case)]
use fantoccini::{ClientBuilder, Locator};
// use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    
    let rawCaps = r#"
        {
            "browserName": "chrome",
            "browserVersion": "109",
            "goog:chromeOptions": {
                "args" : [
                    "--user-data-dir=/home/cocdingarcade/.config/google-chrome/Default",
                    "--headless",
                    "--no-sandbox",
                    "--no-zygote",
                    "--disable-setuid-sandbox",
                    "--disable-dev-shm-usage",
                    "--disable-blink-features=AutomationControlled",
                    "--disable-accelerated-2d-canvas",
                    "--disable-gpu",
                    "--window-size=1920x1080"
                    ],
                "excludeSwitches" : [
                    "enable-automation",
                    "enable-logging"
                    ],
                "useAutomationExtension" : false
            }
        }"#;

    let capsMap = serde_json::from_str::<Map<String, Value>>(rawCaps).unwrap();
    let c = ClientBuilder::native().capabilities(capsMap).connect("http://localhost:9515").await.expect("failed to connect to WebDriver");

    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    c.find(Locator::Css(".mw-disambig")).await?.click().await?;
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await

    // Ok(())
}
