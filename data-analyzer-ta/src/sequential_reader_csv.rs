// sequential_reader.rs

use crate::{
    data_item::DataItem,
    indicator::ExponentialMovingAverage as Ema,
    traits::{Close, Next},
};
use csv::Reader;
use plotters::prelude::*;
use serde::Deserialize;
use std::{
    error::Error,
};

#[derive(Debug, Deserialize)]
struct Record {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn read_and_process_csv_sequential(
    file_path: &str,
    start_date: chrono::NaiveDate,
) -> Result<(Vec<String>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(file_path)?;

    let mut ema = Ema::new(25)?;

    let mut dates = vec![];
    let mut closing_prices = vec![];
    let mut ema_values = vec![];

    for (i, result) in csv_reader.deserialize().enumerate() {
        let record: Record = result?;

        let date = start_date + chrono::Duration::days(i as i64);
        dates.push(date.to_string());
        closing_prices.push(record.close);

        let dt = DataItem::builder()
            .open(record.open)
            .high(record.high)
            .low(record.low)
            .close(record.close)
            .volume(record.volume)
            .build()?;

        let ema_val = ema.next(&dt);
        ema_values.push(ema_val);
        // println!("{}: {} = {:2.2}", date, "EMA", ema_val);
        
    }

    Ok((dates, closing_prices, ema_values))
}
