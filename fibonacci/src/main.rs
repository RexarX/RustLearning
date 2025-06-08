use std::io;

fn get_nth_fibonacci(n: i32) -> Option<i64> {
    if n < 1 {
        return None;
    }

    let (mut a, mut b) = (0_i64, 1_i64);
    for _ in 2..=n {
        match a.checked_add(b) {
            Some(sum) => {
                a = b;
                b = sum;
            }
            None => return None, // Overflow occurred
        }
    }
    Some(b)
}

fn main() {
    println!("Enter number or 'quit' to exit.");

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read the number");
                continue;
            }
        };

        match get_nth_fibonacci(num) {
            Some(fib) => {
                println!("{}-th fibfibonacci number: {}", num, fib);
                break;
            }
            None => {
                println!("Failed to get {}-th fibfibonacci number", num);
                continue;
            }
        };
    }
}
