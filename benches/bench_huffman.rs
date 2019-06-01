#[macro_use]
extern crate bencher;
use bencher::Bencher;
use huffman::build_huffman_tree;

fn bench_build_huffman_tree(b: &mut Bencher) {
    let test_text = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum");
    b.iter(|| build_huffman_tree(&test_text));
}

benchmark_group!(benches, bench_build_huffman_tree);
benchmark_main!(benches);
