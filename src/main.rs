
use iced_test::requester::requester;
use iced_test::lang_option::lang_option;
use scraper::{Html, Selector};

fn main() {
    // let url = String::from("https://en.wiktionary.org/w/index.php?title=Category:Czech_nouns");

    // let res = requester(url);

    // let doc = Html::parse_document(&res);

    println!("{}", lang_option());

    // let pages_selector = Selector::parse("#mw-pages").unwrap();
    // let a_selector = Selector::parse("a").unwrap();

    // for element in doc.select(&pages_selector).next().unwrap().select(&a_selector) {
    //     println!("{}", element.text().collect::<Vec<_>>().join(""));
    // }



}