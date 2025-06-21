fn main() {
    let week: Vec<Day> = vec![
        Day {
            name: "Monday".to_string(),
            value: 35,
        },
        Day {
            name: "Tuesday".to_string(),
            value: 37,
        },
        Day {
            name: "Wednesday".to_string(),
            value: 39,
        },
        Day {
            name: "Thursday".to_string(),
            value: 36,
        },
        Day {
            name: "Friday".to_string(),
            value: 40,
        },
        Day {
            name: "Saturday".to_string(),
            value: 38,
        },
        Day {
            name: "Sunday".to_string(),
            value: 34,
        },
    ];
    let dummy = vec![Day {
        name: "Monday".to_string(),
        value: 35,
    }];
    let n = week.len();
    let mut max = Dis {
        name: &dummy,
        max: 0,
    };
    for i in 0..n {
        for j in i..n {
            let sub = &week[i..=j];
            let total: &i32 = &sub.iter().map(|d| d.value).sum();

            if max.max < *total {
                max = Dis {
                    name: sub,
                    max: *total,
                };
            }
            //    println!("name : {:?} {}",sub,total);
        }
    }

    println!("{:?}", max);
}

#[derive(Debug)]
struct Day {
    name: String,
    value: i32,
}

#[derive(Debug)]
struct Dis<'a> {
    name: &'a [Day],
    max: i32,
}

// kadane's algorithem

fn main() {
    use std::cmp;

    let arr = [-1]; // try with [-1]
    let n = arr.len();

    let mut max = arr[0]; // NOT 0
    let mut cs = arr[0];

    for i in 1..n {
        cs = arr;
        max = cmp::max(max, cs);
    }

    max
}

// simple  brute force

fn main() {
    let arr = [-1];
    let n = arr.len();

    let mut max = arr[0];

    for i in 1..n {
        //    for j in i..n {
        let sub = &arr[i..=j];
        let total: i32 = sub.iter().sum();
        if max < total {
            max = total
        }
        println!("sub {:?} and sum {}", sub, total);
    }

    max
}
