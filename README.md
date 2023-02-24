# rust_sort_benchmark
This is a benchmark to test the speed of the sorting algorythms of sorting_rs.
https://docs.rs/sorting_rs/latest/sorting_rs/
It uses a billion digits of pi to create "random" numbers for sorting benchmarks.
I suggest using wget to download https://stuff.mit.edu/afs/sipb/contrib/pi/pi-billion.txt

On line 11, the_count corresponds to the L1 cache, it may be different on your computer.
On line 14, you can increase the number of repetitions, but you may want to comment out
the slower sorting algorythms. I have already commented out slow and stooge sort.
