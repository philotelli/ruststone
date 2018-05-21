enum CardKind {
    Minion,
    Spell,
    Weapon,
    Quest
}

struct Card {
    name: String,
    mana_cost: i8,
    card_type: CardKind
}

struct Player {
    hero: String,
    mana_per_turn: i8,
    health: i8,
    deck: Vec<Card>,
    hand: Vec<Card>,
    board: Vec<Card>
}

struct Game {
    player1: Player,
    player2: Player,
    turn: i8,

}

fn main() {
    println!("Hello, world!");
}
