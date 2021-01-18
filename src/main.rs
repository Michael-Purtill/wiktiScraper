
// use iced_test::requester::requester;
use iced_test::lang_tools::*;
use iced::{
    button, executor, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput,
};
// use scraper::{Html, Selector};

// fn main() {
//     pos_category_looper(&"https://en.wiktionary.org/wiki/Category:Czech_adverbs".to_string());
// }



fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("WiktiScraper")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Welcome to WiktiScraper").width(Length::Fill)
        .size(100)
        .color([0.5, 0.5, 0.5])
        .horizontal_alignment(HorizontalAlignment::Center).into()
    }
}