mod indicator;
mod data_item;
mod errors;
mod traits;

use crate::indicator::ExponentialMovingAverage as Ema;
use crate::data_item::DataItem;
use crate::traits::{Close, Next};
use csv::Reader; 
use serde::Deserialize; // use for serializing/deserilize rust data structures
use std::env; 
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn main() {
    // print the current directory (debugging)
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    let mut ema = match Ema::new(25) {
        Ok(ema) => ema,
        Err(e) => {
            eprintln!("Error creating EMA: {}", e);
            process::exit(1);
        }
    };

    let mut reader = match Reader::from_path("examples/data/AMZN.csv") {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    };

    for result in reader.deserialize() {
        let record: Record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                process::exit(1);
            }
        };

        let dt = match DataItem::builder()
            .open(record.open)
            .high(record.high)
            .low(record.low)
            .close(record.close)
            .volume(record.volume)
            .build()
        {
            Ok(dt) => dt,
            Err(e) => {
                eprintln!("Error creating DataItem: {}", e);
                process::exit(1);
            }
        };

        let mut ema_val = ema.next(&dt);
        println!("{}: {} = {:2.2}", record.date, "EMA", ema_val);
    }
}
