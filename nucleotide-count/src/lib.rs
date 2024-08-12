use std::collections::HashMap;

// A, C, G, T

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Ok(result) => Ok(*result.get(&nucleotide).unwrap_or(&0)),
        Err(invalid_char) => Err(invalid_char),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = HashMap::new();

    for &nucleotide in &['A', 'C', 'G', 'T'] {
        result.insert(nucleotide, 0);
    }

    for ch in dna.chars() {
        if is_valid_nucleotide(ch) {
            let count = result.entry(ch).or_insert(0);
            *count += 1;
        } else {
            return Err(ch);
        }
    }
    Ok(result)
}

fn is_valid_nucleotide(ch: char) -> bool {
    matches!(ch, 'A' | 'C' | 'G' | 'T')
}
