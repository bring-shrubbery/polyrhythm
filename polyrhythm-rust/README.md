# Polyrhythm

Generate polyrhythms in Rust.

## Usage

### `fn get_polyrhythm(beats: &Vec<usize>) -> Option<Vec<Vec<usize>>>`

- **Input:** Accepts a vector of number of beats for each track.
- **Output:** Returns a vector of same-length vectors, that contain either 1 or 0 indicating beat or no beat respectively. The number of the nested vectors is determined by a least common multiple of the beats in each track.

**Example usage:**

```rust
let beats = vec![2, 3];
let result = get_polyrhythm(&beats);

// result:
// vec![
//   vec![1, 0, 0, 1, 0, 0],
//   vec![1, 0, 1, 0, 1, 0]
// ]
```

## License

[Mozilla Public License 2.0](https://github.com/bring-shrubbery/polyrhythm/blob/main/polyrhythm-rust/LICENSE)
