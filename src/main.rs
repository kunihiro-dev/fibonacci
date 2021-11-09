fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("fbonacci {} is {}", n, result);
}

fn fibonacci(n : i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
