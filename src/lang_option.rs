use crate::requester::requester;
use scraper::{Html, Selector};

pub struct name_link {
    pub name: String,
    pub link: String,
}

pub fn lang_option() -> String {
    let url = String::from("https://en.wiktionary.org/wiki/Wiktionary:List_of_languages");

    let res = requester(url);

    let doc = Html::parse_document(&res);

    let table_selector = Selector::parse(".wikitable").unwrap(); //there are multiple tables so need to figure out how to get the first.
    let tbody_selector = Selector::parse("tbody").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let tables = doc.select(&table_selector).next().unwrap();
    let tbody = tables.select(&tbody_selector).next().unwrap();
    let trs = tbody.select(&tr_selector);
    
    // let mut lang_vec = Vec::new();

    for el in trs {
        let tds: Vec<_> = el.select(&td_selector).collect();

        if tds.len() < 1 {
            continue;
        }

        let real_td = tds[1];

        // lang_vec.push(real_td.text().collect::<Vec<_>>().join(""));
        println!("{}", real_td.text().collect::<Vec<_>>().join(""));
    }

    return "cool".to_string();

}

pub fn cat_2_lemma_link(link: String) -> String {
    let split_link = link.split("Category:").collect::<Vec<_>>();

    let lang_name = split_link[1];

    let lang_name = lang_name.split("_").collect::<Vec<_>>()[0];

    format!("https://en.wiktionary.org/wiki/Category:{}_lemmas", lang_name)
}