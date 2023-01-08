use std::sync::{Arc, Mutex};
use std::thread;
use iced::{button, Align, Application, Button, Column, Command, Container, Element, Length, Settings, Text};

#[derive(Debug, Clone)]
enum Message {
    Play,
}

#[derive(Debug, Clone)]
struct App {
    result: Option<String>,
    button_state: button::State,
}

impl App {
    fn new() -> App {
        App {
            result: None,
            button_state: button::State::new(),
        }
    }

    fn play(&mut self) {
        let n_th = 2;
        let hands = Arc::new(Mutex::new(vec![0; n_th]));
        let mut threads = Vec::new();
        for i in 0..n_th {
            let hands = hands.clone();
            threads.push(thread::spawn(move || {
                let mut hands = hands.lock().unwrap();
                let hand = rand::random::<u8>() % 3;
                hands[i] = hand;
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        let hands = hands.lock().unwrap();
        let hand1 = hands[0];
        let hand2 = hands[1];
        let result = if hand1 == hand2 {
            "Tie!".into()
        } else if (hand1 == 0 && hand2 == 2) ||
                  (hand1 == 1 && hand2 == 0) ||
                  (hand1 == 2 && hand2 == 1) {
            "Player 1 wins!".into()
        } else {
            "Player 2 wins!".into()
        };
        self.result = Some(result);
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        "Rock Paper Scissors".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Play => self.play(),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let result = if let Some(result) = &self.result {
            Text::new(result).size(50)
        } else {
            Text::new("").size(50)
        };

        let play_button = Button::new(&mut self.button_state, Text::new("Play"))
            .on_press(Message::Play)
            .width(Length::Units(100));

        Container::new(
            Column::new()
                .max_width(500)
                .spacing(20)
                .align_items(Align::Center)
                .push(result)
                .push(play_button),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn main() {
    App::run(Settings::default())
}
