// optimization

fn main() {
    use std::cmp;
    let prices = [7, 1, 5, 3, 6, 4];

    let mut best_buy = prices[0];

    let mut result = 0;

    for &i in prices.iter() {
        if best_buy > *&i {
            best_buy = *&i;
        }
        if *&i > best_buy {
            result = cmp::max(result, *&i - best_buy);
        }
    }

    println!("best price {}", best_buy);
    println!("max is {}", result);
}

// brute force
fn main() {
    let prices = [7, 1, 5, 3, 6, 4];
    let n = prices.len();
    let mut best_buy = prices[0];
    let mut ind: i32 = 0;
    let mut result: i32 = 0;

    for (index, i) in prices.iter().enumerate() {
        if best_buy >= *i {
            best_buy = *i;
            ind = index as i32;
        }

        for j in index + 1..n {
            if result < prices[j] {
                result = prices[j];
            }
        }
    }

    result = result - best_buy;
    println!("best price {} and day {}", best_buy, ind);
    println!("max is {}", result);
}
