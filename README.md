# Orientations in Rust

<table>
<tr>
  <td>Build Status</td>
  <td>
    <a href="https://travis-ci.org/rwilson4/orientations">
    <img src="https://travis-ci.org/rwilson4/orientations.svg?branch=master&label=Travis%20CI" alt="travis build status" />
    </a>
  </td>
</tr>
<tr>
  <td>Code Coverage</td>
  <td>
    <a href="https://codecov.io/gh/rwilson4/orientations">
    <img src="https://codecov.io/gh/rwilson4/orientations/branch/master/graph/badge.svg" />
    </a>
  </td>
</tr>
</table>

Code to accompany [Rotations, Orientations, and their Representations](https://www.adventuresinwhy.com/post/quaternions-and-rotations/)
on [Adventures in Why](https://www.adventuresinwhy.com/).

## Architecture

There is a `Rotation` trait that defines the methods a rotation should
support. This trait is implemented by the `Quaternion` struct. For API
details, run `cargo doc --open` which will generate and open the
documentation in a browser. This documentation is also hosted at
[ConvexAnalytics.com](https://www.convexanalytics.com/orientations/orientations/index.html).

## Test cases

To run the test cases, run `cargo test`. The test cases are within the
source files. I would typically put the test cases in a separate
folder, and it looks like many Rustaceans do the same; however, it
doesn't look like I can test private functions this way. I might move
towards having integration tests in a separate folder and unit tests
stay with the code. In this particular code base, integration tests
don't seem as applicable.

## Benchmarks

Benchmarks are available in the `benches` folder. The `README` in that
folder contains a summary of the results. To run the benchmarks, run
`cargo bench`. To process the results and update the `README` in the
`benches` folder, run `python etc/process_benchmarks.py`.