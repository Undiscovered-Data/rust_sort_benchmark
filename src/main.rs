use sorting_rs::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use io::SeekFrom;
use std::time::Instant;

fn make_vec() -> Vec<u64> {
    let mut f = File::open("../../pi_billion.txt").expect("No1");
    let mut start = 10;
    let count = 131072;

    let mut return_vec = Vec::new();
    for _ in 0..1 {

        f.seek(SeekFrom::Start(start)).expect("No2");
        let mut buf = vec![0; count];
        f.read_exact(&mut buf).expect("No3");

        let mut the_mul: u64 = 1_000_000_000_000_000;
        let mut the_res: u64 = 0;
        //let mut return_vec = Vec::new();
        for the_byte in &buf {
            if the_mul == 1 {
                return_vec.push(the_res);
                the_mul = 1_000_000_000_000_000;
                the_res = 0;
            }
            let ascii_num = the_byte - 48;
            let ascii2 = ascii_num as u64;
            let the_add = ascii2 * the_mul;
            the_res += the_add;
            the_mul /= 10;

        }
        start += 131072;

    }

    return_vec
}

fn bingo(myvec: &mut [u64]) {
    bingo_sort(myvec);
    //println!("{:?}", myvec); 
}


fn bubble(myvec: &mut [u64]) {
    bubble_sort(myvec);
    //println!("{:?}", myvec); 
}


fn cocktail(myvec: &mut [u64]) {
    cocktail_sort(myvec);
    //println!("{:?}", myvec); 
}

fn comb(myvec: &mut [u64]) {
    comb_sort(myvec);
    //println!("{:?}", myvec); 
}

fn cycle(myvec: &mut [u64]) {
    cycle_sort(myvec);
    //println!("{:?}", myvec); 
}

fn gnome(myvec: &mut [u64]) {
    gnome_sort(myvec);
    //println!("{:?}", myvec); 
}

fn gnome_up(myvec: &mut [u64]) {
    gnome_up_sort(myvec);
    //println!("{:?}", myvec); 
}

fn heap(myvec: &mut [u64]) {
    heap_sort(myvec);
    //println!("{:?}", myvec); 
}

fn heap_bottom(myvec: &mut [u64]) {
    heap_bottom_up_sort(myvec);
    //println!("{:?}", myvec); 
}

fn heap_weak(myvec: &mut [u64]) {
    weak_heap_sort(myvec);
    //println!("{:?}", myvec); 
}

fn nheap(myvec: &mut [u64]) {
    nheap_sort(myvec);
    //println!("{:?}", myvec); 
}

fn insertion(myvec: &mut [u64]) {
    insertion_sort(myvec);
    //println!("{:?}", myvec); 
}

fn merge(myvec: &mut [u64]) {
    merge_sort(myvec);
    //println!("{:?}", myvec); 
}

fn merge_up(myvec: &mut [u64]) {
    merge_bottom_up_sort(myvec);
    //println!("{:?}", myvec); 
}

fn oddeven(myvec: &mut [u64]) {
    oddeven_sort(myvec);
    //println!("{:?}", myvec); 
}

fn pancake(myvec: &mut [u64]) {
    pancake_sort(myvec);
    //println!("{:?}", myvec); 
}

fn quick(myvec: &mut [u64]) {
    quick_sort(myvec);
    //println!("{:?}", myvec); 
}

fn quick_dual(myvec: &mut [u64]) {
    quick_dual_sort(myvec);
    //println!("{:?}", myvec); 
}

fn selection(myvec: &mut [u64]) {
    selection_sort(myvec);
    //println!("{:?}", myvec); 
}

fn selection_double(myvec: &mut [u64]) {
    selection_double_sort(myvec);
    //println!("{:?}", myvec); 
}

fn shell(myvec: &mut [u64]) {
    shell_sort(myvec);
    //println!("{:?}", myvec); 
}
/*
fn slow(myvec: &mut [u64]) {
    slow_sort(myvec);
    //println!("{:?}", myvec); 
}
*/
fn smooth(myvec: &mut [u64]) {
    smooth_sort(myvec);
    //println!("{:?}", myvec); 
}
/*
fn stooge(myvec: &mut [u64]) {
    stooge_sort(myvec);
    //println!("{:?}", myvec); 
}
*/

fn main() {
    let yayo1 = make_vec();

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    bingo(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Bingo took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    bubble(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Bubble took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    cocktail(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Cocktail took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    comb(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Comb took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    cycle(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Cycle took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    gnome(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Gnome took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    gnome_up(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Gnome_up took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    heap(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Heap took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    heap_bottom(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Heap_bottom took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    heap_weak(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Heap_weak took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    nheap(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Nheap took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    insertion(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Insertion took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    merge(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Merge took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    merge_up(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Merge_up took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    oddeven(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Oddeven took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    pancake(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Pancake took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    quick(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Quick took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    quick_dual(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Quick_dual took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    selection(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Selection took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    selection_double(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Selection_double took {} seconds", elapsed_time.as_secs());

    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    shell(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Shell took {} seconds", elapsed_time.as_secs());
/*
    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    slow(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Slow took {} seconds", elapsed_time.as_secs());
*/
    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    smooth(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Smooth took {} seconds", elapsed_time.as_secs());
/*
    let mut yayo = yayo1.to_vec();
    let now = Instant::now();
    stooge(&mut yayo);
    let elapsed_time = now.elapsed();
    println!("Stoge took {} seconds", elapsed_time.as_secs());
*/

}
