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
    pub outer_selector: String,
    pub inner_selector: String,
}

impl ScrabSelector {
    pub fn new(outer_selector: &str, inner_selector: &str) -> Result<ScrabSelector, &'static str> {
        if outer_selector == "" || inner_selector == "" {
            return Err("Selectors shouldn't be empty");
        }
        Ok(ScrabSelector {
            outer_selector: outer_selector.to_string(),
            inner_selector: inner_selector.to_string(),
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
    let container_selector = Selector::parse(&selector.outer_selector).unwrap();
    let inner_selector = Selector::parse(&selector.inner_selector).unwrap();
    let re = Regex::new("(\n|\t)")?;
    let container = document
        .select(&container_selector)
        .next().unwrap();
    Ok(
        container
        .select(&inner_selector)
        .map(|x| {
            let texts: Vec<_> = x.text()
                .map(|y| re.replace_all(y, "").to_string())
                .filter(|y| y.len() > 0)
                .collect();
            texts
        })
        .collect())
}
