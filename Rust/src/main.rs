use std::time;

/* 
    Throw a pair of dice a thousand times
    Count the occurrences of each result
        How many times we got 2?
        How many times we got 3? … 12?
    Finally, show the counts

Use an array of (at least) 11 ints, for counting */

/* fn main(){
    let mut seed : u32 = time_seed();
    let mut counts : [i32; 11] = [0; 11];
    for _ in 0..1000 {
        let (val, new_seed) = rand_int(1, 6, seed);
        seed = new_seed;
        let (val2, new_seed) = rand_int(1, 6, seed);
        seed = new_seed;
        let sum = val + val2;
        counts[(sum - 2) as usize] += 1;

    }
    for i in 0..11 {
        println!("{}: {}", i + 2, counts[i]);
    }
} */

fn time_seed() -> u32 {
    use std::time::SystemTime as st;
    let now = st::now().duration_since(st::UNIX_EPOCH).unwrap();
    now.as_millis() as u32
}
fn rand_int(nmin: i32, nmax: i32, seed: u32) -> (i32, u32) {
    let mut seed : u32 = seed;
    // From "Xorshift RNGs" by George Marsaglia
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    let range = (nmax + 1 - nmin) as u32;
    let val = nmin + (seed % range) as i32;
    (val, seed)
}
/* 
    Write a Rust function
        Given an array of positive integers values...
        How many times does the maximum change?
    Like counting the skyscrapers' tops
        … which can be seen, looking from left to right
    Implement it using the imperative paradigm
 */
fn main() {
    let arr = [2,5,2,8,9,10,1,7];
    let mut max = 0;
    let mut count = 0;
    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
            count += 1;
        }
    }
    println!("{}",count);
}