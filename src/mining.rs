use crate::block::Block;

/// constant named DIFFICULTY if difficulty of mining or count of zeros in the hash
/// difficulty 5 ~ 10sec per block
pub const DIFFICULTY: usize = 5;

fn target_prefix() -> String {
    "0".repeat(DIFFICULTY)
}

pub fn meets_difficulty(hash: &str) -> bool {
    hash.starts_with(&target_prefix())
}

pub fn mine_block(mut block: Block) -> Block {
    let target = target_prefix();

    println!("Mining block: {} (Difficulty: {} zeros)...", block.index, DIFFICULTY);

    ///here is the performance issue because of calculating hash on every iteration of the loop: to optimise.

    loop {
        block.hash = block.calculate_hash();

        if block.hash.starts_with(&target) {
            println!(
                "Block found! Nonce: {}, Hash: {}",
                block.nonce,
                &block.hash[..16]
            );
            break;
        }

        block.nonce = block.nonce.wrapping_add(1);
    }

    block
}