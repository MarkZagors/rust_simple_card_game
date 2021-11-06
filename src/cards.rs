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
        list.push_back(Card{ name: "Big Bite", description: "Deal 10 to enemy hero", discard_ammount: 0, message: "You deal 10 damage".to_string() });
        list.push_back(Card{ name: "Lick", description: "Heal 5 yourself", discard_ammount: 0, message: "You recover 5 health".to_string() });
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