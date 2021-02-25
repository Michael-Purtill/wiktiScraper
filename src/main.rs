// use iced_test::requester::requester;
use iced::{
    executor, pick_list, scrollable, Align, Application, Command, Container, Element, Length,
    PickList, Scrollable, Settings, Space, Text,
};
use iced_test::lang_tools::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub fn main() -> iced::Result {
    // get_page_content(get_section_by_lang(&"https://en.wiktionary.org/wiki/pes".to_string(), &"Czech".to_string()));
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    scroll: scrollable::State,
    pick_list: pick_list::State<NameLink>,
    selected_language: NameLink,
    lang_list: Vec<NameLink>,
    contents: Vec<section>,
}

#[derive(Debug, Clone)]
enum Message {
    LanguageSelected(NameLink),
    Loaded(Vec<NameLink>),
}

impl Application for Example {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Example, Command<Self::Message>) {
        (
            Self::default(),
            Command::perform(lang_option(), Message::Loaded),
        )
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

        let page_sections = get_page_content(get_section_by_lang(
            &"https://en.wiktionary.org/wiki/pes".to_string(),
            &"Czech".to_string(),
        ));

        for s in page_sections {
            let doc = Document::from(s.content.as_str());

            let node = doc.nth(2).unwrap();

            // if s.name == "Noun[edit]" {
            //     println!("{}", node.name().unwrap());
            // }

            for n in node.children() {
                if n.name().unwrap() == "p" {
                    let header = Text::new(s.name.to_string());
                    let value = Text::new(node.text());

                    content = content.push(header).push(value);
                }

                if n.name().unwrap() == "ul" || node.name().unwrap() == "ol" {
                    //should loop over the node's children here, but get entire text content for now
                    let header = Text::new(s.name.to_string());
                    let value = Text::new(node.text());

                    content = content.push(header).push(value);
                }
            }
        }

        content = content.push(Space::with_height(Length::Units(600)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
