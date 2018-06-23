extern crate scrabber;
#[macro_use]
extern crate prettytable;

use prettytable::Table;
use scrabber::{ScrabSelector, Scrabber};

fn main() {
    let url = "https://enquiry.indianrail.gov.in/mntes/q?opt=TrainRunning&subOpt=ShowRunC";
    let params = vec![
        ("trainNo", "12076"),
        ("jStation", "TVC#false"),
        ("jDate", "24-Jun-2018"),
        ("jDateMap", "24-Jun-2018"),
        ("jDateDay", "Sun"),
    ];
    let selector = ScrabSelector::new("table#ResTab > tbody", "tr");
    let scrabr = Scrabber::new(url, params, selector);

    let mut table = Table::new();
    table.add_row(row!["Details"]);
    let res = scrabr.start();
    res.into_iter().filter(|x| x.len() > 1).for_each(|x| {
        table.add_row(row![x[0], x[1]]);
    });

    table.printstd();
}
