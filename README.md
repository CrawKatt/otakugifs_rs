# otakugifs_rs
Unofficial API Wrapper for otakugifs API

# Usage

## Get a random GIF from the OtakuGifs API
```rust
// Get a random GIF from the OtakuGifs API from console input
async fn get_gif_with_wrapper() {
    let api_wrapper = OtakuGifsApi::new();
    let mut reaction = String::new();
    println!("Enter a reaction:");
    io::stdin()
        .read_line(&mut reaction)
        .expect("Failed to read line");
    let reaction = reaction.trim();
    let gif_url = api_wrapper.fetch_random_gif(&reaction).await.unwrap();
    println!("Random {} GIF URL: {:#?}", reaction, gif_url);
}
```

## Get all reactions from the OtakuGifs API
```rust
// Get all reactions from the OtakuGifs API
async fn get_all_reactions_with_wrapper() {
    let api_wrapper = OtakuGifsApi::new();
    let reactions = api_wrapper.fetch_all_reactions().await.unwrap();
    println!("All reactions: {:#?}", reactions);
}
```
