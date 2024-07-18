use crate::indicator::ExponentialMovingAverage as Ema;
use crate::traits::{Close, Next};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use crate::data_item::DataItem;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Record {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn process_chunk(
    chunk: &[Record],
    start_date: chrono::NaiveDate,
    chunk_index: usize,
    chunk_size: usize,
) -> (Vec<String>, Vec<f64>, Vec<f64>) {
    let mut dates = Vec::new();
    let mut closing_prices = Vec::new();
    let mut ema_values = Vec::new();

    let mut ema = Ema::new(25).unwrap();

    for (i, record) in chunk.iter().enumerate() {
        let date = start_date + chrono::Duration::days((chunk_index * chunk_size + i) as i64);
        dates.push(date.to_string());
        closing_prices.push(record.close);

        let dt = DataItem::builder()
            .open(record.open)
            .high(record.high)
            .low(record.low)
            .close(record.close)
            .volume(record.volume)
            .build()
            .unwrap();

        let ema_val = ema.next(&dt);
        ema_values.push(ema_val);
    }

    (dates, closing_prices, ema_values)
}

pub fn read_and_process_csv_parallel(
    file_path: &str,
    start_date: chrono::NaiveDate,
    chunk_size: usize,
) -> Result<(Vec<String>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut records = Vec::new();
    for result in csv_reader.deserialize::<Record>() {
        let record: Record = result?;
        records.push(record);
    }   

    let chunks: Vec<_> = records.chunks(chunk_size).collect();

    let results: Vec<_> = chunks
        .par_iter()
        .enumerate()
        .map(|(i, chunk)| process_chunk(chunk, start_date, i, chunk_size))
        .collect();

    
    let mut date_res = Vec::new();
    let mut cp_res = Vec::new();
    let mut ema_val_res = Vec::new();

    results.iter().for_each(|(dates, closing_prices, ema_values)| {
        date_res.extend(dates.iter().cloned());
        cp_res.extend(closing_prices.iter().cloned());
        ema_val_res.extend(ema_values.iter().cloned());
    });

    Ok((date_res, cp_res, ema_val_res))
}