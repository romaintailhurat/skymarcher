mod card;
mod display;

use structopt::StructOpt;
use url::Url;

/** CLI API */
#[derive(StructOpt, Debug)]
enum Cli {
    /// Get a card at random =)
    Random {},
    /// Get a card from a specific set
    Card {
        /// The set trigram, for example: iko
        set: String,
        /// The number of the card in the set
        number: u32,
    },
}

/** Builds a url from a fragment */
fn build_query(fragment: &str) -> Result<Url, Box<dyn std::error::Error>> {
    let base_url: Url = Url::parse("https://api.scryfall.com/")?;
    Ok(base_url.join(fragment)?)
}

/** A simple GET request */
fn req(url: Url) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url.as_str())?.text();
    let card: card::Card = serde_json::from_str(&resp?)?;
    display::display_card(card);
    Ok(())
}

fn random() -> Result<(), Box<dyn std::error::Error>> {
    let _ = req(build_query("cards/random/")?)?;
    Ok(())
}

fn card(set: String, number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let c: &str = &*set;
    let n: &str = &*number.to_string();
    let path: &str = &*format!("cards/{}/{}", c, n);
    let q = build_query(path)?;
    let _ = req(q);
    Ok(())
}

/** Manage the call to the CLI */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    match args {
        Cli::Random {} => random(),
        Cli::Card { set, number } => card(set, number),
    }
}
