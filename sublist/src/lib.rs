#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn list_compare<T: PartialEq>(longer_list: &[T], shorter_list: &[T]) -> bool {
    shorter_list.is_empty()
        || longer_list
            .windows(shorter_list.len())
            .any(|window| window == shorter_list)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_len = first_list.len();
    let second_len = second_list.len();

    if first_list == second_list {
        return Comparison::Equal;
    } else if first_len > second_len && list_compare(first_list, second_list) {
        return Comparison::Superlist;
    } else if first_len < second_len && list_compare(second_list, first_list) {
        return Comparison::Sublist;
    }
    else {
        Comparison::Unequal
    }
}
