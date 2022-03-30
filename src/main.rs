fn main() {
    println!("{}", math(6, 9, 6, 9));
    print!("{}", "Hello, world!");
}

fn math(x: i32, y: i32, z: i32, a: i32) -> i32 {
    return x * y + z + a;
}
