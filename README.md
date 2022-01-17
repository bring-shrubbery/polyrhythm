# Polyrhythm.swift

Polyrhythm generation algorithm written in Swift

## Motivation

## Algorithm

Components of the polyrhythms:

- N number of simultaneous rhythms with each having:
  - It's own T_i tempo. 
  - It's own number of beats per measure (time signature).
- A way to measure time. In this case we use two (based on preference):
  - Sample time based - returns specific integer that represents the sample time, given sample rate.
  - Index based - one index step is the smallest step between beats. This is effectively just sample rate approach, with sample rate set to 1.


Below you can see the pseudocode for the algorithm in Python.

```python
def getPolyrhythm(config: { items: { tempo: float, beats: int }[], sample_rate: int }) -> int[][]:
    # TODO
```

## License

[Email here to get a license](antoni.silvestrovic@gmail.com)
