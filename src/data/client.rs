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
    pub fn new() -> Result<Self> {
        let client = ReqwestClient::builder().build()?;
        Ok(Self { client })
    }

    pub async fn get_events(&self, session_token: Option<String>) -> Result<Vec<AOCEvent>> {
        let mut headers = header::HeaderMap::new();
        if let Some(k) = session_token {
            headers.append(
                "Cookie",
                header::HeaderValue::from_str(&format!("session={};", k))?,
            );
        }
        let body = self
            .client
            .get(format!("{}{}", Self::BASE_URL, "/events"))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        let doc = Html::parse_document(&body);

        let event_selector = Selector::parse(".eventlist-event").unwrap();
        let star_selector = Selector::parse(".star-count").unwrap();
        let out_of_selector = Selector::parse(".quiet").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        let mut events: Vec<AOCEvent> = Vec::new();

        for element in doc.select(&event_selector) {
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
