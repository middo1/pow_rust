fn main() {
    println!("Hello, world!");
    println!("{}", my_pow(34.00515, -3))
}
fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n < 0 && n % 2 == 0{
        return my_pow(x * x, n/2);
    } else if n < 0 && n % 2 == -1{
        if n == -1 {
            return my_pow(1.0/ x * 1.0 / x, n/2) * 1.0/ x;
        }
        return my_pow(x * x, n/2) * 1.0/x;
    }
    if n % 2 == 0{
        return my_pow(x * x, n/2);
    } else {
        return my_pow(x * x, n/2) * x;
    }
}
