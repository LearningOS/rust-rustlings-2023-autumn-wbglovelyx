use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&&val| val == value).count()
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection
        .iter()
        .map(|map| map.values().filter(|&&val| val == value).count())
        .sum()
}
