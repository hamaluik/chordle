pub fn mean<'i, I>(iter: I) -> f64
where
    I: Iterator<Item = &'i f64>,
{
    let mut sum = 0.0;
    let mut count = 0;

    for value in iter {
        sum += value;
        count += 1;
    }

    if count == 0 {
        return 0.0;
    }

    sum / count as f64
}

pub fn median<'i, I>(iter: I) -> f64
where
    I: Iterator<Item = &'i f64>,
{
    let mut values: Vec<f64> = iter.cloned().collect();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = values.len();
    if len == 0 {
        return 0.0;
    }

    if len % 2 == 0 {
        (values[len / 2 - 1] + values[len / 2]) / 2.0
    } else {
        values[len / 2]
    }
}

pub fn variance<'i, I>(mean: f64, iter: I) -> f64
where
    I: Iterator<Item = &'i f64>,
{
    let mut sum_squared_diff = 0.0;
    let mut count = 0;

    for value in iter {
        let diff = value - mean;
        sum_squared_diff += diff * diff;
        count += 1;
    }

    if count == 0 {
        return 0.0;
    }

    sum_squared_diff / count as f64
}
