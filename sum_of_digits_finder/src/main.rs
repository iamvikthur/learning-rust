fn sum_of_digits(number : u32) -> u32 {
    let mut sum = 0;
    let mut n = number;

    while n > 0 {
        let last_digit = n % 10; // GET THE LAST DIGIT
        sum += last_digit; // ADD IT TO THE SUM
        n /= 10; // REMOVE THE LAST DIGIT
    }

    sum
}

fn main() {
    let number = 0123456789;
    let result = sum_of_digits(number);
    println!("The sum of the digits of {} is {}", number, result);
}
