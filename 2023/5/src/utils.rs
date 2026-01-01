use common::{errors::AocError, input::InputReader};

pub struct RangeMapping {
    pub dest_start: u64,
    pub source_start: u64,
    pub range_length: u64,
}

pub struct CategoryMap {
    pub mappings: Vec<RangeMapping>,
}

impl CategoryMap {
    pub fn convert(&self, source: u64) -> u64 {
        for mapping in &self.mappings {
            if source >= mapping.source_start
                && source < mapping.source_start + mapping.range_length
            {
                return mapping.dest_start + source - mapping.source_start;
            }
        }
        source
    }
}

pub fn parse_input(filename: &str) -> Result<(Vec<u64>, Vec<CategoryMap>), AocError> {
    let mut blocks = InputReader::as_paragraphs(filename)?.into_iter();

    let seeds_section = blocks
        .next()
        .ok_or_else(|| AocError::from("Missing seeds section"))?;
    let seeds: Vec<u64> = seeds_section
        .strip_prefix("seeds: ")
        .ok_or_else(|| AocError::from("Invalid seeds format"))?
        .split_whitespace()
        .map(|s| s.parse().map_err(AocError::from))
        .collect::<Result<_, _>>()?;

    let mut maps = Vec::new();
    for block in blocks {
        let mut lines = block.lines();
        lines.next();

        let mut mappings = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let nums: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse().map_err(AocError::from))
                .collect::<Result<_, _>>()?;

            if nums.len() != 3 {
                return Err(AocError::from(format!("Invalid mapping line: {}", line)));
            }

            mappings.push(RangeMapping {
                dest_start: nums[0],
                source_start: nums[1],
                range_length: nums[2],
            });
        }
        maps.push(CategoryMap { mappings });
    }
    Ok((seeds, maps))
}
