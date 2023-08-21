use pulldown_cmark::{html, Parser};
use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    let blog = warp::path!("posts" / String).map(|post_slug: String| {
        let file_path = format!("content/posts/{}.md", post_slug);
        let markdown_content =
            fs::read_to_string(&file_path).unwrap_or_else(|_| String::from("Post not found"));

        let parser = Parser::new(&markdown_content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        warp::reply::html(html_output)
    });

    warp::serve(blog).run(([127, 0, 0, 1], 3030)).await;
}
