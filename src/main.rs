use rand::Rng;
use std::cmp::Ordering;
use iced::widget::{column, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

struct GuessingGame {
    user_input: String,
    secret_number: i32,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Sandbox for GuessingGame {
    type Message = Message;

    fn new() -> Self {
        Self { user_input: String::from(""), secret_number: rand::thread_rng().gen_range(1..=100) }
    }

    fn title(&self) -> String {
        String::from("Guess the Number")
    }

    fn view(&self) -> Element<Message> {
        let mut feedback = "Guess a secret number between 1 and 100";
        let guess: Result<i32, std::num::ParseIntError> = self.user_input.trim().parse();

        feedback = if guess.is_err() && !self.user_input.is_empty() {"You must enter a number" } else { feedback };

        feedback = if guess.is_ok() {
            match guess.unwrap().cmp(&self.secret_number) {
                Ordering::Less => "Too small!",
                Ordering::Greater => "Too big!",
                Ordering::Equal => "You win!",
            }
        } else { feedback };

        column![
            text_input("Enter a guess", &self.user_input, Message::InputChanged),
            text(feedback).size(50),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(user_input) => {
                self.user_input = user_input;
            }
        }
    }
}

fn main() -> iced::Result {
    GuessingGame::run(Settings::default())
}
