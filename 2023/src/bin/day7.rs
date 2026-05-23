//! --- Day 7: Camel Cards ---
//!
//! In Camel Cards, you get a list of hands, and your goal is to order them based on the strength
//! of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3,
//! or 2. The relative strength of each card follows this order, where A is the highest and 2 is
//! the lowest.
//!
//! Every hand is exactly one type. From strongest to weakest, they are:
//!
//! Five of a kind, where all five cards have the same label: AAAAA
//! Four of a kind, where four cards have the same label and one card has a different label: AA8AA
//! Full house, where three cards have the same label, and the remaining two cards share
//! a different label: 23332
//! Three of a kind, where three cards have the same label, and the remaining two cards are each
//! different from any other card in the hand: TTT98
//! Two pair, where two cards share one label, two other cards share a second label, and the
//! remaining card has a third label: 23432
//! One pair, where two cards share one label, and the other three cards have a different label
//! from the pair and each other: A23A4
//! High card, where all cards' labels are distinct: 23456
//! Hands are primarily ordered based on type; for example, every full house is stronger than any
//! three of a kind.
//!
//! If two hands have the same type, a second ordering rule takes effect. Start by comparing the
//! first card in each hand. If these cards are different, the hand with the stronger first card is
//! considered stronger. If the first card in each hand have the same label, however, then move on
//! to considering the second card in each hand. If they differ, the hand with the higher second
//! card wins; otherwise, continue with the third card in each hand, then the fourth, then the
//! fifth.
//!
//! So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card
//! is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because
//! its third card is stronger (and both hands have the same first and second card).
//!
//! To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle
//! input). For example:
//!
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//! This example shows five hands; each hand is followed by its bid amount. Each hand wins an
//! amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the
//! second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five
//! hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
//!
//! So, the first step is to put the hands in order of strength:
//!
//! 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
//! KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second
//! card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
//! T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank
//! 5 and T55J5 gets rank 4.
//! Now, you can determine the total winnings of this set of hands by adding up the result of
//! multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So
//! the total winnings in this example are 6440.
//!
//! Find the rank of every hand in your set. What are the total winnings?

use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    Two,
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

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;
        match s {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "T" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            c => Err(format!("invalid card: {}", c)),
        }
    }
}

