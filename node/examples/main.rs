//! example

use std::fmt::Debug;
trait Printable: Debug {
    fn print(&self);
}

#[derive(Debug, Default)]
struct Notes {
    content: String,
    length: u128,
}

impl Printable for Notes {
    fn print(&self) {
        println!("content: {}, length: {}", self.content, self.length)
    }
}

#[derive(Debug, Default)]
struct News {
    author: String,
    content: String,
    date: u64,
}

impl Printable for News {
    fn print(&self) {
        println!(
            "author: {}, content: {}, date: {}",
            self.author, self.content, self.date
        )
    }
}

#[derive(Default, Debug)]
struct Feed {
    post: Vec<Box<dyn Printable>>,
}

impl Feed {
    fn insert_all(&mut self, content: Box<dyn Printable>) {
        self.post.push(content);
    }

    fn print_post(&self) {
        self.post.get(0).unwrap().print();
    }
}

fn main() {
    let news: Vec<Box<News>> = (1..10)
        .into_iter()
        .map(|_| Box::new(News::default()))
        .collect();

    let notes: Vec<Box<Notes>> = (1..10)
        .into_iter()
        .map(|_| Box::new(Notes::default()))
        .collect();

    let mut feed = Feed::default();
    for n in news.into_iter() {
        feed.insert_all(n);
    }

    for nts in notes.into_iter() {
        feed.insert_all(nts)
    }
    feed.print_post();
}
