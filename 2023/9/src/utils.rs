use common::{errors::AocError, input::InputReader};

pub(crate) fn parse_input(filename: &str) -> Result<Vec<Vec<i64>>, AocError> {
    let lines: Vec<Vec<i64>> = InputReader::as_lines(filename)?
        .iter()
        .map(|s| s.split(" ").map(|d| d.parse().unwrap()).collect())
        .collect();
    Ok(lines)
}

fn differentiate(series: &Vec<i64>) -> Vec<i64> {
    return series.windows(2).map(|e| e[1] - e[0]).collect();
}

pub(crate) fn forecast(series: &Vec<i64>) -> i64 {
    let last_el = series.last().unwrap();
    let first_diff = differentiate(series);

    if differentiate(&first_diff).iter().all(|d| *d == 0) {
        return last_el + first_diff.last().unwrap();
    } else {
        return last_el + forecast(&first_diff);
    }
}

pub(crate) fn backcast(series: &Vec<i64>) -> i64 {
    let first_el = series.first().unwrap();
    let first_diff = differentiate(series);

    if differentiate(&first_diff).iter().all(|d| *d == 0) {
        return first_el - first_diff.first().unwrap();
    } else {
        return first_el - backcast(&first_diff);
    }
}
