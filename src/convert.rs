use std::{fs::File, io::Read};

use scraper::{ElementRef, Html, Node, Selector};

pub fn read_html_file(file: String) -> Html {
    let mut file_contents = File::open(file).expect("Unable to open the file.");
    let mut html = String::new();
    file_contents
        .read_to_string(&mut html)
        .expect("Error reading file.");
    Html::parse_document(&html)
}

pub fn process(html: Html) {
    let body_selector = Selector::parse("body").unwrap();

    for e in html.select(&body_selector) {
        process_ele(e, false)
    }
}

fn process_ele(element: ElementRef, child: bool) {
    if !child {
        println!("Element: {:?}", element);
    } else {
        println!("Child: {:?}", element);
        println!("  text: {:?}", element.text());

    }
    for c in element.child_elements() {
        println!("  Child: {:?}", c);
        process_ele(c, true);
    }
}
