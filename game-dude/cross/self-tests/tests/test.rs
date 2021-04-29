#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt;

    #[init]
    fn init() {
    }

    #[test]
    fn will_pass() {
        assert!(true);
    }
}