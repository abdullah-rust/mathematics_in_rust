

fn main() {

}

// 🧮 1. Decimal Number System (Base 10)
// decimal number aese numbers hote hain jo ham roz use karte hain jese 0 se lekr 9 tak
// is ka base 10 tak hota hai kyon ke yeh total 10 numbers hote hain
// humen mind naturally decimal system main hi sochta hai

fn decimal_example(){
    let num=456;
    println!("this is a decimal number : {}",num);
}

//🧮 2. Binary Number System (Base 2)

// yeh sirf 2 numbers hi hote hain 0 our 1  our yeh sirf isi main hi kaam karte hain

// 1 se (on) 0 se (off)

// cumputers Binary numbers use karte hain kyo ke electronic circuits sirf 2 state samjhte hain
// Voltage hai → 1 (ON)

// Voltage nahi hai → 0 (OFF)

// simple binary to decimal function

fn binary_to_decimal() {
    let binary = 0b1011;
    println!("decimal number is {}", binary);
}

fn binary_to_decimal_2(binary: usize) -> i32 {
    let bin = binary.to_string();
    let move_vec: Vec<i32> = bin
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .rev()
        .collect();
    let mut result = 0;
    for (i, item) in move_vec.iter().enumerate() {
        result += item * to_the_power(2, i as i32);
    }

    result
}

fn to_the_power(base: i32, expo: i32) -> i32 {
    let mut result: i32 = 1;
    for _ in 0..expo {
        result *= base;
    }

    result
}

fn decimal_to_binary(mut num: i32) -> String {
    let mut v: Vec<i32> = Vec::new();
    while num > 0 {
        let bnum = num % 2;
        num = num / 2;
        v.push(bnum);
    }

    let result: String = v
        .iter()
        .rev()
        .map(|c| c.to_string())
        .collect();
    result
}


fn positive_binary_to_negative(bin:&str){
    let binary=format!("0{}",bin);
    let move_vec: Vec<u8> = binary
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect();
    let mut flip:Vec<u8>=Vec::new();
    for i in move_vec.iter() {
        if *i==1 {
           flip.push(0); 
        }else {
            flip.push(1);
        }
    }


let mut result: Vec<u8> = vec![0; flip.len()];
let mut carry = 1;

for (i, bit) in flip.iter().enumerate() {
    let sum = bit + carry;
    result[i] = sum % 2;
    carry = sum / 2;
}


 let s: String = result.iter()
                        .map(|n| n.to_string())
                        .rev()
                        .collect::<String>();
                    println!("{}",s);
}

fn negative_binary_to_positive(bin: &str){
       
    let move_vec: Vec<u8> = bin
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect();
    let mut flip:Vec<u8>=Vec::new();

        for i in move_vec.iter() {
        if *i==1 {
           flip.push(0); 
        }else {
            flip.push(1);
        }
    }

    let mut result: Vec<u8> = vec![0; flip.len()];
let mut carry = 1;

for (i, bit) in flip.iter().enumerate() {
    let sum = bit + carry;
    result[i] = sum % 2;
    carry = sum / 2;
}



 let s: String = result.iter()
                        .map(|n| n.to_string())
                        .rev()
                        .collect::<String>();
                    println!("{}",s);

    
}