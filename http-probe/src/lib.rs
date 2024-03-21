use std::time::Duration;
use futures::future::try_join_all;


use reqwest::{Client, Method};
pub mod error;

use error::HttpProbeError::{
    self,
    ClientError,
};

pub async fn test_domains(domains: Vec<Box<str>>, timeout: u64) -> Result<Vec<String>, HttpProbeError> {    
    let method = Method::HEAD;
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
        .map_err(ClientError)?;

    let tasks = domains.into_iter().map(|url| {
        let client = client.clone();
        let url = format!("https://{}", url);
        let method = method.clone();
        async move {
            if is_listening(&client, &url, method).await {
                Ok(Some(url))
            } else {
                Ok(None)
            }
        }
    });

    let https_listening: Vec<String> = try_join_all(tasks).await?
        .into_iter()
        .filter_map(|option| option)
        .collect();
    
    Ok(https_listening)
}

pub async fn is_listening(client: &Client, url: &str, method: Method) -> bool {
    let req = client.request(method, url)
        .header("Connection", "close");

    match req.send().await {
        Ok(_) => true,
        Err(_) => false,
    }

}