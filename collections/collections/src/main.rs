
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

let mut s = String::new();
// any type that implements the Display trait, can have to_string
// called on it.

let data = "initial conetnts";
let s = data.to_string();
let s2 = "Initial contents".to_string();

let s3 = String::from("initial contents");


println!("S {}", s);

let mut s4 = String::from("foo");
s4.push_str("bar");

println!("S {}", s4);

let mut s5 = String::from("foo");
let s6 = "bar";
s5.push_str(s6);
println!("s5 is {}", s5);

let mut string = String::from("lo");
string.push('l');
println!("string ==>' {}", string);

let string1 = String::from("hello, ");
let string2 = String::from("world!");
let string3 = string1 + &string2;

println!("String 3 ==> {}", string3);

let string4 = String::from("tic");
let string5 = String::from("tac");
let string6 = String::from("toe");

// let string7 = string4 + "-" + &string5 + "-" + &string6;
// println!("string7 ==> {}", string7);

//we can use format to accomplish the same thing, but cleaner.
let string8 = format!("{}-{}-{}", string4, string5, string6);
println!("string8 ==> {}", string8);


let len = String::from("Hola").len();

let len = String::from("something cryptic").len();

//accessing characters inside strings can be complicated and is not
// recommended.
// however we ca iterate over strings.
let string9 = String::from("suh dude");

for c in string9.chars(){
    println!("{}", c);
}

//we can iterate over bytes as well:

let string10 = String::from("suh dudes");

for b in string10.bytes(){
    println!("byte value ==> {}", b);
}






}