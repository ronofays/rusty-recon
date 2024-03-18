use std::collections::HashSet;
use reqwest;
use scraper;
use crate::error::SubdomainError;

pub fn query_crtsh(domain: &str) -> Result<HashSet<String>, SubdomainError> {
    let query_url = format!("https://crt.sh/?q={}", domain);
    let response = reqwest::blocking::get(query_url)?;

    // println!("{}", response.text().unwrap());
    let document = scraper::Html::parse_document(&response.text()?);
    let row_selector = scraper::Selector::parse("table > tbody > tr > td > table > tbody > tr").unwrap();

    // Iterate over each row, skipping the first one
    let mut domains = HashSet::new();
    for row in document.select(&row_selector).skip(1) {
        let cells: Vec<_> = row.select(&scraper::Selector::parse("td").unwrap()).collect();

        // Check if there are enough cells to extract the second to last one
        if cells.len() >= 2 {
            let target_cell = &cells[cells.len() - 2]; // Get the second to last cell
            // Print the text content of the target cell
            target_cell.text().into_iter().for_each(|s| {
                domains.insert(s.to_owned());
        });
        }
    }

    Ok(domains)
}