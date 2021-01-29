
// use iced_test::requester::requester;
use iced_test::lang_tools::*;
use iced::{
    pick_list, scrollable, Align, Container, Element, Length, PickList,
    Scrollable, Settings, Space, Text, Application, Command, executor,
};

pub fn main() -> iced::Result {
    // get_section_by_lang(&"https://en.wiktionary.org/wiki/pes".to_string(), &"Czech".to_string());
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    scroll: scrollable::State,
    pick_list: pick_list::State<NameLink>,
    selected_language: NameLink,
    lang_list: Vec::<NameLink>,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected(NameLink),
    Loaded(Vec::<NameLink>),
}

impl Application for Example {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Example, Command<Self::Message>){
        (Self::default(), Command::perform(lang_option(), Message::Loaded))
    }

    fn title(&self) -> String {
        String::from("WiktiScraper")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::LanguageSelected(language) => {
                self.selected_language = language;
            }

            Message::Loaded(list) => {
                self.lang_list = list;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        // let langs = lang_option();

        let pick_list = PickList::new(
            &mut self.pick_list,
            self.lang_list.clone(),
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