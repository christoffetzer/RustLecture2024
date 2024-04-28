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

```bash
cargo bench -- --save-baseline opt0-no-overflow-check
```

cargo bench -- --load-baseline new --baseline opt1-overflow-check

cargo bench -- --load-baseline new2 --baseline opt3-no-overflow-check

Opt3 vs Opt 0 no-overflow

cargo bench -- --baseline opt0-no-overflow-check  --load-baseline opt3-no-overflow-check


```toml
[profile.bench]
overflow-checks = true
opt-level = 2
```

```bash
cargo bench --profile opt0oc -- --save-baseline opt0oc
```

```bash
cargo bench --profile opt1noc -- --save-baseline opt1noc
```

```bash
cargo bench --profile opt3oc -- --save-baseline opt3oc
```

```bash
cargo bench --profile opt2noc -- --save-baseline opt2noc
```

```bash
cargo bench --profile opt0noc -- --save-baseline opt0noc
```

```bash
cargo bench --profile opt3noc -- --save-baseline opt3noc
```

cargo bench -- --baseline opt0oc  --load-baseline opt3oc
cargo bench -- --baseline opt0noc  --load-baseline opt3noc
cargo bench -- --baseline opt3oc  --load-baseline opt3noc
cargo bench -- --baseline opt3oc  --load-baseline opt0noc

cargo bench -- --baseline opt0noc --load-baseline opt3noc