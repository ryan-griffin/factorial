use num::{BigUint, FromPrimitive, One};
use std::fs::File;
use std::io::Write;
use std::thread;

const FACTORIAL: usize = 1000000;
const NUM_THREADS: usize = 8; // Number of threads to use

fn main() {
    // Create a vector to hold the results from each thread
    let mut results = vec![];

    // Divide the work into chunks for each thread
    let chunk_size = FACTORIAL / NUM_THREADS;

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..NUM_THREADS {
        let start = i * chunk_size + 1;
        let end = if i == NUM_THREADS - 1 {
            FACTORIAL
        } else {
            (i + 1) * chunk_size
        };

        // Clone variables needed in the thread
        let mut result = BigUint::one();
        let start_clone = start;
        let end_clone = end;

        // Spawn a thread for the chunk of work
        let handle = thread::spawn(move || {
            for j in start_clone..=end_clone {
                result *= BigUint::from_usize(j).unwrap();
            }
            result
        });

        handles.push(handle);
    }

    // Wait for all threads to finish and collect their results
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Combine the results from each thread into a final result
    let mut final_result = BigUint::one();
    for result in results {
        final_result *= result;
    }

    // Write the final result to a file
    File::create("data.txt")
        .unwrap()
        .write(final_result.to_string().as_bytes())
        .unwrap();
}
