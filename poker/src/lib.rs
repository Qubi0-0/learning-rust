use std::{collections::HashMap, vec};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
struct HandRank<'a> {
    hand: &'a str,
    rank: u16,
    tie_breaker: u16,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        vec![hands[0]]
    } else {
        let mut ranking: Vec<HandRank> = vec![];
        for hand in hands {
            let mut hand_rank = HandRank {
                hand: hand,
                rank: 9,
                tie_breaker: 0,
            };
            hand_rank = HandRank::<'_>::rank_hand(hand_rank);
            ranking.push(hand_rank);
        }
        ranking.sort_by(|a, b| {
            if a.rank == b.rank {
                a.tie_breaker.cmp(&b.tie_breaker)
            } else {
                a.rank.cmp(&b.rank)
            }
        });
        let highest_rank = ranking[0].rank;
        let highest_tie_breaker = ranking[0].tie_breaker;
        ranking
            .into_iter()
            .filter(|hand_rank| {
                hand_rank.rank == highest_rank && hand_rank.tie_breaker == highest_tie_breaker
            })
            .map(|hand_rank| hand_rank.hand)
            .collect()
    }
}
impl<'a> HandRank<'a> {
    fn rank_hand(hand_rank: HandRank<'a>) -> HandRank<'a> {
        let mut hand_rank = hand_rank;
        let sorted_hand: Vec<(u16, &str)> = hand_rank
            .hand
            .split_whitespace()
            .map(|card| {
                let (num, suit) = card.split_at(card.len() - 1);
                let num = match num {
                    "2" => 2,
                    "3" => 3,
                    "4" => 4,
                    "5" => 5,
                    "6" => 6,
                    "7" => 7,
                    "8" => 8,
                    "9" => 9,
                    "10" => 10,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    _ => panic!("Invalid card value!"),
                };
                (num, suit)
            })
            .collect();
        let mut numbers: Vec<_> = sorted_hand.iter().map(|&(num, _)| num).collect();
        let suits: Vec<_> = sorted_hand.iter().map(|&(_, suit)| suit).collect();

        hand_rank.tie_breaker = *numbers.iter().max().unwrap();
        // Checking if hand is straight
        let mut is_straight = check_straight(&mut numbers);
        if is_straight == false && numbers.iter().max().unwrap() == &14 {
            numbers = numbers
                .iter()
                .map(|&number| if number == 14 { 1 } else { number })
                .collect::<Vec<u16>>();
            is_straight = check_straight(&mut numbers);
        }
        // Flush
        if suits.iter().all(|&suit| suit == suits[0]) {
            if is_straight {
                // Straight Flush
                if hand_rank.rank > 1 {
                    hand_rank.rank = 1;
                    hand_rank.tie_breaker = 15 -  *numbers.iter().max().unwrap();
                    return hand_rank;
                }
            } else {
                // Flush
                if hand_rank.rank > 4 {
                    hand_rank.rank = 4;
                    hand_rank.tie_breaker = 15 - *numbers.iter().max().unwrap();
                    return hand_rank;
                }
            }
        } else {
            if is_straight {
                if hand_rank.rank > 5 {
                    hand_rank.rank = 5;
                    hand_rank.tie_breaker = 15 - *numbers.iter().max().unwrap();
                    return hand_rank;
                }
            }
        }

        // Pairs
        let mut pair_hm: HashMap<u16, u16> = HashMap::new();
        for &number in &numbers {
            if pair_hm.contains_key(&number) {
                let count = pair_hm.get_mut(&number).unwrap();
                *count += 1;
            } else {
                pair_hm.insert(number, 1);
            }
        }

        let mut house_check = false;
        let mut two_pair_check = false;

        let mut pair_vec: Vec<_> = pair_hm.into_iter().collect();
        pair_vec.sort_by(|number, count| count.1.cmp(&number.1));

        for (number, count) in &pair_vec {
            match *count {
                4 => {
                    // Four of a kind
                    if hand_rank.rank > 2 {
                        hand_rank.rank = 2;
                        hand_rank.tie_breaker = 15 - *number;
                        return hand_rank;
                    }
                }
                3 => {
                    // is Full house?
                    if house_check {
                        if hand_rank.rank > 3 {
                            hand_rank.rank = 3;
                            hand_rank.tie_breaker = 15 - *number;
                            return hand_rank;
                        }
                    } else {
                        if hand_rank.rank > 6 {
                            hand_rank.rank = 6;
                            hand_rank.tie_breaker = 15 - *number;
                            house_check = true;
                        }
                    }
                }
                2 => {
                    // is Full house?
                    if house_check {
                        if hand_rank.rank > 3 {
                            hand_rank.rank = 3;
                            return hand_rank;
                        }
                    } else if two_pair_check {
                        if hand_rank.rank > 7 {
                            hand_rank.rank = 7;
                            if hand_rank.tie_breaker > *number {
                                hand_rank.tie_breaker = 15 - number;
                            }
                            return hand_rank;
                        }
                    } else {
                        // First Pair
                        if hand_rank.rank > 8 {
                            hand_rank.rank = 8;
                            hand_rank.tie_breaker = 15 - number;
                            two_pair_check = true;
                        }
                    }
                }
                1 => {
                    if hand_rank.rank == 9 {
                        hand_rank.tie_breaker =
                            numbers.iter().map(|&number| 15 - number).min().unwrap();
                    }
                }
                _ => {}
            }
        }
        hand_rank
    }
}

fn check_straight(numbers: &mut Vec<u16>) -> bool {
    numbers.sort();
    let mut prev_number = numbers[0];
    for &number in &numbers[1..] {
        if number != prev_number + 1 {
            return false;
        }
        prev_number = number;
    }
    true
}

/*
S - Spades
C - Clubs
H - Hearts
D - Diamonds
NO RANK in Suits!
*/

/*
Straight flush  -> 1
Four of a kind  -> 2
Full house      -> 3
Flush           -> 4
Straight        -> 5
Three of a kind -> 6
Two pair        -> 7
One pair        -> 8
High Card       -> 9
TOTAL : 9 ranks
*/
