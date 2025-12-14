#![no_std]

pub const LETTER_WEIGHTS: [f32; 26] = [
    8.12,  // A
    1.49,  // B
    2.71,  // C
    4.32,  // D
    12.02, // E
    2.30,  // F
    2.03,  // G
    5.92,  // H
    7.31,  // I
    0.10,  // J
    0.69,  // K
    3.98,  // L
    2.61,  // M
    6.95,  // N
    7.68,  // O
    1.82,  // P
    0.11,  // Q
    6.02,  // R
    6.28,  // S
    9.10,  // T
    2.88,  // U
    1.11,  // V
    2.09,  // W
    0.17,  // X
    2.11,  // Y
    0.07,  // Z
];

pub const COMMON_BIGRAMS: [(u8, u8); 20] = [
    (b't', b'h'),
    (b'h', b'e'),
    (b'i', b'n'),
    (b'e', b'r'),
    (b'a', b'n'),
    (b'r', b'e'),
    (b'o', b'n'),
    (b'a', b't'),
    (b'e', b'n'),
    (b'n', b'd'),
    (b's', b't'),
    (b't', b'o'),
    (b'e', b's'),
    (b'o', b'f'),
    (b'i', b's'),
    (b'i', b't'),
    (b'a', b's'),
    (b'a', b'l'),
    (b'a', b'r'),
    (b'l', b'e'),
];

impl core::fmt::Display for DecipherResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::write(
            f,
            format_args!("Shift: {} Score: {}", self.shift, self.score),
        )
    }
}

impl DecaesarResult {
    pub fn best(&self) -> DecipherResult {
        self.best
    }

    pub fn best_n(&self, output: &mut [DecipherResult], n: usize) {
        let mut sorted = self.results;
        sorted.sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(core::cmp::Ordering::Equal)
        });

        let k = core::cmp::min(n, core::cmp::min(output.len(), sorted.len()));
        for i in 0..k {
            output[i] = sorted[i];
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct DecipherResult {
    pub shift: u8,
    pub score: f32,
}

#[derive(Default, Debug)]
pub struct DecaesarResult {
    pub best: DecipherResult,
    pub results: [DecipherResult; 26],
}

#[derive(Debug)]
pub enum DecaesarError {
    EmptyInput,
    OutputTooSmall { required: usize, provided: usize },
    InvalidShift(u8),
}

pub trait ScoreFunction {
    fn score(&self, input: &[u8], shift: u8) -> f32;
}

pub struct DefaultScorer;

impl ScoreFunction for DefaultScorer {
    fn score(&self, input: &[u8], shift: u8) -> f32 {
        let mut score: f32 = 0.0;
        let mut prev: u8 = 0;

        for &i in input {
            let b = i.to_ascii_lowercase();
            let shifted = shift_byte(b, shift);

            for (a, b) in COMMON_BIGRAMS {
                if a == prev && b == shifted {
                    score += 20.0;
                    break;
                }
            }

            if shifted.is_ascii_lowercase() {
                score += LETTER_WEIGHTS[(shifted - b'a') as usize];
                prev = shifted;
            } else {
                prev = 0;
            }
        }

        score
    }
}

pub struct Decaesar<S> {
    scorer: S,
}

impl<S: ScoreFunction> Decaesar<S> {
    pub const fn new(scorer: S) -> Self {
        Self { scorer }
    }

    // For a given string in byte representation (char slice, pointer to the contiguous byte buffer + length), brute force all possible shifts and score them
    pub fn break_caesar(&self, input: &[u8]) -> Result<DecaesarResult, DecaesarError> {
        if input.is_empty() {
            return Err(DecaesarError::EmptyInput);
        }

        let mut results: [DecipherResult; 26] = [DecipherResult {
            shift: 0,
            score: 0.0,
        }; 26];
        let mut best = DecipherResult {
            shift: 0,
            score: f32::NEG_INFINITY,
        };

        let mut shift: u8 = 0;
        while shift < 26 {
            let score = self.scorer.score(input, shift);
            let r = DecipherResult { shift, score };
            results[shift as usize] = r;

            if r.score > best.score {
                best = r;
            }

            shift += 1;
        }

        Ok(DecaesarResult { best, results })
    }
}

pub fn decode_caesar(input: &[u8], output: &mut [u8], shift: u8) -> Result<(), DecaesarError> {
    if input.is_empty() {
        return Err(DecaesarError::EmptyInput);
    }

    if output.len() < input.len() {
        return Err(DecaesarError::OutputTooSmall {
            provided: output.len(),
            required: input.len(),
        });
    }

    if shift > 25 {
        return Err(DecaesarError::InvalidShift(shift));
    }

    for (i, b) in input.iter().enumerate() {
        output[i] = shift_byte(*b, shift);
    }
    Ok(())
}

fn shift_byte(b: u8, shift: u8) -> u8 {
    if b.is_ascii_lowercase() {
        ((b - b'a' + shift) % 26) + b'a'
    } else if b.is_ascii_uppercase() {
        ((b - b'A' + shift) % 26) + b'A'
    } else {
        b
    }
}
