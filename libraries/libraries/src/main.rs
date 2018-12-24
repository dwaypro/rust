use std::collections::HashMap;

mod sound{
   pub mod instrument{
        pub fn clarinet(){
            println!("toot");
        }      
    }
}

mod performance_group{
    use crate::sound::instrument;

    pub fn clarinet_trio(){
        instrument::clarinet();
    }
}

use crate::sound::instrument;
mod plant {
    pub struct Vegetable {
        pub name: String,
        pub id: i32,
    }

    impl Vegetable{
        pub fn new(name: &str) -> Vegetable{
            Vegetable{
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu{
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

fn main() {
    println!("Hello, world!");

    //absolue path
    crate::sound::instrument::clarinet();
    // relative path
    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicous", v.name);
    println!("The Id is {}", v.id);

    instrument::clarinet();

    performance_group::clarinet_trio();

    let mut map = HashMap::new();

    map.insert(1,2);
    println!("HashMap ==> {:#?}", map);
    println!("hashMap at 1 ==> {:#?}", map.len())
}
