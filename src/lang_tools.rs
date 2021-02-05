use crate::requester::requester;
use scraper::{Html, Selector};
use std::fmt;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[derive(Debug, Clone, Eq)]
pub struct NameLink {
    pub name: String,
    pub link: String,
}

impl Default for NameLink {
    fn default() -> NameLink {
        NameLink {name: "".to_string(), link: "".to_string()}
    }
}

impl PartialEq for NameLink {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.link == other.link
    }
}

impl fmt::Display for NameLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub async fn lang_option() -> Vec<NameLink> {
    let mut lang_vec = Vec::new();

    let url = String::from("https://en.wiktionary.org/wiki/Wiktionary:List_of_languages");

    let res = requester(&url);

    let doc = Html::parse_document(&res);

    let table_selector = Selector::parse(".wikitable").unwrap(); //there are multiple tables so need to figure out how to get the first.
    let tbody_selector = Selector::parse("tbody").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let tables = doc.select(&table_selector).next().unwrap();
    let tbody = tables.select(&tbody_selector).next().unwrap();
    let trs = tbody.select(&tr_selector);

    for el in trs {
        let tds: Vec<_> = el.select(&td_selector).collect(); 

        if tds.len() < 1 {
            continue;
        }

        let real_td = tds[1];

        let anchor = real_td.select(&a_selector).next().unwrap();

        lang_vec.push(NameLink {name: real_td.text().collect::<Vec<_>>().join(""), link: format!("https://en.wiktionary.org{}",anchor.value().attr("href").unwrap())});
        // println!("{}", real_td.text().collect::<Vec<_>>().join(""));
    }

    return lang_vec;
}

pub fn cat_link_2_lemma_link(link: &String) -> String {
    let split_link = link.split("Category:").collect::<Vec<_>>();

    let lang_name = split_link[1];

    let lang_name = lang_name.split("_").collect::<Vec<_>>()[0];

    format!("https://en.wiktionary.org/wiki/Category:{}_lemmas", lang_name)
}

pub fn lemma_link_2_pos_links(url: &String) {
    let res = requester(url);

    let doc = Html::parse_document(&res);

    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let list = doc.select(&ul_selector).next().unwrap();
    let list_items = list.select(&li_selector);
    
    for item in list_items {
        let link = item.select(&a_selector).next().unwrap();

        let href = format!("https://en.wiktionary.org{}", link.value().attr("href").unwrap());

        println!("{}", href);
    }
}

pub fn pos_link_2_word_links(url: &String) {
    let res = requester(url);

    let doc = Html::parse_document(&res);

    let div_selector = Selector::parse("#mw-pages").unwrap();
    let div_content_selector = Selector::parse(".mw-content-ltr").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let div = doc.select(&div_selector).next().unwrap();
    let content = div.select(&div_content_selector).next().unwrap();
    
    let links = content.select(&a_selector);

    for link in links {
        let href = format!("https://en.wiktionary.org{}", link.value().attr("href").unwrap());

        let text = link.text().collect::<Vec<_>>().join("");

        if !text.contains("Appendix:") {
            println!("{}", href);
        }
    }
}

pub fn pos_category_looper(url: &String) {
    pos_link_2_word_links(url);

    let res = requester(url);
    let doc = Html::parse_document(&res);

    let div_selector = Selector::parse("#mw-pages").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let div = doc.select(&div_selector).next().unwrap();

    let links: Vec<_> = div.select(&a_selector).collect();

    let first_two_links = &links[0..2];

    let mut link = if first_two_links[0].text().collect::<Vec<_>>().join("").contains("next page") {
        format!("https://en.wiktionary.org{}", first_two_links[0].value().attr("href").unwrap())
    } else if first_two_links[1].text().collect::<Vec<_>>().join("").contains("next page") {
        format!("https://en.wiktionary.org{}", first_two_links[1].value().attr("href").unwrap())
    } else {
        "no links".to_string()
    };

    while link != "no links" {
        pos_link_2_word_links(&link);

        let res = requester(&link);
        let doc = Html::parse_document(&res);

        let div = doc.select(&div_selector).next().unwrap();

        let links: Vec<_> = div.select(&a_selector).collect();

        let first_two_links = &links[0..2];

        link = if first_two_links[0].text().collect::<Vec<_>>().join("").contains("next page") {
            format!("https://en.wiktionary.org{}", first_two_links[0].value().attr("href").unwrap())
        } else if first_two_links[1].text().collect::<Vec<_>>().join("").contains("next page") {
            format!("https://en.wiktionary.org{}", first_two_links[1].value().attr("href").unwrap())
        } else {
            "no links".to_string()
        };
    }
}

pub fn get_section_by_lang(url: &String, lang: &String) -> String{
    let res = requester(url);
    let doc = Document::from(res.as_str());

    let h2 = doc.find(Attr("id", lang.as_str())).next().unwrap().parent().unwrap();

    let mut my_html = Vec::new();

    my_html.push(h2.html());

    let mut next = h2.next().unwrap();

    loop {
        let mut name = "";
        match next.name() {
            Some(x) => name = x,
            None =>{next = next.next().unwrap(); continue},
        }

        if name == "h2" {
            break;
        }

        my_html.push(next.html());

        next = next.next().unwrap();
    }
    return my_html.join("");
}

pub fn get_headers_by_section(section: String) -> Vec<String> {
    let doc = Document::from(section.as_str());

    // let h2 = doc.find(Attr("id", lang.as_str())).next().unwrap().parent().unwrap();

    let mut headers: Vec<String> = Vec::new();

    for h in doc.find(Name("h3")) {
        headers.push(h.text());
    }

    for h in doc.find(Name("h4")) {
        headers.push(h.text());
    }

    for h in doc.find(Name("h5")) {
        headers.push(h.text());
    }

    println!("{}", headers.join(" "));

    return headers;
}
