extern crate scrabber;

use scrabber::{ScrabSelector, Scrabber};

fn main() {
    let url = "https://enquiry.indianrail.gov.in/mntes/q?opt=TrainRunning&subOpt=ShowRunC";
    let params = vec![
        ("trainNo", "12076"),
        ("jStation", "TVC#false"),
        ("jDate", "22-Jun-2018"),
        ("jDateMap", "22-Jun-2018"),
        ("jDateDay", "Fri"),
    ];
    let selector = ScrabSelector::new("table#ResTab > tbody", "td");
    let scrabr = Scrabber::new(url, params, selector);
    println!("{:#?}", scrabr.start());
}
