use std::{collections::HashMap, vec};
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        vec![hands[0]]
    } else {

        let mut points_map: HashMap<&str, u32> = HashMap::new();

        for hand in hands.iter() {
            let points = rank_hand(hand);
            points_map.insert(hand, points);
        }

        let (best_hand, _) = points_map.iter().max_by_key(|&(_, points)| points).unwrap();

        vec![*best_hand]
    }
}

fn rank_hand(hand: &str) -> u32 {
    let mut rank_to_value: HashMap<&str, u32> = HashMap::new();
    rank_to_value.insert("2", 2);
    rank_to_value.insert("3", 3);
    rank_to_value.insert("4", 4);
    rank_to_value.insert("5", 5);
    rank_to_value.insert("6", 6);
    rank_to_value.insert("7", 7);
    rank_to_value.insert("8", 8);
    rank_to_value.insert("9", 9);
    rank_to_value.insert("10", 10);
    rank_to_value.insert("J", 11);
    rank_to_value.insert("Q", 12);
    rank_to_value.insert("K", 13);
    rank_to_value.insert("A", 14);

    let chunks = hand
        .split(' ')
        .map(|card| {
            let rank = &card[..card.len()-1];
            let value = rank_to_value.get(rank);
            match value {
                Some(v) => *v,
                None => panic!("{} is NOT a valid card!", card),
            }
        })
        .collect::<Vec<u32>>();

    let result = chunks.iter().max().unwrap();

    *result
}

/*
S - Spades
C - Clubs
H - Hearts
D - Diamonds
NO RANK in Suits!
*/
