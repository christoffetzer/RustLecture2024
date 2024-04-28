# Benchmark

Run as follows:

Set in top-level Cargo.toml:

```toml
[profile.bench]
overflow-checks = true
opt-level = 0
```

```bash
cargo bench -- --save-baseline opt1-overflow-check
```

```toml
[profile.bench]
overflow-checks = false
opt-level = 3
```

cargo bench -- --save-baseline opt3-no-overflow-check


```toml
[profile.bench]
overflow-checks = false
opt-level = 3
```

```toml
[profile.bench]
overflow-checks = true
opt-level = 2
```

```bash
cargo bench --profile opt0noc --   --save-baseline checked_0noc
cargo bench --profile opt1noc --   --save-baseline checked_1noc
cargo bench --profile opt2noc --   --save-baseline checked_2noc
cargo bench --profile opt3noc --   --save-baseline checked_3noc
```
