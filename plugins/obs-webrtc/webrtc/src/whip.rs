use anyhow::Result;
use reqwest::header::{HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

pub async fn offer(
    url: &str,
    stream_key: &str,
    local_desc: RTCSessionDescription,
) -> Result<RTCSessionDescription> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/sdp")
    );
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
