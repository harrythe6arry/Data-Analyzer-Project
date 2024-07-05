mod indicator;
mod data_item;
mod errors;
mod traits;

use crate::indicator::ExponentialMovingAverage as Ema;
use crate::data_item::DataItem;
use crate::traits::{Close, Next};
use csv::ReaderBuilder;
use plotters::prelude::*;
use serde::Deserialize;
use std::env;
use std::process;
use std::error::Error;




#[derive(Debug, Deserialize)]
struct Record {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Print the current directory
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

    let mut reader = match ReaderBuilder::new().has_headers(true).from_path("./examples/data/large_dataset.csv") {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    };

    let start_date = chrono::NaiveDate::from_ymd(2024, 1, 1); // Define a start date
    let mut dates = vec![];
    let mut closing_prices = vec![];
    let mut ema_values = vec![];

    for (i, result) in reader.deserialize().enumerate() {
        let record: Record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                process::exit(1);
            }
        };

        // Generate date based on index
        let date = start_date + chrono::Duration::days(i as i64);
        dates.push(date.to_string());
        closing_prices.push(record.close);

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

        let ema_val = ema.next(&dt);
        ema_values.push(ema_val);
        println!("{}: {} = {:2.2}", date, "EMA", ema_val);
    }

    // Plot the data
    plot_ema(&dates, &closing_prices, &ema_values).unwrap();
    Ok(())
}

fn plot_ema(dates: &[String], closing_prices: &[f64], ema_values: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("plotted_image.png", (50000, 3500)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Exponential Moving Average", ("Calibri", 600, &BLACK).into_text_style(&root_area))
        .margin(100)
        .x_label_area_size(70)
        .y_label_area_size(65)
        .build_cartesian_2d(0..dates.len(), *closing_prices.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..*closing_prices
        .iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        dates.iter().enumerate().map(|(idx, _)| (idx, closing_prices[idx])),
        *&RED.stroke_width(3),
    ))?.label("Price Line")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        dates.iter().enumerate().map(|(idx, _)| (idx, ema_values[idx])),
        *&BLUE.stroke_width(3),
    ))?.label("EMA")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
