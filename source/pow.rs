// pow(x^n)  binary exponatial example and why is best bcause hi use time complexity O(log n)

fn main() {
    let mut x = 2.0;
    let mut n = -10;
    let mut result = 1.0;

    if n < 0 {
        n = -n;
        x = 1.0 / x;
    }

    while n > 0 {
        if n % 2 == 1 {
            result *= x;
        }

        x *= x;
        n /= 2;
    }

    println!("{}", result);
}
