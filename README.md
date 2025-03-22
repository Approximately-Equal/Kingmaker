# Kingmaker

Kingmaker is a modular, performant, social choice framework for the simulation, computation, and analysis of strategic voting. It has a focus on the study of strategic or tactical voting, which is concerned questions such as:

1. Which strategies are most effective (for each voting method)?
2. How much do they impact election outcomes?
3. Which voting methods minimize strategic voting?
4. ...etc

## Installation

You can take latest release from `crates.io`, or build from source using Cargo:

```sh
cargo install kingmaker
```

## Getting Started

To get started with Kingmaker, just configure the election parameters and run the simulation. Here's a minimal example:

```rust
use kingmaker::prelude::*;

fn main() {
    // configure election
    let election = Election::builder((), methods::Plurality)
        .add_candidate(0, "A", Some("DEM"), None)
        .add_candidate(1, "B", Some("REP"), None)
        .add_candidate(2, "C", None, None)
        .add_voting_block(
            preferences::Mallows::new(vec![0, 1, 2], 0.2),
            Strategy::new()
                .add_tactic(tactics::Identity, 0.8)
                .add_tactic(tactics::Burial(vec![1]), 0.2),
            5_000,
        )
        .add_voting_block(
            preferences::Mallows::new(vec![2, 1, 0], 0.15),
            Strategy::new()
                .add_tactic(tactics::Identity, 0.7)
                .add_tactic(tactics::Burial(vec![1]), 0.3),
            5_000,
        )
        .build();
    // run election(s)
    let outcomes = election.run_many(1_000, 0);
    // display outcome
    election.display(outcomes);
}
```

## Acknowledgments

- Much thanks to the folks at [pref_voting](https://github.com/voting-tools/pref_voting) for their comprehensive work on social choice research.
