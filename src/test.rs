mod huffman;

#[cfg(test)]
mod tests {
    use super::huffman;
    #[test]
    fn test_decode_huffman_tree() {
        let txt = "Huffman coding is a data compression algorithm.";
        let (encoded, root) = huffman::build_huffman_tree(txt);
        assert_eq!(txt, huffman::decode_huffman_tree(&encoded, &root));
    }
}
