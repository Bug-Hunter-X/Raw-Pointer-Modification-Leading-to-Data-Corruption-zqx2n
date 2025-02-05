fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; //Modify vector elements using safe indexing
    println!("Value at index 0: {}", v[0]);
}