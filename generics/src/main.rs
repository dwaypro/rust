fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![35,36,37,38,39,40];
    // let mut largest = number_list[0]

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    largest(&number_list);
    largest(&number_list2);
}


fn largest(number_list: &[i32]) -> i32{
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    largest
}