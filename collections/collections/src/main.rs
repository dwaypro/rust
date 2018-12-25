
fn main() {

    //creating a vector. Rust doesn't know what types we intend to 
    // store. Vect<T> can hold any type.
    // let v: Vec<i32> = Vec::new();

    // initial values help determine the type of the vector upon instantiation
    // let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    // v.pop();
    println!("v ===> {:#?}", v);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is{}", third),
        None => println!("There is no third element."),
    }

    let mut v2 = vec![1,2,3,4,5];
    // it's important to note that refrences to the vector
    // will cause a compile error, if the array is manipulated afterwards.
    // this wont cause a compile error:
    v2.push(6);
    let first = &v2[0];
    // this will:
    // let first = &v2[0];
    // v2.push(6);
    println!("the first element is: {}", first);

    let v3 = vec![100,32,57];
    for i in &v3{
        println!("element ==> {}", i);
    }

    let mut v4 = vec![100,32,57];
    for i in &mut v4{
        // to change the value we have to dereference using 
        // the operator *
        // *i += 50;
        println!("mutable element ==> {}", i)
    };

    // when we need to store elements of a diferent type in a vector
    // we can define and use an enum
#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),    
};

let mut row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

for i in &mut row{

    println!("mutable element enum == {:#?}", i);
}

}
