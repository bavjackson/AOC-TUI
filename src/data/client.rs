use crate::data::event::AOCEvent;
use color_eyre::Result;
use reqwest::{Client as ReqwestClient, header};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Client {
    client: ReqwestClient,
}

impl Client {
    const BASE_URL: &'static str = "https://adventofcode.com";
    pub fn new(session_key: &Option<String>) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        if let Some(k) = session_key {
            headers.append(
                "Cookie",
                header::HeaderValue::from_str(&format!("session={};", k))?,
            );
        }

        let client = ReqwestClient::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    pub fn set_session_key(&mut self, session_key: String) -> Result<()> {
        let mut headers = header::HeaderMap::new();
        headers.append(
            "Cookie",
            header::HeaderValue::from_str(&format!("session={};", session_key))?,
        );

        let client = ReqwestClient::builder().default_headers(headers).build()?;

        self.client = client;

        Ok(())
    }

    pub async fn get_events(&self) -> Result<Vec<AOCEvent>> {
        let body = self
            .client
            .get(format!("{}{}", Self::BASE_URL, "/events"))
            .send()
            .await?
            .text()
            .await?;

        let doc = Html::parse_document(&body);

        // println!("{:?}\n", doc.html());

        let event_selector = Selector::parse(".eventlist-event").unwrap();
        let star_selector = Selector::parse(".star-count").unwrap();
        let out_of_selector = Selector::parse(".quiet").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        let mut events: Vec<AOCEvent> = Vec::new();

        for element in doc.select(&event_selector) {
            // println!("{:?}\n", element.text());
            let link = element.select(&link_selector).next().unwrap();
            let stars: String;
            let out_of: String;
            let stars_element = element.select(&star_selector).next();

            match stars_element {
                Some(e) => stars = e.text().collect::<String>(),
                None => stars = "".to_string(),
            }

            let out_of_element = element.select(&out_of_selector).next();

            match out_of_element {
                Some(e) => out_of = e.text().collect::<String>(),
                None => out_of = "".to_string(),
            }

            let event = AOCEvent {
                url: format!("{}{}", Self::BASE_URL, link.value().attr("href").unwrap()),
                label: link.text().collect::<String>(),
                stars,
                out_of,
            };

            events.push(event);
        }

        Ok(events)
    }
}
