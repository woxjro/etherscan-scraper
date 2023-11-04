use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct Ticker {
    name: String,
    ticker: String,
    contract_address: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"\(([^)]+)\)").unwrap();
    let name_selector = scraper::Selector::parse("div.hash-tag.text-truncate.fw-medium").unwrap();
    let ticker_selector =
        scraper::Selector::parse("a > div.d-flex.gap-1 > span.text-muted").unwrap();
    let contract_selector =
        scraper::Selector::parse("td > a.d-flex.align-items-center.gap-1.link-dark").unwrap();

    let len = 100;

    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut res = vec![];
    for page in 1..=13 {
        let body = reqwest::blocking::get(format!("https://etherscan.io/tokens?ps={len}&p={page}"))
            .unwrap()
            .text()
            .unwrap();

        let document = scraper::Html::parse_document(&body);

        let names = document.select(&name_selector);
        let contracts = document.select(&contract_selector);
        let tickers = document.select(&ticker_selector);

        names.for_each(|e| a.push(e.text().next().unwrap().to_string()));
        tickers.for_each(|e| {
            b.push(
                re.captures(e.text().next().unwrap())
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_string(),
            )
        });
        contracts.for_each(|e| {
            c.push(
                e.attr("href")
                    .unwrap()
                    .split('/')
                    .last()
                    .unwrap()
                    .to_string(),
            )
        });
    }

    for i in 0..a.len() {
        res.push(Ticker {
            name: a[i].to_owned(),
            ticker: b[i].to_owned(),
            contract_address: c[i].to_owned(),
        });
    }

    let json_string = serde_json::to_string(&res).unwrap();
    println!("{}", json_string);
    Ok(())
}
