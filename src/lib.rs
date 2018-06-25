extern crate regex;
extern crate reqwest;
extern crate scraper;

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

type ScrabParams<'a> = Vec<(&'a str, &'a str)>;
type ScrabResult<T> = Result<T, Box<Error>>;

pub struct ScrabSelector {
    pub container_selector: String,
    pub item_selector: String,
}

impl ScrabSelector {
    pub fn new(container_selector: &str, item_selector: &str) -> Result<ScrabSelector, &'static str> {
        if container_selector == "" || item_selector == "" {
            return Err("Selectors shouldn't be empty");
        }
        Ok(ScrabSelector {
            container_selector: container_selector.to_string(),
            item_selector: item_selector.to_string(),
        })
    }
}

pub struct Scrabber<'a> {
    pub url: &'a str,
    pub params: ScrabParams<'a>,
    pub selector: ScrabSelector,
}

impl<'a> Scrabber<'a> {
    pub fn new(
        url: &'a str,
        params: ScrabParams<'a>,
        selector: ScrabSelector,
    ) -> Result<Scrabber<'a>, &'static str> {
        if url == "" || params.len() == 0 {
            return Err("All params should have valid values.");
        }
        Ok(Scrabber {
            url,
            params,
            selector,
        })
    }

    pub fn start(&self) -> ScrabResult<Vec<Vec<String>>> {
        let html = get_html_from_url(&self.url, &self.params).unwrap();
        Ok(get_vals_from_container(&html, &self.selector)?)
    }
}

fn get_html_from_url(url: &str, params: &ScrabParams) -> ScrabResult<String> {
    let client = Client::new();
    let mut resp = client.post(url).form(&params).send()?;
    Ok(resp.text()?)
}

fn get_vals_from_container(html: &str, selector: &ScrabSelector) -> ScrabResult<Vec<Vec<String>>> {
    let document = Html::parse_document(html);
    let container_selector = Selector::parse(&selector.container_selector).unwrap();
    let item_selector = Selector::parse(&selector.item_selector).unwrap();
    let re = Regex::new("(\n|\t)")?;
    let container = document
        .select(&container_selector)
        .nth(0).unwrap();
    Ok(
        container
        .select(&item_selector)
        .map(|x| {
            let texts: Vec<_> = x.text()
                .map(|y| re.replace_all(y, "").to_string())
                .filter(|y| y.len() > 0)
                .collect();
            texts
        })
        .collect())
}
