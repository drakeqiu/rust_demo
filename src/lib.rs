pub trait Summary {
    fn sumarize(&self) -> String;
}

pub struct NewsArtical {
    pub headline: String,
    pub localtion: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArtical {
    fn sumarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.localtion)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
