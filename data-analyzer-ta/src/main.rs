// main.rs

mod sequential_reader_csv;
mod par_reader_csv;
mod indicator;
mod data_item;
mod traits;
mod errors;

use plotters::prelude::*;
use std::{env, process, time::Instant};
use chrono::{naive::NaiveDateDaysIterator, NaiveDate};
use par_reader_csv::read_and_process_csv_parallel;
use sequential_reader_csv::read_and_process_csv_sequential;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print the current directory
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    let file_path = "./examples/data/superlarge_dataset.csv";
    let start_date_par = NaiveDate::from_ymd(0, 1, 1); 

    let start = Instant::now();

        println!("Reading and processing the CSV file in parallel...");
     
        let (par_dates, par_closing_prices, par_ema_values) = 
        read_and_process_csv_parallel(file_path, start_date_par, 1024  as usize)?;
    let duration_parallel = start.elapsed();

    // Sequential processing with timing
    let start_date_seq = NaiveDate::from_ymd(0, 1, 1);
    let start = Instant::now();
        println!("Reading and processing the CSV file sequentially...");
        let (seq_dates, seq_closing_prices, seq_ema_values) =
        read_and_process_csv_sequential(file_path, start_date_seq)?;
    let duration_sequential = start.elapsed();


    println!("Sequential Results:");
    println!("Dates: {}", seq_dates.len());
    println!("Closing Prices: {}", seq_closing_prices.len());
    println!("EMA Values: {}", seq_ema_values.len());

    println!("Parallel Results:");
    println!("Dates: {}", par_dates.len());
    println!("Closing Prices: {}", par_closing_prices.len());
    println!("EMA Values: {}", par_ema_values.len());

    // // Print the duration of each process
    println!("Sequential processing took: {:?}", duration_sequential);
    println!("Parallel processing took: {:?}", duration_parallel);
    
    // Plotting graphs for small datasets 

    // plot_ema(&seq_dates, &seq_closing_prices, &seq_ema_values)?;
    // par_plot_ema(&par_dates, &par_closing_prices, &par_ema_values)?;
    
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



fn par_plot_ema(dates: &[String], closing_prices: &[f64], ema_values: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("plotted_image_par.png", (50000, 3500)).into_drawing_area();
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