impl Card {
    #[cfg(test)]
    fn as_str(&self) -> &'static str {
        use Card::*;
        match self {
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "T",
            Jack => "J",
            Queen => "Q",
            King => "K",
            Ace => "A",
        }
    }

    /// Returns the rank of the card under ruleset 1.
    fn rank_ruleset_1(&self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }

    /// Returns the rank of the card under ruleset 2.
    /// In ruleset 2, 'J' is the weakest card.
    fn rank_ruleset_2(&self) -> u8 {
        match self {
            Card::Jack => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
}

impl HandType {
    /// Parse a string into a HandType.
    /// e.g., "AAAAA" -> FiveOfAKind
    fn type_from_str_ruleset_1(s: &str) -> Result<Self, ()> {
        use HandType::*;

        // Count the occurrences of each card
        let mut counts = std::collections::HashMap::new();
        for c in s.chars() {
            let card = c.to_string().parse::<Card>().unwrap();
            *counts.entry(card).or_insert(0) += 1;
        }

        // Collect the counts and sort them in descending order
        let mut count_values: Vec<u32> = counts.values().cloned().collect();
        count_values.sort_unstable_by(|a, b| b.cmp(a));

        // Match the sorted counts to determine the hand type
        match count_values.as_slice() {
            [5] => Ok(FiveOfAKind),
            [4, 1] => Ok(FourOfAKind),
            [3, 2] => Ok(FullHouse),
            [3, 1, 1] => Ok(ThreeOfAKind),
            [2, 2, 1] => Ok(TwoPair),
            [2, 1, 1, 1] => Ok(OnePair),
            [1, 1, 1, 1, 1] => Ok(HighCard),
            _ => Err(()),
        }
    }

    /// Parse a string into a HandType with jokers.
    ///
    /// In ruleset 2, 'J' (Jack) cards are jokers and can represent any card.
    /// For each joker, we improve the hand by one step:
    /// HighCard -> OnePair -> ThreeOfAKind -> FourOfAKind -> FiveOfAKind
    fn type_from_str_ruleset_2(s: &str) -> Result<Self, ()> {
        use HandType::*;

        // Count the occurrences of each card, treating 'J' as jokers
        let mut counts: std::collections::HashMap<Card, u32> = std::collections::HashMap::new();
        let mut jokers = 0;

        for c in s.chars() {
            if c == 'J' {
                jokers += 1;
            } else {
                let card = c.to_string().parse::<Card>().map_err(|_| ())?;
                *counts.entry(card).or_insert(0) += 1;
            }
        }

        // Find the highest count of any card
        let highest_count = counts.values().cloned().max().unwrap_or(0);

        // Compute new highest count by adding jokers, capped at 5
        let new_highest_count = std::cmp::min(highest_count + jokers, 5);

        // Map the new highest count to the corresponding hand type
        let hand_type = match new_highest_count {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => ThreeOfAKind,
            2 => OnePair,
            1 => HighCard,
            _ => return Err(()),
        };

        Ok(hand_type)
    }
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
    ruleset: u8,
}

#[cfg(test)]
impl Hand {
    fn cards_to_string(&self) -> String {
        self.cards.iter().map(|c| c.as_str()).collect()
    }
}

impl Hand {
    fn from_str_with_ruleset<F>(s: &str, hand_type_func: F, ruleset: u8) -> Result<Self, ()>
    where
        F: Fn(&str) -> Result<HandType, ()>,
    {
        let mut parts = s.split_whitespace();
        let cards_str = parts.next().ok_or(())?;
        let cards: Vec<Card> = cards_str
            .chars()
            .map(|c| c.to_string().parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;
        let hand_type = hand_type_func(cards_str)?;
        let bid = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Hand {
            bid,
            cards,
            hand_type,
            ruleset,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare the type of hand
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        // Compare cards using the appropriate ranking method
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return match self.ruleset {
                    1 => self_card.rank_ruleset_1().cmp(&other_card.rank_ruleset_1()),
                    2 => self_card.rank_ruleset_2().cmp(&other_card.rank_ruleset_2()),
                    _ => panic!("invalid ruleset"),
                };
            }
        }
        panic!("expect to never compare two equal hands");
    }
}

fn calc_winnings(hands: &[Hand]) -> u32 {
    let mut total = 0;
    let mut rank = 1;
    let mut sorted_hands = hands.to_vec();
    sorted_hands.sort();
    for hand in sorted_hands {
        total += hand.bid * rank;
        rank += 1;
    }
    total
}

fn main() {
    println!("Day 7: Camel Cards");

    let input = include_str!("../../input/day7.txt");

    let hands_ruleset_1 = input
        .lines()
        .map(|line| Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_1, 1))
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse hands");
    let total_winnings = calc_winnings(&hands_ruleset_1);
    println!("Total winnings: {}", total_winnings);

    let hands_ruleset_2 = input
        .lines()
        .map(|line| Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_2, 2))
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse hands");
    let total_winnings_part_2 = calc_winnings(&hands_ruleset_2);
    println!("Total winnings part 2: {}", total_winnings_part_2);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_sort_hands() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let hands = input
            .lines()
            .map(|line| {
                Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_1, 1).unwrap()
            })
            .collect::<Vec<_>>();

        let mut sorted_hands = hands.clone();
        sorted_hands.sort();

        dbg!(&sorted_hands);

        assert_eq!(sorted_hands[0].cards_to_string(), "32T3K");
        assert_eq!(sorted_hands[1].cards_to_string(), "KTJJT");
        assert_eq!(sorted_hands[2].cards_to_string(), "KK677");
        assert_eq!(sorted_hands[3].cards_to_string(), "T55J5");
        assert_eq!(sorted_hands[4].cards_to_string(), "QQQJA");
    }

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let hands = input
            .lines()
            .map(|line| {
                Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_1, 1).unwrap()
            })
            .collect::<Vec<_>>();

        dbg!(&hands);

        let mut sorted_hands = hands.clone();
        sorted_hands.sort();
        dbg!(sorted_hands);

        assert_eq!(calc_winnings(&hands), 6440);
    }

    #[test]
    fn test_solution_part_1() {
        let input = include_str!("../../input/day7.txt");
        let hands = input
            .lines()
            .map(|line| {
                Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_1, 1).unwrap()
            })
            .collect::<Vec<_>>();

        let total_winnings = calc_winnings(&hands);
        assert_eq!(total_winnings, 255048101);
    }

    #[test]
    fn test_type_from_str_ruleset_2() {
        // Example with no jokers
        let hand_type = HandType::type_from_str_ruleset_2("23456").unwrap();
        assert_eq!(hand_type, HandType::HighCard);

        // Example with jokers improving the hand
        let hand_type = HandType::type_from_str_ruleset_2("2JJJJ").unwrap();
        assert_eq!(hand_type, HandType::FiveOfAKind);

        // Another example
        let hand_type = HandType::type_from_str_ruleset_2("AA8AA").unwrap();
        assert_eq!(hand_type, HandType::FourOfAKind);

        // OnePair improved to ThreeOfAKind with jokers
        let hand_type = HandType::type_from_str_ruleset_2("A23A4").unwrap();
        assert_eq!(hand_type, HandType::OnePair);

        let hand_type = HandType::type_from_str_ruleset_2("A23JJ").unwrap();
        assert_eq!(hand_type, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_example_part_2() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        let hands = input
            .lines()
            .map(|line| {
                Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_2, 2).unwrap()
            })
            .collect::<Vec<_>>();

        let total_winnings = calc_winnings(&hands);
        assert_eq!(total_winnings, 5905);
    }

    #[test]
    fn test_solution_part_2() {
        let input = include_str!("../../input/day7.txt");
        let hands = input
            .lines()
            .map(|line| {
                Hand::from_str_with_ruleset(line, HandType::type_from_str_ruleset_2, 2).unwrap()
            })
            .collect::<Vec<_>>();

        let total_winnings = calc_winnings(&hands);
        assert_eq!(total_winnings, 252279775);
    }
}
