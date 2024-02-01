use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardSuite {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl CardSuite {
    pub fn from_string(suite_str: &str) -> Self {
        match suite_str {
            "C" => CardSuite::Clubs,
            "D" => CardSuite::Diamonds,
            "H" => CardSuite::Hearts,
            "S" => CardSuite::Spades,
            _ => panic!("Invalid suite string"),
        }
    }
}

impl fmt::Display for CardSuite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardSuite::Clubs => write!(f, "C"),
            CardSuite::Diamonds => write!(f, "D"),
            CardSuite::Hearts => write!(f, "H"),
            CardSuite::Spades => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardRank {
    LowAce = 1,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardRank {
    pub fn as_number(&self) -> u8 {
        *self as u8
    }

    pub fn from_number(n: u8) -> Self {
        // let value = ((n - 2) % 13) + 2; // use mod 13 to wrap around // If there is no LowAce
        let value = ((n - 1) % 14) + 1; // use mod 14 to wrap around // with LowAce
        match value {
            1 => CardRank::LowAce,
            2 => CardRank::Two,
            3 => CardRank::Three,
            4 => CardRank::Four,
            5 => CardRank::Five,
            6 => CardRank::Six,
            7 => CardRank::Seven,
            8 => CardRank::Eight,
            9 => CardRank::Nine,
            10 => CardRank::Ten,
            11 => CardRank::Jack,
            12 => CardRank::Queen,
            13 => CardRank::King,
            14 => CardRank::Ace,
            _ => unreachable!(),
        }
    }

    pub fn from_string(rank_str: &str) -> Self {
        match rank_str {
            "1" => CardRank::LowAce, // note that this is not a real rank string, just a way to initialize a LowAce
            "2" => CardRank::Two,
            "3" => CardRank::Three,
            "4" => CardRank::Four,
            "5" => CardRank::Five,
            "6" => CardRank::Six,
            "7" => CardRank::Seven,
            "8" => CardRank::Eight,
            "9" => CardRank::Nine,
            "10" => CardRank::Ten,
            "J" => CardRank::Jack,
            "Q" => CardRank::Queen,
            "K" => CardRank::King,
            "A" => CardRank::Ace,
            _ => panic!("Invalid rank string"),
        }
    }
}

impl fmt::Display for CardRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardRank::LowAce => write!(f, "A"),
            CardRank::Two => write!(f, "2"),
            CardRank::Three => write!(f, "3"),
            CardRank::Four => write!(f, "4"),
            CardRank::Five => write!(f, "5"),
            CardRank::Six => write!(f, "6"),
            CardRank::Seven => write!(f, "7"),
            CardRank::Eight => write!(f, "8"),
            CardRank::Nine => write!(f, "9"),
            CardRank::Ten => write!(f, "10"),
            CardRank::Jack => write!(f, "J"),
            CardRank::Queen => write!(f, "Q"),
            CardRank::King => write!(f, "K"),
            CardRank::Ace => write!(f, "A"),
        }
    }
}

impl Ord for CardRank {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for CardRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Interesting idea, but not needed for this exercise

impl Add<u8> for CardRank {
    type Output = Self;

    fn add(self, other: u8) -> Self::Output {
        CardRank::from_number(self as u8 + other)
    }
}

impl Add<CardRank> for CardRank {
    type Output = Self;

    fn add(self, other: CardRank) -> Self::Output {
        CardRank::from_number(self as u8 + other as u8)
    }
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct Card {
    rank: CardRank,
    suite: CardSuite,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Card {
    pub fn get_highest_card(cards: &[Card]) -> Card {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal));
        cards.last().unwrap().clone()
    }

    pub fn rank(&self) -> &CardRank {
        &self.rank
    }

    pub fn new(card_str: &str) -> Self {
        assert!(
            card_str.len() >= 2,
            "Card string must be at least 2 characters long"
        );

        let rank_str = &card_str[..(card_str.len() - 1)];
        let suite_str = &card_str[(card_str.len() - 1)..];

        let rank = CardRank::from_string(rank_str);
        let suite = CardSuite::from_string(suite_str);

        Card { rank, suite }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suite)
    }
}

pub struct CardVec(Vec<Card>);

