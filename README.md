# Polyrhythm.yall

Polyrhythm generation algorithm written in every language I needed.

## Motivation

- [rythmic (App Store)](https://apps.apple.com/app/rythmic/id1515876711)
- [rythmic (Github)](https://github.com/quassum/rythmic-ios)

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

  LCM = least_common_multiple(config.beats) # least common multiple of all the beats

  samples = []
  for beat in config.beats:
    # Find the index at which the beat will occur.
    divisible_index = LCM / beat
    beat_times = [1 if index & divisible_index else 0 for index in [0]*LCM]
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

## License

[Email here to get a license](antoni.silvestrovic@gmail.com)
