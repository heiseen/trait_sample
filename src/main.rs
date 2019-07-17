fn main() {
    let tweet = Tweet {
        name: String::from("cjw"),
        email: String::from("cjw.haha@gmail.com"),
    };

    let news_article = NewsArticle {
        headline: String::from("aaa"),
        author: String::from("bbb"),
    };

    notify(tweet, news_article);
}

pub fn notify<T: Summary>(item1: T, item2: T) {
    println!("item1 : {} ,item2 : {}", item1.summer(), item2.summer());
}

pub trait Summary {
    fn summer(&self) -> String;
}

pub struct NewsArticle {
    headline: String,
    author: String,
}

pub struct Tweet {
    name: String,
    email: String,
}

impl Summary for NewsArticle {
    fn summer(&self) -> String {
        format!("headline -> {} , author -> {}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    fn summer(&self) -> String {
        format!("name -> {} , email -> {}", self.name, self.email)
    }
}










