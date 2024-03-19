use std::collections::HashSet;
use chrono::prelude::*;
use crate::error::SubdomainError;
use crate::util::clean_url;

pub fn query_crtsh(domain: &str) -> Result<HashSet<String>, SubdomainError> {
    let query_url = format!("https://crt.sh/?q={}&output=json", domain);
    let response = reqwest::blocking::get(query_url)?;

    // println!("{}", response.text().unwrap());
    let today = Utc::now().date_naive();
    let document = scraper::Html::parse_document(&response.text()?);
    let row_selector = scraper::Selector::parse("table > tbody > tr > td > table > tbody > tr").unwrap();

    // Iterate over each row, skipping the first one
    let mut domains = HashSet::new();
    for row in document.select(&row_selector).skip(1) {
        let cells: Vec<_> = row.select(&scraper::Selector::parse("td").unwrap()).collect();

        // Check if the cert is still valid
        let date_cell = cells[3].text()
            .collect::<Vec<_>>().
            join("")
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let date = NaiveDate::from_ymd_opt(date_cell[0] as i32, date_cell[1], date_cell[2]).unwrap();
        if date < today {
            continue;
        }
        
        // Add subdomains
        let target_cell = &cells[cells.len() - 2]; // Get the second to last cell
        target_cell.text().into_iter().for_each(|s| {
            domains.insert(clean_url(s).to_owned());
        });
    }

    Ok(domains)
}