use num::{BigUint, FromPrimitive, One};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, process::exit};

fn main() {
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

    for i in 0..num_threads {
        // Calculate the start and end of the chunk
        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 {
            factorial
        } else {
            (i + 1) * chunk_size
        };

        // Clone progress counter
        let progress_clone = Arc::clone(&progress);

        // Spawn a thread for the chunk of work
        let handle = thread::spawn(move || {
            // create initial result for chunk
            let mut result = BigUint::one();

            // Lock the progress counter within the Mutex
            let mut progress = progress_clone.lock().unwrap();

            for j in start..=end {
                // Multiply the result by j
                result *= BigUint::from_usize(j).unwrap();

                // Update the progress counter within the Mutex
                *progress += 1;

                // Print progress
                if *progress % 1000 == 0 {
                    println!("{}/{}", *progress, factorial);
                }
            }
            result
        });

        handles.push(handle);
    }

    // Wait for all threads to finish and collect their results
    for (i, handle) in handles.into_iter().enumerate() {
        results.push(handle.join().unwrap());
        println!("Thread {} finished", i);
    }

    // Combine the results from each thread into a final result
    println!("Combining results from threads...");
    let mut final_result = BigUint::one();
    for result in results {
        final_result *= result;
    }

    // Write the final result to a file
    let final_result_str = final_result.to_string();
    println!("Writing {}B to data.txt...", final_result_str.len());
    let file = File::create("data.txt").unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(final_result_str.as_bytes()).unwrap();
    writer.flush().unwrap(); // Ensure that all buffered data is written

    println!("Factorial written to data.txt");
}
