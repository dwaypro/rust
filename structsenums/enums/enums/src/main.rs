// use std::option::Option;

enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn route(ip_type: IpAddrKind){

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
}
