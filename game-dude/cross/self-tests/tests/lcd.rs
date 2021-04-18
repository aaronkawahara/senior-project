#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use board::Board;
    use defmt;//::{assert_eq, unwrap};

    #[init]
    fn init() -> Board {
        Board::init()
    }

    #[test]
    fn confirm_rcc_config(board: &mut Board) {
        assert!(true);
    }
}