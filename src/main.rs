mod conversions;
mod models;

use clap::Parser;
use std::cmp::min;
use chrono;
use std::error::Error;
use scraper::{ElementRef, Html, Selector};
use conversions::conversion;
use models::arguments::Args;
use models::country_model::Country;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let address_string = String::from("https://www.worldometers.info/coronavirus/");
    let resp = reqwest::get(&address_string)
        .await?
        .text()
        .await?;

    
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

    let row_count:usize = min(args.count.into(), items.len());
    if !args.world_stats {
        let mut temp_items = parse_for_countries(world_rows);
        temp_items.append(&mut items);
        items = temp_items;
    }

    println!("\n\nSource: {} as of {}", address_string, chrono::Local::now().to_rfc2822());
    let table = conversion::convert_row_items_for_table(&items[0..row_count]);
    println!("{}", table.display().unwrap());

    Ok(())
}

fn parse_for_countries(raw_data: Vec<ElementRef>) -> Vec<Country> {
    let mut parsed: Vec<Country> = Vec::new();
    for row in raw_data {
        let td_sel = Selector::parse("td").expect("error parsing row");
        let tds: Vec<ElementRef> = row.select(&td_sel).collect();
        parsed.push(Country::parse_entry(&tds))
    }
    parsed
}
