## Setup

```bash
git clone git@github.com:jhrcook/advent-of-code-2023-rust.git
cd advent-of-code-2023-rust
```

## Execute puzzles

Test:

```bash
cargo check
cargo test
```

Dev build and run:

```bash
cargo run --release
cargo run --release -- --day 1
```

Full install and run:

```bash
cargo install --path .
aoc-2023
aoc-2023 --day 1
```

For estimating start-up time:

```bash
aoc-2023 --empty
```
