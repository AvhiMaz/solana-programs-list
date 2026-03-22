# Quasar Counter

A simple counter program built with [Quasar](https://github.com/blueshift-gg/quasar) on Solana.

## Program

Program ID: `9QWoDrTfMpWpgeh8qP9FqY6NdyqordB6qbfndEw6HjxY`

### State

- **Counter** — PDA derived from `[b"counter"]`, stores a `count` (u64) and `bump` (u8)

### Instructions

| Discriminator | Instruction | Description |
|---------------|-------------|-------------|
| 0 | `initialize` | Creates the counter PDA and sets count to 0 |
| 1 | `increment` | Increments the counter by 1 |
| 2 | `decrement` | Decrements the counter by 1 |

### Testing

Tests use [quasar-svm](https://github.com/blueshift-gg/quasar-svm) to execute instructions locally without a validator. Accounts are committed to the SVM store after each successful instruction, so subsequent calls only need to pass new or overridden accounts.

## Build

```bash
quasar build
```

## Test

```bash
quasar test
```
