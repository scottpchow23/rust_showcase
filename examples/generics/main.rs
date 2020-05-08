fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let int_list = vec![1, 2, 3];
    let largest_int = largest(&int_list);
    let float_list = vec![1.0, 2.0, 3.0];
    let largest_float = largest(&float_list);

    println!("The largest integer was: {}", largest_int);
    println!("The largest float was: {}", largest_float);
}
