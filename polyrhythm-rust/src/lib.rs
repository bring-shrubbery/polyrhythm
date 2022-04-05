mod lcm;

pub use crate::lcm::lcm_vec;

pub fn get_polyrhythm(beats: &Vec<usize>) -> Option<Vec<Vec<usize>>> {
  if beats.len() <= 1 {
    return None;
  }
  let lcm_val = lcm_vec(beats);
  let mut samples: Vec<Vec<usize>> = Vec::new();
  for beat_ptr in beats {
    let divisible_index = lcm_val / *beat_ptr;
    let beat_times_init: Vec<usize> = vec![0; lcm_val];
    let beat_times: Vec<usize> = beat_times_init
      .iter()
      .enumerate()
      .map(|(i, _)| if i % divisible_index == 0 { 1 } else { 0 })
      .collect();
    samples.push(beat_times);
  }
  Some(samples)
}

#[cfg(test)]
mod tests {
  use crate::get_polyrhythm;
  #[test]
  fn basic_3_2_polyrhythm() {
    let beats = vec![3, 2];
    let result = get_polyrhythm(&beats);

    match result {
      Some(res) => {
        println!("{:?}", res);
        assert_eq!(res, vec![vec![1, 0, 1, 0, 1, 0], vec![1, 0, 0, 1, 0, 0]]);
      }
      None => panic!("Should return value!"),
    }
  }
}
