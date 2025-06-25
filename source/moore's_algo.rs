// moore's votng algorithm
fn main() {
    let nums = [2, 1, 1, 2];
    let mut count = 0;
    let mut candidate = 0;

    for &num in nums.iter() {
        if count == 0 {
            candidate = num;
            count = 1;
        } else if candidate == num {
            count += 1;
        } else {
            count -= 1;
        }
    }

    return candidate;
}

fn main() {
    let arr = [2, 1, 2];
    let n = arr.len();
    let mut f = 1;
    let mut c = arr[0];
    for i in arr.iter() {
        if c == *i {
            f += 1;
        } else {
            c = *i;
            f -= 1;
        }

        if f > n / 2 {
            break;
        }
    }

    println!("majority is {}", c);
}

// majority array brute force

fn main() {
    let mut arr = [2, 2, 1, 1, 1, 2, 2, 1, 1, 1];
    let n = arr.len();
    arr.sort();

    let mut majority_element = -1;
    for i in 0..n {
        let mut count = 0;

        for j in 0..n {
            if arr[i] == arr[j] {
                count += 1;
            }
        }

        if count > n / 2 {
            majority_element = arr[i];
            break;
        }
    }
    println!("{}", majority_element);
}

// optimize solution  of sum pair

fn main() {
    let arr = [2, 3, 11, 15];
    let target = 13;
    let mut f = 0;
    let mut l = arr.len() - 1;

    loop {
        if arr[f] + arr[l] > target {
            l -= 1;
        } else if arr[f] + arr[l] < target {
            f += 1;
        } else if arr[f] + arr[l] == target {
            println!("first index is {} second index is {}", f, l);
            break;
        }
    }
}

// brute force solution of pair sum

fn main() {
    let arr = [2, 2, 11, 7];
    let n = arr.len();
    let target = 9;
    for i in 0..n {
        for j in i + 1..n {
            if arr[i] + arr[j] == target {
                println!("first index {} second inedex {}", i, j);
                break;
            }
        }
    }
}
