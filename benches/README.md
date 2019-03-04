# Benchmarks

All benchmarks run on a 2017 Macbook Pro with a 3.1 GHz Intel Core i7
processor and 16 GB 2133 MHz LPDDR3 RAM.

Group | Function | Runtime (ns) | Throughput (Mops/s)
------|----------|-------------:|-------------------:
angle\_axis | angle\_axis |   42.2 |   23.7
before\_after | after |   45.8 |   21.8
before\_after | after\_safe |   46.3 |   21.6
before\_after | before |   20.8 |   48.1
cross\_product | cross\_product |    2.2 |  451.8
dot\_product | dot\_product |    1.6 |  625.7
from\_angle\_axis | from\_angle\_axis |   26.0 |   38.5
inverse | inverse |    5.5 |  183.4
inverse | inverse\_without\_check |    3.3 |  300.9