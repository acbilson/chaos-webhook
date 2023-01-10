use reqwest;
use scraper;

fn main() {
    let response = reqwest::blocking::get(
        "https://alexbilson.dev/plants",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let link_selector = scraper::Selector::parse(".fill-list a").unwrap();

    let hrefs = document.select(&link_selector).map(|x| x.value().attr("href").unwrap());

    hrefs
        .zip(1..11)
        .for_each(|(item, _)| read_content(&item));
}

fn read_content(uri: &str) {
    let response = reqwest::blocking::get(uri)
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h1#jump").unwrap();
    let content_selector = scraper::Selector::parse(".e-content p").unwrap();

    let content = document
        .select(&content_selector)
        .map(|x| x.inner_html().to_string())
        .take(3)
        .collect::<Vec<String>>()
        .join("");

    let title = document
        .select(&title_selector)
        .map(|x| x.inner_html().to_string())
        .next()
        .unwrap_or_default();

    println!("{:?}: {:?}", &title, &content);

    //return (title, content);
}
