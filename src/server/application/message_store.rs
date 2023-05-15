pub struct MessageStore {
    authors: Vec<String>,
    texts: Vec<String>,
}

impl MessageStore {
    pub fn new() -> MessageStore {
        MessageStore {
            authors: Vec::new(),
            texts: Vec::new(),
        }
    }

    pub fn add_message(&mut self, author: String, text: String) {
        self.authors.push(author);
        self.texts.push(text);
    }
    pub fn get_authors(&self) -> &Vec<String> {
        &self.authors
    }
    pub fn get_texts(&self) -> &Vec<String> {
        &self.texts
    }
}
