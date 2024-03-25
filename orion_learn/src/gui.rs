use iced::{widget, Alignment, Element, Font, Pixels, Theme,};
use iced::widget::{button, column, text,text_input,Space};
use iced::executor;
use iced::{Application, Command};

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
} 

#[derive(Debug,Clone)]
pub enum Message {
    UserName(String),
    Password(String),
    ENTER,
}
#[derive(Clone,Debug)]
pub struct TextBox {
    user: String,
    pass: String,
    final_username: String,
    final_password: String,
}

impl Application for TextBox {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = executor::Default;


    fn new(_flags: ()) -> (TextBox, Command<Self::Message>) {
        (TextBox {
            user: String::new(),
            pass: String::new(),
            final_username: String::new(),
            final_password: String::new(),
        }, Command::none())
    }
    fn title(&self) -> String {
        String::from("Login")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UserName(a) => self.user = a,
            Message::Password(a) => self.pass = a,
            Message::ENTER => {
                if !self.pass.is_empty() {
                    self.final_password = self.pass.clone();
                    self.final_username = self.user.clone();
                }
            },
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        widget::theme::Theme::Dark
    }


    fn view(&self) -> Element<'_, Self::Message> {
        let user = text_input("Empty", &self.user,)
        .on_input(Message::UserName)
        .padding(10)
        .size(20);
    
        let pass = text_input("", &self.pass,)
        .on_input(Message::Password)
        .padding(10)
        .size(20)
        .secure(true)
        .icon(text_input::Icon { 
            font: Font::default(), 
            code_point: '🔒', 
            size: Some(Pixels(28.0)), 
            spacing: 10.0, side: 
            text_input::Side::Right,
        });
        
        let a = column![
            text("Username").size(18),
            user,
            text("Password").size(18),
            pass,
            Space::new(0, 10),
            button("Confirm").on_press(Message::ENTER),
            Space::new(0,10),
            text(format!("Password: {} ",self.final_password)).size(18),
            text(format!("Username: {} ",self.final_username)).size(18),

        ]
        .padding(10)
        .align_items(Alignment::Start);
    
        a.into()
    }
}