use anyhow::Result;
use log::{debug, warn};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Url,
};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

pub async fn offer(
    url: &str,
    bearer_token: Option<&str>,
    local_desc: RTCSessionDescription,
) -> Result<(RTCSessionDescription, Url)> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/sdp"));

    if let Some(bearer_token) = bearer_token {
        if !bearer_token.is_empty() {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {bearer_token}"))?,
            );
        }
    }

    let res = client
        .post(url)
        .headers(headers)
        .body(local_desc.sdp)
        .send()
        .await?;

    let url = res.url().to_owned();
    let body = res.text().await?;
    let sdp = RTCSessionDescription::answer(body)?;

    Ok((sdp, url))
}

pub async fn delete(url: &Url) -> Result<()> {
    let client = reqwest::Client::new();

    debug!("Sending DELETE to whip resource: {url}");
    let res = client.delete(url.to_owned()).send().await?;

    if !res.status().is_success() {
        warn!("Failed DELETE of whip resource: {}", res.status())
    }

    Ok(())
}
