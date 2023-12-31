extern crate csv;
extern crate nalgebra as na;

use std::error::Error;
use std::path::PathBuf;
use std::env;
use std::time::Instant;
use na::DVector;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    // Set up the CSV reader
    let data_path = PathBuf::from(env::var("GITHUB_WORKSPACE")?)
        .join("Data")
        .join("winequality-red.csv");
    let mut rdr = csv::Reader::from_path(data_path)?;
    
    let mut rows: Vec<Vec<f64>> = Vec::new();

    // Read and collect all rows into the rows Vec
    for result in rdr.deserialize() {
        let record: Vec<f64> = result?;
        rows.push(record);
    }

    // Transpose the rows to columns
    let data: Vec<DVector<f64>> = (0..rows[0].len()).map(|i| {
        DVector::from_vec(rows.iter().map(|row| row[i]).collect::<Vec<f64>>())
    }).collect();

    // Compute statistics for each column
    let means: Vec<f64> = data.iter().map(|v| v.mean()).collect();
    let medians: Vec<f64> = data.iter().map(|v| {
        let mut sorted: Vec<f64> = v.clone().data.into();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }).collect();
    let std_devs: Vec<f64> = data.iter().map(|v| {
        let mean = v.mean();
        f64::sqrt(v.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / v.len() as f64)
    }).collect();

    // Print statistics for each column
    for ((mean, median), std_dev) in means.iter().zip(&medians).zip(&std_devs) {
        println!("Mean = {}, Median = {}, Std Dev = {}", mean, median, std_dev);
    }

    // Time taken
    let end_time = start_time.elapsed();
    let time_taken = end_time.as_secs() as f64 + end_time.subsec_millis() as f64 * 0.001;
    println!("\nStatistics generated in {} seconds.", time_taken);

    Ok(())
}
