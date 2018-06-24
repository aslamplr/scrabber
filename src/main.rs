extern crate scrabber;
#[macro_use]
extern crate prettytable;
extern crate chrono;
extern crate clap;

use chrono::{Local, NaiveDate};
use clap::{App, Arg};
use prettytable::Table;
use scrabber::{ScrabSelector, Scrabber};

fn main() {
    let date = Local::today().format("%d-%b-%Y").to_string();
    let matches = App::new("Scrabber NTES CLI")
        .version("0.1.0")
        .author("Aslam <aslamplr@gmail.com>")
        .about("Scrab the train status from NTES")
        .arg(
            Arg::with_name("train_no")
                .short("t")
                .long("train")
                .value_name("TRAIN#")
                .help("Train number of the desired train, eg. 12076")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("station_name")
                .short("s")
                .long("station")
                .value_name("STATION")
                .help("Abbrevated station name, eg. TVC")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("journey_date")
                .short("d")
                .long("date")
                .value_name("DATE")
                .help("Journey date, eg. 24-Jun-2018")
                .takes_value(true)
                .default_value(&date),
        )
        .get_matches();

    let train_no = matches.value_of("train_no").unwrap();
    let station = format!("{}#false", matches.value_of("station_name").unwrap());
    let date = matches.value_of("journey_date").unwrap();
    let day = NaiveDate::parse_from_str(date, "%d-%b-%Y")
        .unwrap()
        .format("%a")
        .to_string();

    let url = "https://enquiry.indianrail.gov.in/mntes/q?opt=TrainRunning&subOpt=ShowRunC";
    let params = vec![
        ("trainNo", train_no),
        ("jStation", &station),
        ("jDate", date),
        ("jDateMap", date),
        ("jDateDay", &day),
    ];
    let selector = ScrabSelector::new("table#ResTab > tbody", "tr");
    let scrabr = Scrabber::new(url, params, selector);

    let mut table = Table::new();
    table.add_row(row!["Details"]);
    let res = scrabr.start().unwrap();
    res.into_iter().filter(|x| x.len() > 1).for_each(|x| {
        table.add_row(row![x[0], x[1..].join(" ")]);
    });

    table.printstd();
}
