use std::collections::LinkedList;

use rand::Rng;

use crate::circle::CircleList;

#[derive(Clone)]
pub struct Card {
    pub name: &'static str,
    pub description: &'static str,
    pub discard_ammount: usize,
    pub message: String,
}

struct CardsAll {}

impl CardsAll {
    pub fn new() -> LinkedList<Card> {
        let mut list = LinkedList::new();
        list.push_back(Card{ name: "Big Bite", description: "Deal 10 to enemy", discard_ammount: 0, message: "You take a big bite. Deal 10 damage".to_string() });
        list.push_back(Card{ name: "Rage", description: "Deal 35 to enemy, discard 3 cards", discard_ammount: 3, message: "You charge at your enemy. Deal 35 damage".to_string() });
        list.push_back(Card{ name: "Second Wind", description: "Heal 50 to yourself, discard 5 cards", discard_ammount: 5, message: "You feel anew! Heal 50 health".to_string() });
        list.push_back(Card{ name: "Bloodsucker", description: "Deal 10 to enemy, heal 10, and discard 1 card", discard_ammount: 1, message: "You taste some blood. Deal 10 damage and heal 10".to_string() });
        list.push_back(Card{ name: "Sprint", description: "Draw 3 cards", discard_ammount: 0, message: "You reposition. Draw 3 additional cards".to_string() });
        list.push_back(Card{ name: "Refresh", description: "Discard 5, then draw 6 cards, and heal 5", discard_ammount: 5, message: "You clear your mind. You draw 6 cards, and heal 5".to_string() });
        list.push_back(Card{ name: "Scrach", description: "Deal 10 damage to enemy, draw 2 cards, discard 2 cards", discard_ammount: 2, message: "You conjure a swift strike. Deal 10, and draw 2 cards".to_string() });
        list
    }
}

pub fn generate_choose() -> CircleList<Card> {
    let mut list = CircleList::new();
    let cards_all = CardsAll::new();
    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        let index: usize = rng.gen::<usize>() % cards_all.len();
        list.data.push_back(cards_all.iter().nth(index).unwrap().clone());
    }
    list
}