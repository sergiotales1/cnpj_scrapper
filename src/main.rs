use dotenv::dotenv;
use lettre::message::SinglePart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // scrapper::run().await?;
    dotenv().ok();

    let mut html_file = File::open("template/index.html")?; // Replace with your HTML file path
    let mut html_content = String::new();
    html_file.read_to_string(&mut html_content)?;

    let personalized_content = html_content.replace("[nome da empresa]", "Sergio Tales");

    let sender_mail = "sergiotalesdev@gmail.com";
    let receiver_mail = "algumemailai3@gmail.com";

    let smtp_server = "smtp.sendgrid.net";
    let smtp_username = "apikey";
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not found in environment");

    let email = lettre::Message::builder()
        .from(sender_mail.parse()?)
        .to(receiver_mail.parse()?)
        .subject("Hello")
        .singlepart(SinglePart::html(personalized_content))?;

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
    Ok(())
}
