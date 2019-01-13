use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a sald for lunch today", post.content());    
}
