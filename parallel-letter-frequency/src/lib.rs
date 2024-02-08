use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = vec![];

    if input.is_empty() || worker_count == 0 {
        return HashMap::new();
    }

    let text = input.join("");
    let chunk_size = std::cmp::max(1, text.len() / worker_count);

    let chunks: Vec<_> = text
        .chars()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect();
    for chunk in chunks {
        let handle = thread::spawn(move || string_counter(&chunk));
        handles.push(handle);
    }
    

    let mut frequency = HashMap::new();

    for handle in handles {
        let thread_frequency = handle.join().unwrap();
        for (key, value) in thread_frequency {
            *frequency.entry(key).or_insert(0) += value;
        }
    }

    frequency
}

fn string_counter(input: &str) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            *frequency.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    frequency
}
