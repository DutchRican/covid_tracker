mod conversions;
mod models;

use std::{cmp::min, error::Error};
use crate::models::{country_model::Country, arguments::Arguments};
use clap::Parser;
use scraper::{ElementRef, Html, Selector};

use crate::conversions::conversion;

const URL: &str = "https://www.worldometers.info/coronavirus/";

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = get_parsed_arguments();
    let address_string = String::from("https://www.worldometers.info/coronavirus/");
    let resp = get_raw_text().await?;

    let fragment = Html::parse_fragment(&resp);
    let main_table = Selector::parse("table#main_table_countries_today tbody").unwrap();
    let table_fragment = fragment.select(&main_table).next().unwrap();
    let world_row_selector = Selector::parse("tr.total_row_world:not([data-continent])")
        .expect("Could not find the world row");
    let country_row_selector =
        Selector::parse(r#"tr[style=""]"#).expect("Could not find the country rows");

    let world_rows: Vec<ElementRef> = table_fragment.select(&world_row_selector).collect();
    let rows: Vec<ElementRef> = table_fragment.select(&country_row_selector).collect();

    let mut items = parse_for_countries(rows);
    items.sort_by(|a, b| b.total_cases.cmp(&a.total_cases));

    let row_count: usize = min(args.count.into(), items.len());
    if !args.world_stats {
        let mut temp_items = parse_for_countries(world_rows);
        temp_items.append(&mut items);
        items = temp_items;
    }

    println!(
        "\n\nSource: {} as of {}",
        address_string,
        chrono::Local::now().to_rfc2822()
    );
    let table = conversion::convert_row_items_for_table(&items[0..row_count], args.show_full);
    println!("{}", table.display().unwrap());
    Ok(())
}

async fn get_raw_text() -> Result<String, reqwest::Error> {
    let address_string = String::from(URL);
     reqwest::get(&address_string).await?.text().await
}
fn get_parsed_arguments() -> Arguments {
    Arguments::parse()
}

fn parse_for_countries(raw_data: Vec<ElementRef>) -> Vec<Country> {
    let mut parsed: Vec<Country> = Vec::new();
    for row in raw_data {
        let td_sel = Selector::parse("td").expect("error parsing row");
        let tds: Vec<ElementRef> = row.select(&td_sel).collect();
        parsed.push(Country::build(&tds))
    }
    parsed
}
