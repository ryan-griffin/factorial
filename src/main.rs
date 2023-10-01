use log::{info, LevelFilter};
use num::{BigUint, FromPrimitive, One};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, process::exit};

fn main() {
    // Initialize logger
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    // Parse command line arguments for FACTORIAL and NUM_THREADS
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <FACTORIAL> <NUM_THREADS>", args[0]);
        exit(1);
    }

    let factorial = args[1]
        .parse::<usize>()
        .expect("Invalid FACTORIAL argument");
    let num_threads = args[2]
        .parse::<usize>()
        .expect("Invalid NUM_THREADS argument");

    // Divide the work into chunks for each thread
    let chunk_size = factorial / num_threads;

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Create a vector to hold the results from each thread
    let mut results = vec![];

    // Create a shared progress counter using an Arc (atomic reference counter) and a Mutex
    let progress = Arc::new(Mutex::new(0));

    for i in 1..=num_threads {
        // Calculate the start and end of the chunk
        let start = (i - 1) * chunk_size + 1;
        let end = if i == num_threads {
            factorial
        } else {
            i * chunk_size
        };

        // Clone progress counter
        let progress_clone = Arc::clone(&progress);

        // Spawn a thread for the chunk of work
        let handle = thread::spawn(move || {
            info!("Thread {} started", i);

            // create initial result for chunk
            let mut result = BigUint::one();

            // Lock the progress counter within the Mutex
            let mut progress = progress_clone.lock().unwrap();

            for j in start..=end {
                // Multiply the result by j
                result *= BigUint::from_usize(j).unwrap();

                // Update the progress counter within the Mutex
                *progress += 1;

                if *progress % 10000 == 0 {
                    info!("{}/{}", *progress, factorial);
                }
            }
            result
        });

        handles.push(handle);
    }

    // Wait for all threads to finish and collect their results
    for (i, handle) in handles.into_iter().enumerate() {
        results.push(handle.join().unwrap());
        info!("Thread {} finished", i + 1);
    }

    info!("Combining results from threads...");

    // Initialize final result
    let mut final_result = BigUint::one();

    // Combine the results from each thread into the final result
    for (i, result) in results.into_iter().enumerate() {
        final_result *= result;
        info!("{}/{}", i + 1, num_threads);
    }

    info!("Final result is {}B", final_result.to_bytes_be().len());
    info!("Converting to string...");

    // Convert final result to string
    let final_result_str = final_result.to_string();

    // Create file name
    let file_name = format!("{}.txt", factorial);

    info!("Writing to {}...", file_name);

    // Create a buffered file writer
    let mut writer = BufWriter::new(File::create(&file_name).unwrap());

    // Write result to the file
    writer.write_all(final_result_str.as_bytes()).unwrap();

    // Ensure that all buffered data is written
    writer.flush().unwrap();

    info!("Factorial written to {}", file_name);
}
