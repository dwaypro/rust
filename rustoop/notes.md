characteristics of object oriented languages:
    "An Object packages both data and the procedures that
    operate on that data"

1.)    encapsulation that hides implementation details. The implementation
   
   - details of an object aren't accessible to code using that object
    ie private functions and having a public api.
   - This is true in rust by creating a struct and then implementing/attaching methods to that struct, defining a pub keyword so that they are accessible on the main thread. 

2.) Inheritance as a Type SYstem and as Code Sharing
    - Rust doesn't support true inheritance, there isn't really a way to define a struct that inherits from a parent structs fields and methods. 
    - You can use default trait method implementations instead. Any type implementing the same trait will in a sense share code. 
    - This is desirable, as it enable more precision in what is inherited. Rather than simply inheriting everything from a super class, you can implement a single trait instead. 

notes on Rust oop practices. Interesting that Rust supports concepts like encapsulation through defining structs and implementing functions. Rust doesn't support other concepts like Inheritance. Which may be a good thing considering you can always create and use trait method implementations instead. Meaning you don't have to inherit entire superclasses if you only need a single function. Another reason to use Inheritance is for the concept of polymorphism, or being able to use a child type in places of a parent type. Rust uses generics to abstract over different possible types, so in a sense Rust supports something called bounded parametric polymorphism. Inheritance runs the risk of sharing more code than necessary.    