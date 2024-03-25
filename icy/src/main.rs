mod text_box;

use crate::text_box::TextBox;
use iced::widget::{button, column, text};
use iced::{window, Alignment, Application, Sandbox, Settings};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: iced::Size { width: 600.0f32, height: 260.0f32 },
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    TextBox::run(settings)
    // Incr::run(Settings::default())
} 

#[derive(Default)]
pub struct Incr {
    increment: i32,
}

#[derive(Debug,Clone, Copy)]
pub enum Counter {
    Increment,
    Decrement,
}

impl Sandbox for Incr{
    type Message = Counter;

    fn new() -> Self {
        Self {
           increment : 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Counter::Increment => self.increment += 1,
            Counter::Decrement => self.increment -= 1,

        }   
    }


    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            button("Increment").on_press(Counter::Increment),
            text(self.increment).size(50),
            button("Decrement").on_press(Counter::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()

    }
}

