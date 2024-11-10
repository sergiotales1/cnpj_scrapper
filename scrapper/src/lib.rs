use csv::Writer;
use std::error::Error;
use std::fs::{self, File};
use thirtyfour::prelude::*;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let url = "https://cnpj.biz/empresas/recife-pe?id=63091457";

    driver.goto(url).await?;

    let cnpj_elems = driver
        .find_all(By::Css("p.flex.items-center.text-sm.text-gray-500"))
        .await?;

    fs::create_dir("output")?;

    let mut wtr = Writer::from_path("output/cnpj.csv")?;

    for cnpj_elem in cnpj_elems {
        let cnpj_value = cnpj_elem.text().await?;
        let clean_cnpj: String = cnpj_value.chars().filter(|c| c.is_digit(10)).collect();

        if clean_cnpj.len() == 14 {
            wtr.write_record(&[clean_cnpj])?;
        }
    }

    wtr.flush()?;
    driver.quit().await?;
    process_emails().await?;
    Ok(())
}

async fn process_emails() -> Result<(), Box<dyn Error + Send + Sync>> {
    let file = File::open("output/cnpj.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = Writer::from_path("output/emails.csv")?;

    for result in rdr.records() {
        let record = result?;
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        let cnpj = &record[0];
        let url = format!("https://cnpj.biz/{}", cnpj);
        driver.goto(&url).await?;

        let email_elements = driver.find_all(By::Css("b.copy")).await?;
        for email_element in email_elements {
            let email_value = email_element.text().await?;
            if email_value.contains('@') {
                wtr.write_record(&[email_value])?;
            }
        }

        wtr.flush()?;
        driver.quit().await?;
    }
    Ok(())
}
