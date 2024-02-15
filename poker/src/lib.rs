use std::collections::LinkedList;
use std::{collections::HashMap, vec};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
struct HandRank {
    rank: u16,
    tie_breaker: str,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        vec![hands[0]]
    } else if hands.iter().all(|hand| hand.replace(" ", "").len() == 10) {
        todo!()
    } else {
        panic!("Card are not with correct lenght!!")
    }
}

fn rank_hand(hand: &str) -> HandRank {
    let sorted_hand: Vec<(u16, &str)> = hand
        .split_whitespace()
        .map(|card| {
            let (num, suit) = card.split_at(1);
            let num = num.parse::<u16>().unwrap();
            (num, suit)
        })
        .collect();

    let numbers: LinkedList<_> = sorted_hand.iter().map(|&(num, _)| num).collect();
    let suits: LinkedList<_> = sorted_hand.iter().map(|&(_, suit)| suit).collect();
    let mut hand_rank: u16;
    let mut tie_breaker: &str;

    

    HandRank
}

/*
S - Spades
C - Clubs
H - Hearts
D - Diamonds
NO RANK in Suits!
*/

/*
Straight flush  -> 9
Four of a kind  -> 8
Full house      -> 7
Flush           -> 6
Straight        -> 5
Three of a kind -> 4
Two pair        -> 3
One pair        -> 2
High Card       -> 1
TOTAL : 9 ranks
*/
