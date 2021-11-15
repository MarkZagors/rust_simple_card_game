#![feature(linked_list_remove)]

mod cards;
use cards::Card as Card;
mod circle;
use circle::CircleList as CircleList;

use std::{collections::LinkedList, io::stdout};
use colored::*;
use crossterm::ExecutableCommand;
use crossterm::cursor::MoveTo;
use crossterm::{event::{Event, KeyCode, KeyEvent, KeyModifiers, read}, terminal::{Clear, ClearType}};
use rand::Rng;

struct State {
    enemy_hp: i32,
    enemy_max_hp: i32,
    enemy_power: i32,
    enemy_name: &'static str,
    enemy_cards: LinkedList<Card>,
    player_hp: i32,
    messages: LinkedList<String>,
    win: bool,
    lose: bool,
    stage: i32,
    not_played: bool,
}

fn render(state: &State) {
    stdout()
    .execute(Clear(ClearType::All)).unwrap()
    .execute(MoveTo(0,0)).unwrap();

    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(75));
    println!("\u{2502}Hp: {:<14}Name: {:<14}Stage: {:<30}\u{2502}", state.enemy_hp, state.enemy_name, state.stage);
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(75));

    // println!("\u{2502}{:^75}\u{2502}", "\u{2694} 50 \u{2694}   Test   \u{2661} 50 \u{2661}");
    // println!("\u{2502}{}\u{2502}", " ".repeat(75));
    println!("\u{2502}{:^75}\u{2502}", "vs");
    // println!("\u{2502}{}\u{2502}", " ".repeat(75));
    // println!("\u{2502}{:^75}\u{2502}", "\u{2694} 50 \u{2694}   Test   \u{2661} 50 \u{2661}");

    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(75));
    println!("\u{2502}Hp: {:<71}\u{2502}", state.player_hp);
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(75));

    for message in &state.messages {
        println!("\u{2022} {}", &message);
    }
    println!("\u{2500}{}\u{2500}", "\u{2500}".repeat(75));
}

fn render_cards(hand: &CircleList<Card>) {
    let iter = hand.data.iter();
    let mut index = 0;

    for card in iter {
        if index == hand.index {
            println!("> {:15}{:60}", card.name.red().bold(), card.description.red().bold());
        } else {
            println!("> {:15}{:60}", card.name, card.description);
        }
        index += 1;
    }
}

fn render_choose(cards_choose: &CircleList<Card>) {
    stdout()
    .execute(Clear(ClearType::All)).unwrap()
    .execute(MoveTo(0,0)).unwrap();

    println!("Choose a card to add to your deck: ");
    let iter = cards_choose.data.iter();
    let mut index = 0;

    for card in iter {
        if index == cards_choose.index {
            println!("> {:15}{:60}", card.name.red().bold(), card.description.red().bold());
        } else {
            println!("> {:15}{:60}", card.name, card.description);
        }
        index += 1;
    }
}

fn play_card(hand: &mut CircleList<Card>, state: &mut State) {
    let card = hand.get_current().clone();
    if hand.data.len() <= card.discard_ammount { state.not_played = true; return }

    state.messages.push_back(format!("You play {}", card.name));
    hand.data.remove(hand.index);
    discard(hand, card.discard_ammount, state);
    hand.index = 0;
    match card.name {
        "Bite" => { state.enemy_hp -= 5; },
        "Big Bite" => { state.enemy_hp -= 10; },
        "Lick" => { state.player_hp += 5; },
        "Rage" => { state.enemy_hp -= 35; },
        _ => {},
    }
    state.messages.push_back(card.message);

    if state.enemy_hp <= 0 {
        state.messages.push_back("You win!".to_string());
        state.messages.push_back("Press Enter to continue...".to_string());
        state.win = true;
    }
}

fn enemy_play_card(state: &mut State) {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen::<usize>() % state.enemy_cards.len();
    let card = state.enemy_cards.iter().nth(index).clone().unwrap();
    match card.name {
        "Bite" => { state.player_hp -= state.enemy_power; state.messages.push_back(format!("{} deals {} damage", state.enemy_name, state.enemy_power)) },
        _ => {},
    }

    if state.player_hp <= 0 {
        state.messages.push_back("You lose :<".to_string());
        state.messages.push_back("Press Esc to quit, or Enter to start a new game.".to_string());
        state.lose = true;
    }
}

fn discard(cards_hand: &mut CircleList<Card>, ammount: usize, state: &mut State) {
    for _ in 0..ammount {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen::<usize>() % cards_hand.data.len();
        state.messages.push_back(format!("Discarded {}", cards_hand.data.iter().nth(index).unwrap().name)); 
        cards_hand.data.remove(index);
    }
}

