use std::vec;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
struct HandRank<'a> {
    hand: &'a str,
    rank: u16,
    tie_breaker: String,
    index: usize,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        vec![hands[0]]
    } else if hands.iter().all(|hand| hand.replace(" ", "").len() == 10) {
        let mut ranking: Vec<HandRank> = vec![];
        for (idx, hand) in hands.iter().enumerate() {
            let mut hand_rank = HandRank {
                hand: hand,
                rank: 0,
                tie_breaker: String::new(),
                index: idx,
            };
            hand_rank = rank_hand(hand_rank);
            ranking.push(hand_rank);
        }
        todo!()
    } else {
        panic!("Card Hands are not Valid!!")
    }
}

fn rank_hand(hand_rank: HandRank<'_>) -> HandRank {
    let mut hand_rank = hand_rank;
    let sorted_hand: Vec<(u16, &str)> = hand_rank.hand
        .split_whitespace()
        .map(|card| {
            let (num, suit) = card.split_at(1);
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

    // Flush (+ Straight?)
    if suits.iter().eq(suits.iter()) {
        let mut is_straight = true;

        numbers.sort();
        let mut prev_number = numbers[0];
        for &number in &numbers[1..] {
            if number != prev_number + 1 {
                // Flush
                if hand_rank.rank < 4 {
                    hand_rank.rank = 4;
                    hand_rank.tie_breaker = numbers.iter().max().unwrap().to_string();
                }
                is_straight = false;
                break;
            }
            prev_number = number;
        }
        if is_straight {
            // Straight Flush
            if hand_rank.rank < 4 {
                hand_rank.rank = 1;
                hand_rank.tie_breaker = numbers.iter().max().unwrap().to_string();
            }
        }
    }

    // Pairs


    hand_rank
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
