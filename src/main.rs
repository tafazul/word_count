
use std::io;

struct WordCounter {
    text: String
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    // Prompt the user for text input
    println!("Please Enter a text:");

    // Read user input from the console
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // Create a WordCounter instance using the input text
    let word_counter = WordCounter::new(&user_input);

    // Call the count_words function and print the word count to the screen
    let count = word_counter.count_words();
    println!("Word Count: {}", count);

}
