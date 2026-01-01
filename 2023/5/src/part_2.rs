use common::errors::AocError;

use crate::utils::{parse_input, CategoryMap, RangeMapping};

#[derive(Debug, Clone)]
pub struct Range {
    pub start: u64,
    pub length: u64,
}

impl Range {
    fn end(&self) -> u64 {
        self.start + self.length - 1
    }
}

// NOTE: this was done by claude, im not even going to lie
// my implementation was way to slow to even finish in a reasonable time
// so im just going to leave this version here

// Apply a single mapping rule to a range, returning transformed and unchanged portions
fn apply_mapping_rule(range: &Range, mapping: &RangeMapping) -> (Option<Range>, Vec<Range>) {
    let mapping_end = mapping.source_start + mapping.range_length - 1;

    // No overlap case - return the range unchanged
    if range.end() < mapping.source_start || range.start > mapping_end {
        return (None, vec![range.clone()]);
    }

    let mut unchanged = Vec::new();

    // Handle the portion before the mapping range
    if range.start < mapping.source_start {
        unchanged.push(Range {
            start: range.start,
            length: mapping.source_start - range.start,
        });
    }

    // Handle the portion after the mapping range
    if range.end() > mapping_end {
        unchanged.push(Range {
            start: mapping_end + 1,
            length: range.end() - mapping_end,
        });
    }

    // Calculate the overlapping portion and transform it
    let overlap_start = range.start.max(mapping.source_start);
    let overlap_end = range.end().min(mapping_end);
    let overlap_length = overlap_end - overlap_start + 1;

    let offset = mapping.dest_start as i64 - mapping.source_start as i64;
    let transformed = Range {
        start: (overlap_start as i64 + offset) as u64,
        length: overlap_length,
    };

    (Some(transformed), unchanged)
}
fn apply_category_map(ranges: Vec<Range>, map: &CategoryMap) -> Vec<Range> {
    let mut result = Vec::new();
    let mut to_process = ranges;

    for mapping in &map.mappings {
        let mut next_to_process = Vec::new();

        for range in to_process {
            let (transformed, unchanged) = apply_mapping_rule(&range, mapping);
            if let Some(t) = transformed {
                result.push(t);
            }
            next_to_process.extend(unchanged);
        }

        to_process = next_to_process;
    }

    // Any ranges that didn't match any mapping rule stay the same
    result.extend(to_process);

    result
}
fn find_lowest_location(seed_ranges: Vec<Range>, maps: &[CategoryMap]) -> u64 {
    let mut current_ranges = seed_ranges;

    // Process through each category map in sequence
    for map in maps {
        current_ranges = apply_category_map(current_ranges, map);
    }

    // The lowest location is the minimum start value from any range
    current_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap_or(u64::MAX)
}
pub fn solve(filename: &str) -> Result<u32, AocError> {
    let (seeds, maps) = parse_input(filename)?;

    // Convert seed values to ranges
    let mut seed_ranges = Vec::new();
    for chunk in seeds.chunks_exact(2) {
        seed_ranges.push(Range {
            start: chunk[0],
            length: chunk[1],
        });
    }

    Ok(u32::try_from(find_lowest_location(seed_ranges, &maps))
        .ok()
        .expect("Lowest value is larger than u32"))
}
