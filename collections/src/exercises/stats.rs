use std::collections::HashMap;

#[derive(Debug)]
pub struct MeanMedianMode {
  mean: f64,
  median: f64,
  mode: i32,
}

pub fn get_mean_median_mode(v: &Vec<i32>) -> MeanMedianMode {
  MeanMedianMode {
    mean: get_mean(&v),
    median: get_median(&v),
    mode: get_mode(&v),
  }
}

fn get_occurances(v: &Vec<i32>) -> HashMap<i32, i32> {
  let mut occurances = HashMap::new();
  for i in v {
    occurances
      .entry(*i)
      .and_modify(|o| *o += 1)
      .or_insert(1);
  }

  occurances
}

fn get_mean(v: &Vec<i32>) -> f64 {
  let sum = v.iter()
    .cloned()
    .reduce(|x, total| { x + total });

  match sum {
    Some(x) => x as f64 / v.len() as f64,
    None => 0.0
  }
}

fn get_median(v: &Vec<i32>) -> f64 {
  let mut min = match v.first() { Some(x) => *x, None => 0 };
  let mut max = min;

  for value in v {
    min = min.min(*value);
    max = max.max(*value);
  }

  (max - min) as f64 / 2.0 + min as f64
}

fn get_mode(v: &Vec<i32>) -> i32 {
  let occurances = get_occurances(v);
  let mut most_common = (&0, &0);

  for (value, occurances) in &occurances {
    if most_common.1 < occurances {
      most_common = (value, occurances);
    }
  }

  *most_common.0
}