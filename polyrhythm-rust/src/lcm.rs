pub fn lcm_vec(vec: &Vec<usize>) -> usize {
  let mut result: usize = 1;
  for ptr in vec {
    result = lcm(result, *ptr);
  }
  result
}

pub fn lcm(first: usize, second: usize) -> usize {
  first * second / gcd(first, second)
}

pub fn gcd(first: usize, second: usize) -> usize {
  let mut max = first;
  let mut min = second;
  if min > max {
    let val = max;
    max = min;
    min = val;
  }
  loop {
    let res = max % min;
    if res == 0 {
      return min;
    }
    max = min;
    min = res;
  }
}
