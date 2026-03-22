# Quasar Merkle Tree

An on-chain incremental Merkle tree program built with [Quasar](https://github.com/blueshift-gg/quasar) on Solana.

### Instructions

| Discriminator | Instruction | Description |
|---------------|-------------|-------------|
| 0 | `initialize` | Creates the Merkle tree PDA and sets up the initial zero-hash state |
| 1 | `insert` | Inserts a 32-byte leaf into the tree and recomputes the root |
| 2 | `verify` | Verifies a Merkle proof for a given leaf at a given index |

