/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        vec![hands[0]]
    } else {
        todo!()
    }
}


// S - Spades
// C - Clubs
// H - Hearts
// D - Diamonds
// NO RANK in Suits!

