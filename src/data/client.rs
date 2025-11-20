use color_eyre::Result;
use reqwest::{header, Client as ReqwestClient};
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    pub fn new(session_key: Option<String>) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        if let Some(k) = session_key {
            headers.append("Cookie", header::HeaderValue::from_str(&format!("session={}", k))?);
        }

        let client = ReqwestClient::builder().default_headers(headers).build()?;
        Ok(Self {
            client
        })
    }

    pub fn set_session_key(&mut self, session_key: String) -> Result<()> {
        let mut headers = header::HeaderMap::new();
        headers.append("Cookie", header::HeaderValue::from_str(&format!("session={}", session_key))?);

        let client = ReqwestClient::builder().default_headers(headers).build()?;

        self.client = client;

        Ok(())
    }
}

