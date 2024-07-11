use crate::indicator::ExponentialMovingAverage as Ema;
use crate::data_item::DataItem;
use crate::traits::{Close, Next};
use csv_async::AsyncReaderBuilder;
use futures::StreamExt;
use serde::Deserialize;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tokio::task;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[derive(Debug, Deserialize)]
struct Record {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub async fn read_and_process_csv_parallel(
    file_path: &str,
    start_date: chrono::NaiveDate,
) -> Result<(Vec<String>, Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = tokio::fs::File::open(file_path).await?;
    let file = file.compat();
    let mut reader = AsyncReaderBuilder::new()
        .has_headers(true)
        .create_deserializer(file);

    let ema = Arc::new(Mutex::new(Ema::new(25)?));

    // using the Arc and Mutex to share the data between tasks
    let dates = Arc::new(Mutex::new(vec![]));
    let closing_prices = Arc::new(Mutex::new(vec![]));
    let ema_values = Arc::new(Mutex::new(vec![]));

    let mut records = reader.deserialize::<Record>();

    while let Some(result) = records.next().await {
        let record: Record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };

        let ema = Arc::clone(&ema);
        let dates = Arc::clone(&dates);
        let closing_prices = Arc::clone(&closing_prices);
        let ema_values = Arc::clone(&ema_values);

        task::spawn(async move {
            let i = match dates.lock() {
                Ok(mut dates) => {
                    let i = dates.len();
                    let date = start_date + chrono::Duration::days(i as i64);
                    dates.push(date.to_string());
                    i
                }
                Err(poisoned) => {
                    eprintln!("Mutex poisoned: {:?}", poisoned);
                    return;
                }
            };

            if let Err(poisoned) = closing_prices.lock().map(|mut cp| cp.push(record.close)) {
                eprintln!("Mutex poisoned: {:?}", poisoned);
                return;
            }

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
                    return;
                }
            };

            let ema_val = match ema.lock() {
                Ok(mut ema) => ema.next(&dt),
                Err(poisoned) => {
                    eprintln!("Mutex poisoned: {:?}", poisoned);
                    return;
                }
            };

            if let Err(poisoned) = ema_values.lock().map(|mut ev| ev.push(ema_val)) {
                eprintln!("Mutex poisoned: {:?}", poisoned);
                return;
            }

            // println!("{}: {} = {:2.2}", start_date + chrono::Duration::days(i as i64), "EMA", ema_val);
        });
    }

    // Wait for all tasks to complete
    tokio::task::yield_now().await;

    let dates = Arc::try_unwrap(dates).unwrap().into_inner().unwrap();
    let closing_prices = Arc::try_unwrap(closing_prices).unwrap().into_inner().unwrap();
    let ema_values = Arc::try_unwrap(ema_values).unwrap().into_inner().unwrap();

    Ok((dates, closing_prices, ema_values))
}