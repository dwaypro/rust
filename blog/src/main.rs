use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    post.request_review();
    assert_eq!("", post.content());


    println!("post.content ==> {:?}", post.content());
    post.approve();
    println!("post.content after approve==> {:?}", post.content());
    assert_eq!("I ate a sald for lunch today", post.content());    
}
