use pulldown_cmark::{html, Options, Parser};
use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("posts" / String).map(|post_slug: String| {
        let file_path = format!("posts/{}.md", post_slug);
        let markdown_content =
            fs::read_to_string(&file_path).unwrap_or_else(|_| String::from("Post not found"));

        let parser = pulldown_cmark::Parser::new(&markdown_content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        warp::reply::html(html_output)
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
