
// use iced_test::requester::requester;
use iced_test::lang_tools::*;
use iced::{
    pick_list, scrollable, Align, Container, Element, Length, PickList,
    Sandbox, Scrollable, Settings, Space, Text,
};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    scroll: scrollable::State,
    pick_list: pick_list::State<NameLink>,
    selected_language: NameLink,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected(NameLink),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("WiktiScraper")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = language;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let langs = lang_option();

        let pick_list = PickList::new(
            &mut self.pick_list,
            langs,
            Some(self.selected_language.clone()),
            Message::LanguageSelected,
        );

        let mut content = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Text::new("Choose a Language"))
            .push(pick_list);

        content = content.push(Space::with_height(Length::Units(600)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}