use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt::{self, Formatter};

// GAME ENGINE
struct GameEngine {
    player: Player,
    dealer: Player,
    deck: Deck,
}

enum RoundOutcome {
    PlayerWins,
    DealerWins,
    Draw,
    PlayerBusts,
    DealerBusts,
    DoubleBust,
}

impl GameEngine {
    fn new() -> Self {
        Self {
            player: Player::new(),
            dealer: Player::new(),
            deck: Deck::new(),
        }
    }

    fn inital_deal(&mut self) {
        self.deck.shuffle();

        for _ in 0..2 {
            self.player.hand.add(self.deck.draw_card());
            self.dealer.hand.add(self.deck.draw_card());
        }
    }

    fn run(&mut self) {
        while !self.player.stand && !self.dealer.stand {
            while self.player.hand.value() < 17 {
                self.player.hand.add(self.deck.draw_card());
            }

            println!("Player hand: {}", self.player.hand);
            self.player.stand = true;

            while !self.dealer.hand.value() < 17 {
                self.dealer.hand.add(self.deck.draw_card());
            }

            println!("Dealer hand: {}", self.dealer.hand);
            self.dealer.stand = true;
        }

        let player_hand_value = self.player.hand.value();
        let dealer_hand_value = self.dealer.hand.value();

        use RoundOutcome::*;

        match Self::evaluate_round(player_hand_value, dealer_hand_value) {
            PlayerWins => println!("Player wins! {}", player_hand_value),
            DealerWins => println!("Dealer win! {}", dealer_hand_value),
            Draw => println!(
                "It's a draw! Player: {}, Dealer: {}",
                player_hand_value, dealer_hand_value
            ),
            PlayerBusts => println!("Player busts! Dealer wins."),
            DealerBusts => println!("Dealer busts! Player wins."),
            DoubleBust => println!("Both bust! No winner."),
        };
    }

    fn evaluate_round(player_score: u8, dealer_score: u8) -> RoundOutcome {
        use RoundOutcome::*;
        use std::cmp::Ordering::*;

        match (player_score > 21, dealer_score > 21) {
            (true, true) => DoubleBust,
            (true, false) => PlayerBusts,
            (false, true) => DealerBusts,
            (false, false) => match player_score.cmp(&dealer_score) {
                Greater => PlayerWins,
                Less => DealerWins,
                Equal => Draw,
            },
        }
    }
}

// PLAYER AND DEALER
#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Player {
    stand: bool,
    hand: Hand,
}

impl Hand {
    fn new() -> Self {
        Self { cards: vec![] }
    }

    fn value(&self) -> u8 {
        self.cards.iter().map(|x| x.value).sum()
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let card_list = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}", card_list)
    }
}

impl Player {
    fn new() -> Self {
        Self {
            hand: Hand::new(),
            stand: false,
        }
    }
}

// CARD AND DECK
#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq)]
struct Card {
    kind: CardKind,
    suit: Suit,
    value: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit {
    Diamand,
    Heart,
    Spade,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Suit::*;

        let suit = match self {
            Diamand => "♦",
            Spade => "♠",
            Club => "♣",
            Heart => "♥",
        };

        write!(f, "{}", suit)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FaceType {
    King,
    Queen,
    Jack,
}

impl fmt::Display for FaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use FaceType::*;

        let face = match self {
            King => "K",
            Queen => "Q",
            Jack => "J",
        };

        write!(f, "{}", face)
    }
}

#[derive(Debug, PartialEq)]
enum CardKind {
    Number,
    Face(FaceType),
    Ace,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let face = match &self.kind {
            CardKind::Ace => "A".to_string(),
            CardKind::Face(face_type) => face_type.to_string(),
            CardKind::Number => self.value.to_string(),
        };

        write!(f, "{}{}", face, self.suit)
    }
}

impl Deck {
    fn new() -> Self {
        use CardKind::*;
        use FaceType::*;
        use Suit::*;

        const SUITS: [Suit; 4] = [Diamand, Heart, Spade, Club];
        const FACE_TYPE: [FaceType; 3] = [King, Queen, Jack];

        let mut cards: Vec<Card> = vec![];

        for number in 2..11 {
            for suit in SUITS.iter() {
                cards.push(Card {
                    kind: Number,
                    suit: *suit,
                    value: number,
                });
            }
        }

        for shape in SUITS.iter() {
            cards.push(Card {
                kind: Ace,
                suit: *shape,
                value: 11,
            });
            for face_type in FACE_TYPE.iter() {
                cards.push(Card {
                    kind: Face(*face_type),
                    suit: *shape,
                    value: 10,
                });
            }
        }

        Self { cards }
    }

    fn draw_card(&mut self) -> Card {
        let card = self.cards.pop().unwrap_or_else(|| {
            eprintln!(
                "Ops, the deck is empty. This shouldn't happen so maybe the dealer is cheating!"
            );
            std::process::exit(1);
        });

        card
    }

    fn shuffle(&mut self) {
        let mut rng = ThreadRng::default();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut game = GameEngine::new();
    game.inital_deal();
    game.run();
}
