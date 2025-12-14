# decaesar

Small `no_std` library for breaking Caesar ciphers.

No heap allocation.
Scoring is user-defined.

---

## Basic usage

```rust
#![no_std]

use decaesar::{Decaesar, DefaultScorer, decode_caesar};

let input = b"Khoor zruog";

// Create breaker with default scorer
let breaker = Decaesar::new(DefaultScorer);

// Try all shifts and score them
let result = breaker.break_caesar(input).unwrap();

// Best scoring shift
let best = result.best;

// Decode using the best shift
let mut output = [0u8; 11];
decode_caesar(input, &mut output, best.shift).unwrap();

assert_eq!(&output, b"Hello world");
```

---

## User-defined scoring

```rust
#![no_std]

use decaesar::{Decaesar, ScoreFunction};

struct SimpleScorer;

impl ScoreFunction for SimpleScorer {
    fn score(&self, input: &[u8], shift: u8) -> f32 {
        let mut score = 0.0;

        for &b in input {
            // local decode (same logic as library shift_byte, kept inline for the example)
            let c = if b.is_ascii_lowercase() {
                ((b - b'a' + shift) % 26) + b'a'
            } else if b.is_ascii_uppercase() {
                ((b - b'A' + shift) % 26) + b'A'
            } else {
                b
            };

            if c == b' ' {
                score += 5.0;
            } else if c.is_ascii_digit() {
                score -= 10.0;
            }
        }

        score
    }
}

let breaker = Decaesar::new(SimpleScorer);
let result = breaker.break_caesar(b"Ymnx nx f yjxy").unwrap();

assert_eq!(result.best.shift, 5);
```

---

## Top N results

`best_n(output, n)` writes up to `min(n, output.len(), 26)` items into `output`, sorted by score (best first).

```rust
#![no_std]

use decaesar::{Decaesar, DefaultScorer, DecipherResult};

let breaker = Decaesar::new(DefaultScorer);
let result = breaker.break_caesar(b"Khoor zruog").unwrap();

let mut top: [DecipherResult; 3] = [DecipherResult::default(); 3];
result.best_n(&mut top, 3);

// best guess first
assert_eq!(top[0].shift, 3);
```

---

## Notes

- `#![no_std]`
- No allocation
- Caller provides output buffers
- Scoring logic is controlled by the user
