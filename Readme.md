# DATA PIPELINE INTRO

The purpose of this repository is to experiment with Rust parallelization for data analysis mainly 
using Polars Library and compare againt python pandas doing somewhat of a benchmark.

# SETUP

Data comes from a file in data/foods.parquet which comes from the an example data from polars library.

Fist idea would be to run the file_dup.sh which would duplicate the file 800 times into a .data/directory
Duplicate the basic data to create an scenario where working in parallel makes sense

1) ./data_dup.sh

Run Rust and collect results:
1) cargo run --release  # compiles the project
2) time ./target/release/data_pipeline

Run Python equivalent project and collect results
3) python main.py


# RESULTS

Current results in intel 12th Gen i7 - 1270P - 16 threads.

Python:

real 0m2.362s
user 0m2.328s
sys  0m1.465s

Rust:
real  0m0.167s
user  0m0.191s
sys   0m0.278s

# CONCLUSION

Rust is about 14 times faster than python with these conditions
