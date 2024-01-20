use futures::{Stream, StreamExt, task::{Context, Poll}};
use std::pin::Pin;

struct TextStream<'a> {
    tokens: Vec<&'a str>,
}

impl<'a> TextStream<'a> {
    fn new(text: &'a str) -> Self {
        TextStream {
            tokens: text.lines().collect(),
        }
    }
}

impl<'a> Stream for TextStream<'a> {
    type Item = &'a str;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(line) = self.tokens.pop() {
            Poll::Ready(Some(line))
        } else {
            Poll::Ready(None) // End of stream
        }
    }
}


#[tokio::main]
async fn main() {
    let text = "Hello\nworld\nin\nRust";
    let mut stream = TextStream::new(text);

    while let Some(line) = stream.next().await {
        println!("{}", line);
    }
}