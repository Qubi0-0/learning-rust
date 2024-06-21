use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant_dictionary: HashMap<&str, &str> = HashMap::from([
        ("G", "grass"),
        ("C", "clover"),
        ("R", "radishes"),
        ("V", "violets"),
    ]);
    let mut result: Vec<&'static str> = Vec::new();
    let student_list = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let (up, down) = diagram.split_at(diagram.find('\n').unwrap());
    if let Some(student_index) = student_list.iter().position(|&s| s == student) {
        let plant_codes_up = &up[student_index * 2..student_index * 2 + 2];
        let plant_codes_down = &down[student_index * 2 + 1..student_index * 2 + 3];
        let plant_codes = format!("{}{}", plant_codes_up, plant_codes_down);
        for code in plant_codes.chars() {
            if let Some((_, plant)) = plant_dictionary
                .iter()
                .find(|&(k, _)| k == &code.to_string())
            {
                result.push(plant);
            }
        }
    }
    result
}
