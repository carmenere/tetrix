use reqwest::StatusCode;
use std::error::Error;

#[tokio::test]
async fn get_version() -> Result<(), Box<dyn Error>> {
    let client = httpc_test::new_client("http://localhost:8888")?;
    let r = client.do_get("/version").await?;

    r.print().await?;

    let r = r.status().as_u16();

    if r != StatusCode::OK.as_u16() {
        Err("Http status != 200 Ok".into())
    } else {
        Ok(())
    }
}
