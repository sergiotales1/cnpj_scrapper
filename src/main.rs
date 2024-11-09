use csv::Writer;
use std::error::Error;
use std::fs::File;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://cnpj.biz/empresas/recife-pe?id=63091457")
        .await?;

    let cnpj_elems = driver
        .find_all(By::Css("p.flex.items-center.text-sm.text-gray-500"))
        .await?;

    let mut wtr = Writer::from_path("cnpj.csv")?;

    for cnpj_elem in cnpj_elems {
        let cnpj_value = cnpj_elem.text().await?;

        let clean_cnpj: String = cnpj_value.chars().filter(|c| c.is_digit(10)).collect();

        if clean_cnpj.len() == 14 {
            wtr.write_record(&[clean_cnpj])?;
        }
    }

    wtr.flush()?;

    // Quit the driver
    driver.quit().await?;

    let file = File::open("cnpj.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut wtr = Writer::from_path("emails.csv")?;
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

            let contains_at = email_value.contains('@');

            if contains_at {
                wtr.write_record(&[email_value])?;
            }
        }

        wtr.flush()?;
        driver.quit().await?;
    }

    Ok(())
}