impl fmt::Display for CardVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum PokerHandRank {
    HighCard = 1,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl PokerHandRank {
    pub fn as_number(&self) -> u8 {
        *self as u8
    }

    pub fn from_number(n: u8) -> Self {
        match n {
            1 => PokerHandRank::HighCard,
            2 => PokerHandRank::OnePair,
            3 => PokerHandRank::TwoPairs,
            4 => PokerHandRank::ThreeOfAKind,
            5 => PokerHandRank::Straight,
            6 => PokerHandRank::Flush,
            7 => PokerHandRank::FullHouse,
            8 => PokerHandRank::FourOfAKind,
            9 => PokerHandRank::StraightFlush,
            10 => PokerHandRank::RoyalFlush,
            _ => panic!("Invalid number for PokerHandRank"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub struct PokerHand<'a> {
    rank: PokerHandRank,
    cards: [Card; 5],
    raw: &'a str,
    pairs: Option<(Pair, Option<Pair>)>,
    triplet: Option<Triplet>,
    quadruplet: Option<Quadruplet>,
    sequence: Option<Sequence>,
}

impl<'a> fmt::Display for PokerHand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.raw)
    }
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl<'a> PokerHand<'a> {
    /// This function creates a suite map from a hand of cards.
    ///
    /// The suite map is an array of 4 elements, each representing one of the four suites in a deck of cards.
    /// The index of the array represents the suite (as a usize), and the value at that index represents the count of cards of that suite in the hand.
    /// For example, if the array is [2, 1, 1, 1], it means there are 2 cards of the first suite, and 1 card each of the other three suites.
    ///
    /// This is called a "map" because it maps each suite to the count of cards of that suite.
    fn get_suite_map(cards: [Card; 5]) -> [usize; 4] {
        let mut suites = [0; 4];
        for card in &cards {
            let suite = card.suite as usize;
            suites[suite] += 1;
        }
        suites
    }

    /// This function creates a rank map from a hand of cards.
    ///
    /// The rank map is an array of 15 elements, each representing one of the 15 possible ranks in a deck of cards (1-14, with 1 and 14 both representing Ace).
    /// The index of the array represents the rank (as a usize), and the value at that index represents the count of cards of that rank in the hand.
    /// For example, if the array is [0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2], it means there are 2 Aces and no other cards.
    ///
    /// This is called a "map" because it maps each rank to the count of cards of that rank.
    fn get_rank_map(cards: [Card; 5]) -> [usize; 15] {
        let mut ranks = [0; 15];
        for card in &cards {
            let rank = card.rank.as_number() as usize;
            ranks[rank] += 1;
            if rank == 14 {
                ranks[1] += 1; // Ace can also be considered as 1
            }
        }
        ranks
    }

    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    fn is_flush(suite_map: [usize; 4]) -> bool {
        suite_map.iter().any(|&s| s == 5)
    }

    fn is_straight(rank_map: [usize; 15]) -> bool {
        (rank_map
            .windows(5)
            .any(|window| window.iter().all(|&r| r == 1))
            && !(rank_map[13] == 1 && rank_map[14] == 1 && rank_map[2] == 1)) // Not a straight if there is a King, an Ace, and a Two
            || (rank_map[0..5].iter().all(|&r| r == 1) && rank_map[14] == 1) // Ace to 5 straight
    }

    fn is_high_sequence(rank_map: [usize; 15]) -> bool {
        rank_map[10..15].iter().all(|&r| r == 1)
    }

    fn is_three_of_a_kind(ranks: &[usize; 15]) -> bool {
        ranks.iter().any(|&r| r == 3)
    }

    fn is_four_of_a_kind(ranks: &[usize; 15]) -> bool {
        ranks.iter().any(|&r| r == 4)
    }

    fn is_full_house(ranks: &[usize; 15]) -> bool {
        PokerHand::is_three_of_a_kind(ranks) && PokerHand::is_one_pair(ranks)
    }

    fn is_two_pairs(ranks: &[usize; 15]) -> bool {
        ranks.iter().filter(|&&r| r == 2).count() == 2
    }

    fn is_one_pair(ranks: &[usize; 15]) -> bool {
        ranks.iter().any(|&r| r == 2)
    }

    pub fn get_triplet(cards: [Card; 5]) -> Option<Triplet> {
        let ranks = PokerHand::get_rank_map(cards);
        if PokerHand::is_three_of_a_kind(&ranks) || PokerHand::is_full_house(&ranks) {
            let mut rank = CardRank::from_number(ranks.iter().position(|&r| r == 3).unwrap() as u8);
            if rank == CardRank::LowAce {
                rank = CardRank::Ace;
            }
            let cards = cards
                .iter()
                .filter(|&card| card.rank == rank)
                .cloned()
                .collect::<Vec<Card>>();
            Some(Triplet {
                rank,
                cards: [cards[0], cards[1], cards[2]],
            })
        } else {
            None
        }
    }

    pub fn get_quadruplet(cards: [Card; 5]) -> Option<Quadruplet> {
        let ranks = PokerHand::get_rank_map(cards);
        if PokerHand::is_four_of_a_kind(&ranks) {
            let mut rank = CardRank::from_number(ranks.iter().position(|&r| r == 4).unwrap() as u8);
            if rank == CardRank::LowAce {
                rank = CardRank::Ace;
            }
            let cards = cards
                .iter()
                .filter(|&card| card.rank == rank)
                .cloned()
                .collect::<Vec<Card>>();
            Some(Quadruplet {
                rank,
                cards: [cards[0], cards[1], cards[2], cards[3]],
            })
        } else {
            None
        }
    }

    pub fn get_pairs(cards: [Card; 5]) -> Option<(Pair, Option<Pair>)> {
        let ranks = PokerHand::get_rank_map(cards);

        if PokerHand::is_two_pairs(&ranks) {
            let first_pair: Pair = {
                let rank =
                    CardRank::from_number(ranks.iter().rposition(|&r| r == 2).unwrap() as u8);
                let cards = cards
                    .iter()
                    .filter(|&card| card.rank == rank)
                    .cloned()
                    .collect::<Vec<Card>>();
                Pair {
                    rank,
                    cards: [cards[0], cards[1]],
                }
            };
            let second_pair: Pair = {
                let rank = CardRank::from_number(
                    ranks
                        .iter()
                        .position(|&r| r == 2 && CardRank::from_number(r as u8) != first_pair.rank)
                        .unwrap() as u8,
                );
                let cards = cards
                    .iter()
                    .filter(|&card| card.rank == rank)
                    .cloned()
                    .collect::<Vec<Card>>();
                Pair {
                    rank,
                    cards: [cards[0], cards[1]],
                }
            };
            Some((first_pair, Some(second_pair)))
        } else if PokerHand::is_one_pair(&ranks) || PokerHand::is_full_house(&ranks) {
            let pair = {
                let rank = CardRank::from_number(ranks.iter().position(|&r| r == 2).unwrap() as u8);
                let cards = cards
                    .iter()
                    .filter(|&card| card.rank == rank)
                    .cloned()
                    .collect::<Vec<Card>>();
                Pair {
                    rank,
                    cards: [cards[0], cards[1]],
                }
            };

            Some((pair, None))
        } else {
            None
        }
    }

    pub fn get_sequence(cards: [Card; 5]) -> Option<Sequence> {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.rank.cmp(&b.rank));
        let ranks =
            PokerHand::get_rank_map(cards.clone().try_into().expect("Expected exactly 5 cards"));
        if PokerHand::is_straight(ranks) {
            let min_rank = CardRank::from_number(ranks.iter().position(|&r| r > 0).unwrap() as u8);
            let max_rank = CardRank::from_number(ranks.iter().rposition(|&r| r > 0).unwrap() as u8);

            // LowAce
            match (min_rank, max_rank) {
                (CardRank::LowAce, CardRank::Ace) => {
                    cards[4].rank = CardRank::LowAce;
                    cards.sort_by(|a, b| a.rank.cmp(&b.rank));
                    Some(Sequence {
                        rank: CardRank::Five,
                        cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                    })
                }
                _ => Some(Sequence {
                    rank: max_rank,
                    cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                }),
            }
        } else {
            None
        }
    }

    pub fn get_rank(cards: [Card; 5]) -> PokerHandRank {
        let suites = PokerHand::get_suite_map(cards);
        let ranks = PokerHand::get_rank_map(cards);

        let is_flush = PokerHand::is_flush(suites);
        let is_straight = PokerHand::is_straight(ranks);

        if is_flush && is_straight {
            if PokerHand::is_high_sequence(ranks) {
                return PokerHandRank::RoyalFlush;
            } else {
                return PokerHandRank::StraightFlush;
            }
        }

        if PokerHand::is_four_of_a_kind(&ranks) {
            return PokerHandRank::FourOfAKind;
        }

        if PokerHand::is_full_house(&ranks) {
            return PokerHandRank::FullHouse;
        }

        if is_flush {
            return PokerHandRank::Flush;
        }

        if is_straight {
            return PokerHandRank::Straight;
        }

        if PokerHand::is_three_of_a_kind(&ranks) {
            return PokerHandRank::ThreeOfAKind;
        }

        if PokerHand::is_two_pairs(&ranks) {
            return PokerHandRank::TwoPairs;
        }

        if PokerHand::is_one_pair(&ranks) {
            return PokerHandRank::OnePair;
        }

        PokerHandRank::HighCard
    }

    fn cards_from_raw_string(raw: &'a str) -> [Card; 5] {
        let card_strs: Vec<&str> = raw.split_whitespace().collect();
        assert!(card_strs.len() == 5, "Hand string must contain 5 cards");

        let mut cards: [Card; 5] = [
            Card::new(card_strs[0]),
            Card::new(card_strs[1]),
            Card::new(card_strs[2]),
            Card::new(card_strs[3]),
            Card::new(card_strs[4]),
        ];

        cards.sort_by(|a, b| a.rank.cmp(&b.rank)); // sort ascending
        cards
    }

    pub fn new(raw: &'a str) -> Self {
        let cards: [Card; 5] = PokerHand::cards_from_raw_string(raw);
        let rank = PokerHand::get_rank(cards);
        let pairs: Option<(Pair, Option<Pair>)>;
        let triplet: Option<Triplet>;
        let quadruplet: Option<Quadruplet>;
        let sequence: Option<Sequence>;

        match rank {
            PokerHandRank::HighCard => {
                pairs = None;
                triplet = None;
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::OnePair => {
                pairs = PokerHand::get_pairs(cards);
                triplet = None;
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::TwoPairs => {
                pairs = PokerHand::get_pairs(cards);
                triplet = None;
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::ThreeOfAKind => {
                pairs = None;
                triplet = PokerHand::get_triplet(cards);
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::Straight => {
                pairs = None;
                triplet = None;
                quadruplet = None;
                sequence = PokerHand::get_sequence(cards);
            }
            PokerHandRank::Flush => {
                pairs = None;
                triplet = None;
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::FullHouse => {
                pairs = PokerHand::get_pairs(cards);
                triplet = PokerHand::get_triplet(cards);
                quadruplet = None;
                sequence = None;
            }
            PokerHandRank::FourOfAKind => {
                pairs = None;
                triplet = None;
                quadruplet = PokerHand::get_quadruplet(cards);
                sequence = None;
            }
            PokerHandRank::StraightFlush => {
                pairs = None;
                triplet = None;
                quadruplet = None;
                sequence = PokerHand::get_sequence(cards);
            }
            PokerHandRank::RoyalFlush => {
                pairs = None;
                triplet = None;
                quadruplet = None;
                sequence = PokerHand::get_sequence(cards);
            }
        }

        PokerHand {
            raw,
            rank,
            cards,
            pairs,
            triplet,
            quadruplet,
            sequence,
        }
    }
}

pub struct PokerHandVec<'a>(Vec<PokerHand<'a>>);

impl<'a> fmt::Display for PokerHandVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|hand| hand.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn untie(hands: Vec<PokerHand>, rank: PokerHandRank) -> Vec<PokerHand> {
    let hands: Vec<PokerHand> = hands.into_iter().filter(|hand| hand.rank == rank).collect();

    match rank {
        PokerHandRank::HighCard => untie_highest(hands, None),
        PokerHandRank::Flush => untie_highest(hands, None),
        PokerHandRank::ThreeOfAKind => untie_three(hands),
        PokerHandRank::FourOfAKind => untie_four(hands),
        PokerHandRank::OnePair => untie_one_pair(hands),
        PokerHandRank::TwoPairs => untie_two_pairs(hands),
        PokerHandRank::Straight => untie_straight(hands),
        PokerHandRank::StraightFlush => untie_straight(hands),
        PokerHandRank::FullHouse => untie_full_house(hands),
        _ => hands,
    }
}

fn get_cards(hands: Vec<PokerHand>) -> Vec<Card> {
    hands.iter().flat_map(|hand| hand.cards.clone()).collect()
}

// This function takes a vector of PokerHand objects and a vector of CardRank objects.
// It returns a new vector containing only those PokerHand objects that contain all the CardRank objects specified in the ranks vector.
// Each PokerHand object's cards are mapped to their ranks, creating a new vector of ranks.
// The function then checks if this vector of ranks contains all the CardRank objects specified in the ranks vector.
// If a PokerHand object contains all the specified ranks, it is included in the output vector.
// If it does not contain all the specified ranks, it is excluded from the output vector.
fn filter_hands(hands: Vec<PokerHand>, ranks: Vec<CardRank>) -> Vec<PokerHand> {
    hands
        .iter()
        .filter(|&hand| {
            let hand_ranks = hand.cards.map(|card| card.rank);
            ranks.iter().all(|&rank| hand_ranks.contains(&rank))
        })
        .cloned()
        .collect()
}

fn filter_cards(cards: Vec<Card>, ranks: Vec<CardRank>) -> Vec<Card> {
    cards
        .into_iter()
        .filter(|card| !ranks.contains(&card.rank))
        .collect()
}

#[derive(Debug, Clone)]
struct UntieHighestConfig {
    count: usize,
    ranks: Vec<CardRank>,
}

impl Default for UntieHighestConfig {
    fn default() -> Self {
        Self {
            count: 5,
            ranks: vec![],
        }
    }
}

fn untie_highest(hands: Vec<PokerHand>, config: Option<UntieHighestConfig>) -> Vec<PokerHand> {
    let config = config.unwrap_or_default();
    // tied ranks
    let skip_ranks = config.ranks.clone();
    let mut tied_ranks = config.ranks.clone();

    // remove the hands which don't have all the highest cards
    let mut hands: Vec<PokerHand> = filter_hands(hands, tied_ranks.clone());

    // get all remaining cards
    let mut all_cards: Vec<Card> = get_cards(hands.clone());
    all_cards = filter_cards(all_cards, skip_ranks);

    // find the highest card
    let mut highest_card = Card::get_highest_card(&all_cards);

    tied_ranks.push(highest_card.rank);
    hands = filter_hands(hands, tied_ranks.clone());

    // check the next highest card until all have been checked
    for _ in 0..(config.count - 1) {
        if hands.len() <= 1 {
            return hands;
        }
        if all_cards.len() == 0 {
            break;
        }
        all_cards = get_cards(hands.clone());
        // remove the highest cards from the list of all cards
        all_cards = filter_cards(all_cards, tied_ranks.clone());
        highest_card = Card::get_highest_card(&all_cards);
        tied_ranks.push(highest_card.rank);
        hands = filter_hands(hands, tied_ranks.clone());
    }
    hands
}

fn untie_straight(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut sequences: Vec<(Sequence, &PokerHand)> = hands
        .iter()
        .filter_map(|hand| hand.sequence.as_ref().map(|seq| (seq.clone(), hand)))
        .collect();

    sequences.sort_by(|a, b| b.0.rank.cmp(&a.0.rank));

    let highest_sequence_rank = sequences[0].0.rank;

    sequences
        .into_iter()
        .filter(|(seq, _)| seq.rank == highest_sequence_rank)
        .map(|(_, hand)| hand.clone())
        .collect()
}

fn untie_four(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut quadruplets: Vec<(Quadruplet, &PokerHand)> = hands
        .iter()
        .filter_map(|hand| hand.quadruplet.as_ref().map(|quad| (quad.clone(), hand)))
        .collect();

    quadruplets.sort_by(|a, b| b.0.rank.cmp(&a.0.rank));

    let highest_quadruplet_rank = quadruplets[0].0.rank;

    let hands: Vec<PokerHand<'_>> = quadruplets
        .into_iter()
        .filter(|(quad, _)| quad.rank == highest_quadruplet_rank)
        .map(|(_, hand)| hand.clone())
        .collect();

    untie_highest(
        hands,
        Some(UntieHighestConfig {
            count: 1,
            ranks: vec![highest_quadruplet_rank],
        }),
    )
}

fn untie_one_pair(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut pairs: Vec<((Pair, Option<Pair>), &PokerHand)> = hands
        .iter()
        .filter_map(|hand| hand.pairs.as_ref().map(|pairs| (pairs.clone(), hand)))
        .collect();

    pairs.sort_by(|a, b| b.0 .0.rank.cmp(&a.0 .0.rank));

    let highest_pair_rank = pairs[0].0 .0.rank;

    let hands: Vec<PokerHand<'_>> = pairs
        .into_iter()
        .filter(|((pair, _), _)| pair.rank == highest_pair_rank)
        .map(|(_, hand)| hand.clone())
        .collect();

    untie_highest(
        hands,
        Some(UntieHighestConfig {
            count: 3,
            ranks: vec![highest_pair_rank],
        }),
    )
}

fn untie_two_pairs(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut pairs: Vec<((Pair, Option<Pair>), &PokerHand)> = hands
        .iter()
        .filter_map(|hand| hand.pairs.as_ref().map(|pairs| (pairs.clone(), hand)))
        .collect();

    pairs.sort_by(|a, b| b.0 .0.rank.cmp(&a.0 .0.rank));

    let first_pair_rank = pairs[0].0 .0.rank;

    pairs = pairs
        .into_iter()
        .filter(|((pair, _), _)| pair.rank == first_pair_rank)
        .collect();

    let second_pair_rank = pairs
        .iter()
        .filter_map(|((_, second_pair), _)| second_pair.as_ref().map(|pair| pair.rank))
        .max();

    if let Some(second_pair_rank) = second_pair_rank {
        pairs = pairs
            .into_iter()
            .filter(|((_, second_pair), _)| {
                second_pair
                    .as_ref()
                    .map_or(false, |pair| pair.rank == second_pair_rank)
            })
            .collect();
    }

    let hands: Vec<PokerHand> = pairs.into_iter().map(|(_, hand)| hand.clone()).collect();

    if hands.len() > 1 {
        untie_highest(
            hands,
            Some(UntieHighestConfig {
                count: 1,
                ranks: vec![first_pair_rank, second_pair_rank.unwrap_or(first_pair_rank)],
            }),
        )
    } else {
        hands
    }
}

fn untie_three(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut triplets: Vec<(Triplet, &PokerHand)> = hands
        .iter()
        .map(|hand| (hand.triplet.unwrap().clone(), hand))
        .collect();

    triplets.sort_by(|a, b| b.0.rank.cmp(&a.0.rank));

    let highest_triplet_rank = triplets[0].0.rank;

    triplets = triplets
        .into_iter()
        .filter(|(triplet, _)| triplet.rank == highest_triplet_rank)
        .collect();

    let hands: Vec<PokerHand<'_>> = triplets
        .clone()
        .into_iter()
        .map(|(_, hand)| hand.clone())
        .collect();

    untie_highest(
        hands,
        Some(UntieHighestConfig {
            count: 2,
            ranks: vec![highest_triplet_rank],
        }),
    )
}

fn untie_full_house(hands: Vec<PokerHand>) -> Vec<PokerHand> {
    let mut triplets: Vec<(Triplet, &PokerHand)> = hands
        .iter()
        .map(|hand| (hand.triplet.unwrap().clone(), hand))
        .collect();

    triplets.sort_by(|a, b| b.0.rank.cmp(&a.0.rank));

    let highest_triplet_rank = triplets[0].0.rank;

    triplets = triplets
        .into_iter()
        .filter(|(triplet, _)| triplet.rank == highest_triplet_rank)
        .collect();

    let hands: Vec<PokerHand<'_>> = triplets
        .clone()
        .into_iter()
        .map(|(_, hand)| hand.clone())
        .collect();

    untie_highest(
        hands,
        Some(UntieHighestConfig {
            count: 2,
            ranks: vec![highest_triplet_rank],
        }),
    )
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut scored_hands: Vec<PokerHand> = hands
        .iter()
        .map(|&hand_str| PokerHand::new(hand_str))
        .collect();

    scored_hands.sort_by(|a, b| a.rank.cmp(&b.rank));
    let highest_rank: PokerHandRank = scored_hands.last().unwrap().rank;
    let winners = untie(scored_hands, highest_rank);
    winners
        .iter()
        .filter(|hand| hand.rank == highest_rank)
        .map(|hand| hand.raw)
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Triplet {
    rank: CardRank,
    cards: [Card; 3],
}

impl Ord for Triplet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Triplet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Quadruplet {
    rank: CardRank,
    cards: [Card; 4],
}

impl Ord for Quadruplet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Quadruplet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pair {
    rank: CardRank,
    cards: [Card; 2],
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Sequence {
    rank: CardRank,
    cards: [Card; 5],
}

impl Ord for Sequence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Sequence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