fn draw_cards(cards_deck: &mut LinkedList<Card>, cards_hand: &mut CircleList<Card>, all_cards: &LinkedList<Card> , state: &mut State, ammount: u16) {
    let mut rng = rand::thread_rng();
    for _ in 0..ammount {
        if cards_hand.data.len() >= 6 { break; }
        if cards_deck.len()-1 == 0 { cards_deck.append(&mut generate_card_deck(all_cards,state)); }
        let index: usize = rng.gen::<usize>() % cards_deck.len();
        let card = cards_deck.iter().nth(index).unwrap().clone();
        cards_deck.remove(index);
        cards_hand.data.push_back(card);
    }
}

fn init_all_cards() -> LinkedList<Card> {
    let mut list: LinkedList<Card> = LinkedList::new();
    list.push_back(Card{ name: "Bite", description: "Deal 5 to enemy", discard_ammount: 0, message: "You deal 5 damage".to_string() });
    list.push_back(Card{ name: "Bite", description: "Deal 5 to enemy", discard_ammount: 0, message: "You deal 5 damage".to_string() });
    list.push_back(Card{ name: "Bite", description: "Deal 5 to enemy", discard_ammount: 0, message: "You deal 5 damage".to_string() });
    list.push_back(Card{ name: "Lick", description: "Heal 5 to your", discard_ammount: 0, message: "You recover 5 health".to_string() });
    list.push_back(Card{ name: "Lick", description: "Heal 5 to your", discard_ammount: 0, message: "You recover 5 health".to_string() });
    list.push_back(Card{ name: "Lick", description: "Heal 5 to your", discard_ammount: 0, message: "You recover 5 health".to_string() });
    list
}

fn generate_card_deck(all_cards: &LinkedList<Card>, state: &mut State) -> LinkedList<Card> {
    let mut list: LinkedList<Card> = LinkedList::new();
    for card in all_cards {
        list.push_back(card.clone());
    }
    if !state.win { state.messages.push_back("Deck reshuffed".to_string()); }
    list
}


fn main() {
    loop { //New Game
        let mut all_cards = init_all_cards();
        let mut state = State{enemy_hp: 20, enemy_max_hp: 20, enemy_power:2, enemy_name: "Bat", player_hp: 20, messages: LinkedList::new(), win:false, enemy_cards: LinkedList::new(), lose: false, stage: 1, not_played: false};
        loop {
            let mut cards_hand : CircleList<Card> = CircleList::new();
            let mut card_deck = generate_card_deck(&all_cards, &mut state);
            draw_cards(&mut card_deck, &mut cards_hand, &all_cards, &mut state, 3);

            state.enemy_cards.push_back(Card{ name: "Bite", description: "", discard_ammount: 0, message: format!("{} deals 5 damage", state.enemy_name) });
            loop { //GAME LOOP
                render(&state);
                render_cards(&cards_hand);
                match read().unwrap() {
                    Event::Key(KeyEvent{ code: KeyCode::Down, modifiers: KeyModifiers::NONE }) => cards_hand.move_next(),
                    Event::Key(KeyEvent{ code: KeyCode::Up, modifiers: KeyModifiers::NONE }) => cards_hand.move_prev(),
                    Event::Key(KeyEvent{ code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
                        state.messages.clear();

                        if state.win { break; }
                        if state.lose { break; }
                        play_card(&mut cards_hand, &mut state);

                        if state.not_played { 
                            state.not_played = false;
                            state.messages.push_back("Not enough cards in hand!".to_string());
                            continue; 
                        }

                        if !state.win {
                            enemy_play_card(&mut state);
                            draw_cards(&mut card_deck, &mut cards_hand, &all_cards, &mut state, 2);
                        }
                    },
                    Event::Key(KeyEvent{ code: KeyCode::Char('s'), modifiers: KeyModifiers::NONE }) => {
                        if !state.lose {
                            state.messages.clear();
                            state.messages.push_back("You skipped a turn...".to_string());
                            enemy_play_card(&mut state);
                            draw_cards(&mut card_deck, &mut cards_hand, &all_cards, &mut state, 2);
                        }
                    },
                    Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE }) => return,
                    _ => (),
                }
            }

            if state.lose { break; }
            let mut cards_choose = cards::generate_choose();
            loop { //CHOOSE CARD
                render_choose(&cards_choose);
                match read().unwrap() {
                    Event::Key(KeyEvent{ code: KeyCode::Down, modifiers: KeyModifiers::NONE }) => cards_choose.move_next(),
                    Event::Key(KeyEvent{ code: KeyCode::Up, modifiers: KeyModifiers::NONE }) => cards_choose.move_prev(),
                    Event::Key(KeyEvent{ code: KeyCode::Enter, modifiers: KeyModifiers::NONE }) => {
                        all_cards.push_back(cards_choose.data.iter().nth(cards_choose.index).unwrap().clone());
                        break;
                    },
                    Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE }) => return,
                    _ => (),
                }
            }

            state.enemy_max_hp += 10;
            state.enemy_power += 1;
            state.enemy_hp = state.enemy_max_hp;
            state.win = false;
            state.player_hp = 20;
            state.stage += 1;
        }
    }
    
}

