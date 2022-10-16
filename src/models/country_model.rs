use scraper::ElementRef;

#[derive(Debug)]
pub struct Country {
    pub name: String,
    pub total_cases: i32,
    pub new_cases: i32,
    pub total_deaths: i32,
    pub new_deaths: i32,
    pub total_recovered: i32,
    pub new_recovered: i32,
    pub active_cases: i32,
    pub serious_critical: i32,
    pub cases_per_mil: i32,
    pub deaths_per_mil: i32,
    pub total_tests: i32
}

impl Country {
    pub fn parse_entry(entry: &Vec<ElementRef>) -> Country {
        Country {
            name: entry[1].text().next().unwrap().to_string(),
            total_cases: get_parsed_number(entry[2]),
            new_cases: get_parsed_number(entry[3]),
            total_deaths: get_parsed_number(entry[4]),
            new_deaths: get_parsed_number(entry[5]),
            total_recovered: get_parsed_number(entry[6]),
            new_recovered: get_parsed_number(entry[7]),
            active_cases: get_parsed_number(entry[8]),
            serious_critical: get_parsed_number(entry[9]),
            cases_per_mil: get_parsed_number(entry[10]),
            deaths_per_mil: get_parsed_number(entry[11]),
            total_tests: get_parsed_number(entry[12]),
        }
    }
}

fn get_parsed_number(element: ElementRef) -> i32 {
    let text = match element.text().next() {
        Some(v) => v.to_string().trim().replace(",", ""),
        None => "0".to_string(),
    };

    text.parse().unwrap_or(-1)
}


#[cfg(test)]
mod tests {
    use scraper::{Html, Selector};

    use super::*;
    #[test]
    fn get_actual_number() {
        let raw_html = String::from("<p>55555</P");
        let parsed = Html::parse_fragment(&raw_html);
        let selector = Selector::parse("p").unwrap();
        let element: Vec<ElementRef> = parsed.select(&selector).collect();

        let actual = get_parsed_number(element.first().unwrap().to_owned());

        assert_eq!(actual, 55555);
    }
    
    #[test]
    fn handle_empty_string() {
        let raw_html = String::from("<p>  </P");
        let parsed = Html::parse_fragment(&raw_html);
        let selector = Selector::parse("p").unwrap();
        let element: Vec<ElementRef> = parsed.select(&selector).collect();

        let actual = get_parsed_number(element.first().unwrap().to_owned());

        assert_eq!(actual, -1);
    }
}