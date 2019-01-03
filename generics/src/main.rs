fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list2 = vec![35,36,37,38,39,40];



    let p = Point {x:5, y:10};

    // let p1 = Point;

    // println!("p.x = {}", p.x());

    let largestVal = largest(&number_list);

    println!("largest val ==> {}", largestVal);
    largest(&number_list2);
}


// fn largest(number_list: &[i32]) -> i32{
//     let mut largest = number_list[0];

//     for &number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     largest
// }

fn largest_char(list: &[char]) -> char{
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


//generics 
struct Point<T, U>{
    x:T,
    y:U,
}

impl<T, U> Point<T, U>{
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W>{
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



// fn largest_list<T>(list: &[T]) -> T{
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest    
// }


