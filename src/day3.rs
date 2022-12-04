use std::collections::HashSet;
use crate::utils::get_input;

pub fn part1() -> u32 {
    let mut set = HashSet::new();
    let mut priority_sum = 0;

    for line in get_input(3).lines() {
        set.clear();

        let bytes = line.bytes();
        let mid = bytes.len() / 2;

        for (index, byte) in bytes.enumerate() {
            if index < mid {
                set.insert(byte);
            } else if set.contains(&byte) {
                // Found the duplicate
                priority_sum += priority_of(byte) as u32;
                break;
            }

            // Note that we don't add byte to the set here because then
            // we'd find duplicates just contained within the second half
            // of the rucksack
        }
    }

    priority_sum
}

pub fn part2() -> u32 {
    let mut priority_sum = 0;

    for group in get_input(3).lines().array_chunks::<3>() {
        let group = group.map(str::as_bytes);

        let mut set: HashSet<&u8> = HashSet::from_iter(group[0]);
        set.retain(|e| group[1].contains(e));
        set.retain(|e| group[2].contains(e));

        assert_eq!(set.len(), 1);
        priority_sum += priority_of(**set.iter().next().unwrap()) as u32;
    }

    priority_sum
}

fn priority_of(v: u8) -> u8 {
    match v as char {
        'a'..='z' => v - ('a' as u8) + 1,
        'A'..='Z' => v - ('A' as u8) + 27,
        _ => unreachable!()
    }
}
