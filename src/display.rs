use super::card;

fn handle_power(card: card::Card) {
    match card.power {
        Some(_) => println!("{}/{}", card.power.unwrap(), card.toughness.unwrap()),
        None => (),
    }
}

pub fn display_card(card: card::Card) {
    println!("{} {}", card.name, card.mana_cost);
    println!("{}", card.type_line);
    println!("{}", card.oracle_text);
    handle_power(card)
}
