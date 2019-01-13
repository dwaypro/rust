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