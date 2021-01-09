use crate::requester::requester;
use scraper::{Html, Selector};

pub fn lang_option() -> String {
    let url = String::from("https://en.wiktionary.org/wiki/Wiktionary:List_of_languages");

    let res = requester(url);

    let doc = Html::parse_document(&res);

    let table_selector = Selector::parse(".wikitable").unwrap(); //there are multiple tables so need to figure out how to get the first.

    let tables = doc.select(&table_selector).next().unwrap();

    return tables.html();

}