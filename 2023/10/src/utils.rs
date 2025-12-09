use aoc_common_rust::errors::AocError;
use aoc_common_rust::input::InputReader;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum Tile {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
    G_ = b'.',
    S_ = b'S',
    _X,
}

impl TryFrom<&char> for Tile {
    type Error = AocError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match *value {
            '|' => Ok(Self::NS),
            '-' => Ok(Self::EW),
            'L' => Ok(Self::NE),
            'J' => Ok(Self::NW),
            '7' => Ok(Self::SW),
            'F' => Ok(Self::SE),
            '.' => Ok(Self::G_),
            'S' => Ok(Self::S_),
            _ => Err(AocError::Parse(format!(
                "Invalid tile character: {}",
                value
            ))),
        }
    }
}

pub(crate) fn parse_input(filename: &str) -> Vec<Vec<Tile>> {
    InputReader::as_char_grid(filename)
        .unwrap()
        .iter()
        .map(|row| row.iter().map(|e| Tile::try_from(e).unwrap()).collect())
        .collect()
}
