# Polyrhythm

Polyrhythm generation algorithm written in every language I need.

## Motivation

- [rythmic (App Store)](https://apps.apple.com/app/rythmic/id1515876711)

## Algorithm

Components of the polyrhythms:

- `N` number of simultaneous rhythms with each having:
  - It's own `T_i` tempo.
  - It's own number of beats per measure (time signature).
- A way to measure time. In this case we use two (based on preference):
  - Sample time based - returns specific integer that represents the sample time, given sample rate.
  - Index based - one index step is the smallest step between beats. This is effectively just sample rate approach, with sample rate set to 1.

Below you can see the pseudocode for the algorithm in Python.

```python
# Pseudocode, does not actually run.
def getPolyrhythm(beats: int[]) -> int[][]:
  if len(config.beats) <= 1: throw "Need to have some beats"

  LCM = least_common_multiple(config.beats) # least common multiple (LCM) of all the beats

  samples = []
  for beat in config.beats:
    # Find the index at which the beat will occur.
    divisible_index = LCM / beat
    beat_times = [1 if index % divisible_index == 0 else 0 for index in [0]*LCM]
    samples.append(beat_times)

  return samples
  # Samples now look like this for beats=[2, 3]:
  # [
  #   [1, 0, 0, 1, 0, 0],
  #   [1, 0, 1, 0, 1, 0]
  # ]

def getPolyrhythmSampleTimes(samples: int[][], bpm: float, sample_rate: float): -> float[][]:
  pass
  # TODO: Finish the func
  # 1. Normalize samples values
  # 2. Multiply by sample time
```

```bash
$ python main.py -b 2,3

[1, 0, 0, 1, 0, 0]
[1, 0, 1, 0, 1, 0]
```

## Installation

### Swift

Add following URL to Swift Package Manager in Xcode:

```
https://github.com/bring-shrubbery/polyrhythm/tree/main/polyrhythm-rust
```

### Rust

Add polyrhythm crate to your `Cargo.toml` file:

```toml
polyrhythm = "x.x.x"
```

## License

[Mozilla Public License 2.0](https://github.com/bring-shrubbery/polyrhythm/blob/main/LICENSE)
