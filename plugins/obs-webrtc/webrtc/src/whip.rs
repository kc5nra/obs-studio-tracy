use anyhow::Result;
use log::{debug, warn};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

pub async fn offer(
    url: &str,
    stream_key: &str,
    local_desc: RTCSessionDescription,
) -> Result<RTCSessionDescription> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/sdp"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {stream_key}"))?,
    );

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

pub async fn delete(url: &str, stream_key: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {stream_key}"))?,
    );

    debug!("Sending DELETE to whip endpoint");
    let res = client.delete(url).headers(headers).send().await?;

    if !res.status().is_success() {
        warn!("Failed DELETE of whip endpoint: {}", res.status())
    }

    Ok(())
}
