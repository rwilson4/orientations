# Benchmarks

All benchmarks run on a 2017 Macbook Pro with a 3.1 GHz Intel Core i7
processor and 16 GB 2133 MHz LPDDR3 RAM.

Struct | Function | Version | Runtime (ns) | Throughput (Mops/s)
-------|----------|---------|-------------:|-------------------:
quaternion |  | rotate\_vector |    9.5 |  105.0
quaternion | angle\_axis | angle\_axis |   42.3 |   23.6
quaternion | before\_after | after |   42.7 |   23.4
quaternion | before\_after | after\_safe |   46.9 |   21.3
quaternion | before\_after | before |   21.3 |   46.9
quaternion | from\_angle\_axis | from\_angle\_axis |   25.4 |   39.4
quaternion | inverse | inverse |    5.4 |  184.3
quaternion | inverse | inverse\_without\_check |    3.3 |  302.8
vector3d | cross\_product | cross\_product |    2.3 |  442.4
vector3d | dot\_product | dot\_product |    1.6 |  623.6