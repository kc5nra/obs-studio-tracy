use anyhow::Result;
use log::{debug, warn};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

pub async fn offer(
    url: &str,
    bearer_token: &str,
    local_desc: RTCSessionDescription,
) -> Result<RTCSessionDescription> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/sdp"));

    if !bearer_token.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {bearer_token}"))?,
        );
    }

    let res = client
        .post(url)
        .headers(headers)
        .body(local_desc.sdp)
        .send()
        .await?;

    let body = res.text().await?;
    let sdp = RTCSessionDescription::answer(body)?;
    Ok(sdp)
}

pub async fn delete(url: &str) -> Result<()> {
    if url.is_empty() {
        debug!("Not sending DELETE since there is no WHIP resource.");
        return Ok(());
    }

    let client = reqwest::Client::new();

    debug!("Sending DELETE to whip resource.");
    let res = client.delete(url).send().await?;

    if !res.status().is_success() {
        warn!("Failed DELETE of whip resource: {}", res.status())
    }

    Ok(())
}
