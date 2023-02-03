#![allow(non_snake_case)]
use fantoccini::{ ClientBuilder }; //, Locator};
use serde_json::{ Value, Map, json };
// use std::process::Command;
use std::fs;
use std::path::Path;
// mod paris_swap; //leetcode mod to activate RustAnalyser hints; not related to this project.

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    match ExecuteScripts().await {
        Ok(()) => (),
        Err(err) => println!("error {}: ", err),
    }

    Ok(())
}

/* async fn main() -> () {

    // executeShell();
    Command::new("sh").arg("-c").arg("cd $HOME ; cd /Documents/Rust/chromedriver_linux64").arg("../chromedriver_linux64/chromedriver --port=5000").spawn().expect("ls command failed to start");

    let rawCaps = r#"
        {
            "browserName": "chrome",
            "browserVersion": "109",
            "goog:chromeOptions": {
                "args" : [
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
                    ]
            }
        }"#;

    let link1 = "https://en.wikipedia.org/wiki/Foobar";
    let link2 = "https://docs.rs/tokio/1.10.1/tokio/task/index.html";
    let link3 = "https://www.w3.org/TR/webdriver1";

    let join = tokio::task::spawn(async {
        scraper(rawCaps, link1).await.unwrap();
    });

    let join2 = tokio::task::spawn(async {
        scraper2(rawCaps, link2).await.unwrap();
    });

    let join3 = tokio::task::spawn(async {
        scraper3(rawCaps, link3).await.unwrap();
    });

    join.await.unwrap();
    join2.await.unwrap();
    join3.await.unwrap();
} */

/* async fn scraper(rawCaps: &str, link: &str) -> Result<(), fantoccini::error::CmdError>{
    let capsMap = serde_json::from_str::<Map<String, Value>>(rawCaps).unwrap();
    let c = ClientBuilder::native().capabilities(capsMap).connect("http://localhost:5000").await.expect("failed to connect to WebDriver");

    c.goto(link).await?;
    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    c.find(Locator::Css(".mw-disambig")).await?.click().await?;
    let el = c.find(Locator::Css(".mw-disambig")).await?;
    let text = match el.text().await{
        Ok(text) =>text,
        Err(e) => panic!("Eooror{}", e)
    };

    println!("string output {}", text);

    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");

    c.close().await
}

async fn scraper2(rawCaps: &str, link: &str) -> Result<(), fantoccini::error::CmdError>{
    let capsMap = serde_json::from_str::<Map<String, Value>>(rawCaps).unwrap();
    let c = ClientBuilder::native().capabilities(capsMap).connect("http://localhost:5000").await.expect("failed to connect to WebDriver");
    // let session_id = c.session_id().await.unwrap();
    // let session_id = match c.session_id().await.unwrap(){
    //     Some(session_id) => session_id,
    //     None => panic!("could not get session id")
    // };
    // println!("scraper2 session_id: {}", session_id);

    c.goto(link).await?;
    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), link);

    c.find(Locator::Css("#what-are-tasks")).await?.click().await?;
    let el = c.find(Locator::Css("#what-are-tasks")).await?;
    let text = match el.text().await{
        Ok(text) =>text,
        Err(e) => panic!("Eooror{}", e)
    };

    println!("string output {}", text);

    c.find(Locator::LinkText("Working with Tasks")).await?.click().await?;

    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://docs.rs/tokio/1.10.1/tokio/task/index.html#working-with-tasks");

    c.close().await
}

async fn scraper3(rawCaps: &str, link: &str) -> Result<(), fantoccini::error::CmdError>{
    let capsMap = serde_json::from_str::<Map<String, Value>>(rawCaps).unwrap();
    let c = ClientBuilder::native().capabilities(capsMap).connect("http://localhost:5000").await.expect("failed to connect to WebDriver");

    c.goto(link).await?;
    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), link);

    c.find(Locator::Css("#x18-user-prompts")).await?.click().await?;
    let el = c.find(Locator::Css("#x18-user-prompts")).await?;
    let text = match el.text().await{
        Ok(text) =>text,
        Err(e) => panic!("Eooror{}", e)
    };

    println!("string output {}", text);

    c.find(Locator::LinkText("current user prompt")).await?.click().await?;

    let _url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://docs.rs/tokio/1.10.1/tokio/task/index.html#working-with-tasks");

    c.close().await
}
 */

async fn ExecuteScripts() -> Result<(), fantoccini::error::CmdError> {
    // executeShell(); relative path
    // Command::new("sh").arg("-c").arg("cd /home/cocdingarcade/Documents/Rust/chromedriver_linux64; ./chromedriver --port=5000").spawn().expect("ls command failed"); //.arg("sudo ./chromedriver_linux64/chromedriver --port=5000").spawn().expect("ls command failed to start");

    let rawCaps =
        r#"
    {
        "browserName": "chrome",
        "browserVersion": "109",
        "goog:chromeOptions": {
            "args" : [
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
                ]
        }
    }"#;

    let path = Path::new("/home/cocdingarcade/Documents/Rust/wikiexample/src/executeAsyncFanto.js");
    let contentscript = fs::read_to_string(path)?;
    let capsMap = serde_json::from_str::<Map<String, Value>>(rawCaps).unwrap();
    let c = ClientBuilder::native()
        .capabilities(capsMap)
        .connect("http://localhost:5000").await
        .expect("failed to connect to WebDriver");

    let link = "https://en.wikipedia.org/wiki/Foobar";
    c.goto(link).await?;

    let date = json!(12.5);
    println!("date {}", &date);

    let callbackValue = c.execute_async(&contentscript, vec![date]).await?;
    let returnedObject = &json!(callbackValue);
    println!("callbackValue: {}", returnedObject["testkey"]);

    // c.close().await
    // c.persist().await;

    let _session_result = match c.persist().await {
        Ok(()) => (),
        Err(error) => panic!("error creating session {} ", error),
    };

    let GetTableScript =
        r#"
            const [date2, callback] = arguments;
            console.log('document.body.tableData: ', document.body.tableData);
            callback(document.body.tableData);
        "#;

    loop {
        let date2 = json!(12.5);
        let callbackValue = c.execute_async(&GetTableScript, vec![date2]).await?;

        println!("callbackvalue {}", callbackValue);

        if callbackValue["rustkey"] == "done" {
            println!("Returned from Js Script {}: ", callbackValue);
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(10000)).await;
        println!("thread sleeping for 10 second.");
    }

    c.close().await
    // Ok(())
}