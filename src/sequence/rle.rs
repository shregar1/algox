use crate::abstraction::AlgorithmTrait;
use super::abstraction::SequenceAlgorithmTrait;

/// Run-Length Encoding (RLE) for lossless sequence compression.
pub struct RunLengthEncoding;

impl RunLengthEncoding {
    /// Encodes a sequence into run-length pairs (item, count).
    pub fn encode<T: PartialEq + Clone>(sequence: &[T]) -> Vec<(T, usize)> {
        if sequence.is_empty() {
            return Vec::new();
        }

        let mut encoded = Vec::new();
        let mut current_item = sequence[0].clone();
        let mut count = 1;

        for item in &sequence[1..] {
            if item == &current_item {
                count += 1;
            } else {
                encoded.push((current_item, count));
                current_item = item.clone();
                count = 1;
            }
        }
        encoded.push((current_item, count));
        encoded
    }

    /// Decodes run-length pairs back into the original sequence.
    pub fn decode<T: Clone>(encoded: &[(T, usize)]) -> Vec<T> {
        let mut decoded = Vec::new();
        for (item, count) in encoded {
            for _ in 0..*count {
                decoded.push(item.clone());
            }
        }
        decoded
    }
}

impl AlgorithmTrait for RunLengthEncoding {
    fn name(&self) -> &'static str {
        "run_length_encoding"
    }

    fn len(&self) -> usize {
        0
    }

    fn clear(&mut self) {}
}

impl SequenceAlgorithmTrait for RunLengthEncoding {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_encode_decode() {
        let input = vec!['A', 'A', 'A', 'B', 'B', 'C', 'A', 'A'];
        let encoded = RunLengthEncoding::encode(&input);
        assert_eq!(
            encoded,
            vec![('A', 3), ('B', 2), ('C', 1), ('A', 2)]
        );
        let decoded = RunLengthEncoding::decode(&encoded);
        assert_eq!(decoded, input);
    }
}
