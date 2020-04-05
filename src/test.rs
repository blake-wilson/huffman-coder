mod huffman;

#[cfg(test)]
mod tests {
    use super::huffman;
    use std::collections::HashMap;
    use std::str::from_utf8;
    #[test]
    fn test_decode_huffman_tree() {
        let txt = "Huffman coding is a data compression algorithm.";
        let hm = &mut HashMap::new();
        let (encoded, root) = huffman::build_huffman_tree(txt.as_bytes(), hm);
        let buffer = &mut Vec::new();
        huffman::decode_huffman_tree(&encoded, &root, buffer);
        assert_eq!(txt, from_utf8(buffer).unwrap());
    }
}
