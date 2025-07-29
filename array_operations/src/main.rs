fn sum_of_elements(array: &[i32]) -> i32 {
    let mut sum = 0;
    let array_length = array.len();

    for i in 0..array_length {
        sum += array[i];
    }

    sum
}

fn max_element(numbers: &[i32]) -> i32 {
    let mut max = numbers[0];
    let array_length = numbers.len();

    for i in 1..array_length {
        if numbers[i] > max {
            max = numbers[i];
        }
    }

    max
}

fn min_element(numbers: &[i32]) -> i32 {
    let mut min = numbers[0];

    for &num in numbers.iter() {
        if num < min {
            min = num;
        }
    }

    min
}

fn average_of_elements(numbers: &[i32]) -> f64 {
    let sum = sum_of_elements(&numbers);
    let count = numbers.len() as f64;

    if count == 0.0 {
        return 0.0;
    }

    sum as f64 / count
}

fn main() {
    let numbers = [10, 5, 8, 12, 7];

    let sum = sum_of_elements(&numbers);
    let max = max_element(&numbers);
    let min = min_element(&numbers);
    let average = average_of_elements(&numbers);

    println!("Sum of eletments: {}", sum);
    println!("Max element: {}", max);
    println!("Min element: {}", min);
    println!("Average of elements: {:.2}", average);
}
