# Framing Experiment

## Goal

Verify that the length-prefixed decoder correctly handles stream
fragmentation, multiple frames, and oversized frames.

## Methodology

Run the deterministic unit tests in `crates/forge-network/src/framing.rs`:

```bash
cargo test -p forge-network framing
```

Scenarios covered:

- Complete frame in one buffer
- Partial header
- Partial payload
- Multiple frames in one buffer
- Oversized frame rejected
- Zero-length payload accepted

## Interpretation

The decoder never allocates more than the configured maximum and never returns
a partial frame. This is the minimum correctness requirement for any TCP
application protocol.
