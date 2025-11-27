# Toy Payment Engine

A toy payment engine that handles deposits, withdrawals, disputes, resolutions, and chargebacks, and outputs a dump of the resulting client account/balance data;

## Usage

```bash
cargo run -- transactions.csv > accounts.csv
```

## Assumptions

- Only deposits can be disputed (withdrawals don't make sense in the way they are described in the assignment)
- Disputes require sufficient available funds to prevent negative balances
- Locked/frozen accounts ignore all subsequent transactions
- Invalid disputes/resolves/chargebacks (non-existent tx, not under dispute) are silently ignored
- Using f32 for monetary amounts for the sake of simplicity (BIG NONO IN THE REAL WORLD! 0.1$ + 0.2$ != 0.30000000000000004$)

## Correctness

- Mainly verified through manual testing due to time constraints
- Type system ensures valid event transitions
