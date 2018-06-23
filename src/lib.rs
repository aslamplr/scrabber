extern crate regex;
extern crate reqwest;
extern crate scraper;

use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct ScrabSelector {
    pub outer_selector: String,
    pub inner_selector: String,
}

impl ScrabSelector {
    pub fn new(outer_selector: &str, inner_selector: &str) -> ScrabSelector {
        ScrabSelector {
            outer_selector: outer_selector.to_string(),
            inner_selector: inner_selector.to_string(),
        }
    }
}

pub struct Scrabber<'a> {
    pub url: &'a str,
    pub params: Vec<(&'a str, &'a str)>,
    pub selector: ScrabSelector,
}

impl<'a> Scrabber<'a> {
    pub fn new(
        url: &'a str,
        params: Vec<(&'a str, &'a str)>,
        selector: ScrabSelector,
    ) -> Scrabber<'a> {
        Scrabber {
            url,
            params,
            selector,
        }
    }

    pub fn start(&self) -> Vec<Vec<String>> {
        let html = get_html_from_url(&self.url, &self.params);
        get_vals_from_container(&html, &self.selector)
    }
}

fn get_html_from_url(url: &str, params: &Vec<(&str, &str)>) -> String {
    let client = Client::new();
    let mut resp = client.post(url).form(&params).send().unwrap();
    resp.text().unwrap()
}

fn get_vals_from_container(html: &str, selector: &ScrabSelector) -> Vec<Vec<String>> {
    let document = Html::parse_document(html);
    let container_selector = Selector::parse(&selector.outer_selector).unwrap();
    let inner_selector = Selector::parse(&selector.inner_selector).unwrap();
    let re = Regex::new("(\n|\t)").unwrap();
    document
        .select(&container_selector)
        .next()
        .unwrap()
        .select(&inner_selector)
        .map(|x| {
            let texts: Vec<_> = x.text()
                .map(|y| re.replace_all(y, "").to_string())
                .filter(|y| y.len() > 0)
                .collect();
            texts
        })
        .collect()
}
