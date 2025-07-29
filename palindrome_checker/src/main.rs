fn main() {
    let number = 12321;
    
    if number_is_palindrome(number) 
    {
        println!("Number is a palindrome");
    } else {
        println!("Number is not a palindrome");
    }
}

fn number_is_palindrome(mut number: u32) -> bool {
    let original = number;
    let mut reversed = 0;
    
    while number > 0 {
        let digits = number % 10;
        reversed = reversed * 10 + digits;
        number /= 10;
    }
    
    return original == reversed;
}