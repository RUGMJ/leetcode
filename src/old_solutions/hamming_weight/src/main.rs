fn main() {
    println!("Hello, world!");
}

#[allow(non_snake_case)]
fn hammingWeight(n: u32) -> i32 {
    n.count_ones() as i32
}
