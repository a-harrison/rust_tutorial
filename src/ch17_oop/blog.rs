// use crate::ch17_oop::stateful_post::Post;
use crate::ch17_oop::transition_post::Post;

pub fn main() {
    // Create new, empty Post that defaults into the 'Draft' state.
    let mut post = Post::new();

    // Add text to the Post, still in the 'Draft' state
    post.add_text("I ate a salad for lunch today");

    // // ---- For stateful Posts ------
    // assert_eq!("", post.content());
    //
    // // Request review of the Post, putting it into the 'PendingReview' state.
    // post.request_review();
    // assert_eq!("", post.content());
    //
    // // Post is approved, so '.content()' will now return the text we added.
    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    // ---- For transition Posts ----
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
