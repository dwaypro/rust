// use std::option::Option;

enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}


#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}


enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn route(ip_type: IpAddrKind){

}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    
    route(home);
    
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
     println!("Testerooo {:#?}", absent_number);

     let penny = Coin::Penny;

     let quarter = Coin::Quarter(UsState::Alabama);
     let value = value_in_cents(penny);
     let value2 = value_in_cents(quarter);
     println!("Penny {:#?}", value);   
     println!("Quarter {:#?}", value2); 

     let five = Some(5);
     let six = plus_one(five);
     let none = plus_one(None);
     println!("six ==> {:#?}", none);
}
