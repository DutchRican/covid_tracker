use crate::Country;
use thousands::Separable;

use cli_table::{format::Justify, Cell, Color, Style, Table, TableStruct};

pub fn convert_row_items_for_table(row_items: &[Country]) -> TableStruct {
    row_items
        .iter()
        .map(|row| {
            vec![
                row.name.as_str().cell().justify(Justify::Center),
                row.total_cases
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right),
                row.new_cases
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right),
                row.total_deaths
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right)
                    .foreground_color(Some(Color::Red)),
                row.new_deaths
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right),
                row.total_recovered
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right)
                    .foreground_color(Some(Color::Green)),
                row.new_recovered
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right),
                row.active_cases
                    .separate_with_commas()
                    .cell()
                    .justify(Justify::Right),
                // row.serious_critical
                //     .to_string()
                //     .cell()
                //     .justify(Justify::Right),
                // row.cases_per_mil.to_string().cell().justify(Justify::Right),
                // row.deaths_per_mil
                //     .to_string()
                //     .cell()
                //     .justify(Justify::Right),
                row.total_tests.separate_with_commas().cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec![
            "Country".cell().justify(Justify::Center).bold(true),
            "Total Cases".cell().bold(true),
            "New Cases".cell().bold(true),
            "Total Deaths".cell().bold(true),
            "New Deaths".cell().bold(true),
            "Total Recovered".cell().bold(true),
            "New Recovered".cell().bold(true),
            "Active Cases".cell().bold(true),
            // "Critical Cases".cell().bold(true),
            // "Cases / MM".cell().bold(true),
            // "Deaths / MM".cell().bold(true),
            "Total Tests".cell().bold(true),
        ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_for_vec_country() {
        let list = vec![Country {active_cases:33, name: String::from("Test Country"), 
        total_cases: 5550, new_cases: 15, total_deaths: 1, new_deaths: 1, total_recovered: 44, new_recovered: 33, serious_critical: 0, 
        cases_per_mil: 231, deaths_per_mil: 10, total_tests: 4000 }];

        let act = convert_row_items_for_table(&list);

        assert_eq!(act.display().unwrap().to_string().contains("Test Country"), true);
        assert_eq!(act.display().unwrap().to_string().contains("5,550"), true);
    }
}
