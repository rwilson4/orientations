# Benchmarks

All benchmarks run on a 2017 Macbook Pro with a 3.1 GHz Intel Core i7
processor and 16 GB 2133 MHz LPDDR3 RAM.

Struct | Function | Version | Runtime (ns) | Throughput (Mops/s)
-------|----------|---------|-------------:|-------------------:
quaternion | angle\_axis | angle\_axis |   42.2 |   23.7
quaternion | before\_after | after |   43.0 |   23.3
quaternion | before\_after | after\_safe |   46.3 |   21.6
quaternion | before\_after | before |   19.5 |   51.3
quaternion | from\_angle\_axis | from\_angle\_axis |   25.6 |   39.1
quaternion | inverse | inverse |    5.5 |  183.1
quaternion | inverse | inverse\_without\_check |    3.3 |  300.4
vector3d | cross\_product | cross\_product |    2.2 |  448.7
vector3d | dot\_product | dot\_product |    1.6 |  625.2