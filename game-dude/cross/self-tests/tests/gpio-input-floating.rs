#![no_std]
#![no_main]

use defmt_rtt as _;    // transport layer for defmt logs
use stm32l4xx_hal as hal; // the HAL we'll test
use panic_probe as _;  // panicking behavior

use hal::{
    gpio::{
        gpioc,
        Input,
        Floating
    },
};

struct TestInput {
    // using PC8 and PC9 for input
    input_ground: gpioc::PC9<Input<Floating>>,
    input_vdd: gpioc::PC8<Input<Floating>>,
}

#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use crate::TestInput;
    use stm32l4xx_hal::{
        prelude::*,
        stm32,
        gpio::{
            gpioc,
        },
    };

    #[init]
    fn init() -> TestInput {
        let peripherals = stm32::Peripherals::take().unwrap();
        let mut rcc = peripherals.RCC.constrain();
        let mut gpioc = peripherals.GPIOC.split(&mut rcc.ahb2);
        let input_vdd = gpioc
            .pc8
            .into_floating_input(&mut gpioc.moder, &mut gpioc.pupdr);
        let input_ground = gpioc
            .pc9
            .into_floating_input(&mut gpioc.moder, &mut gpioc.pupdr);

        TestInput {input_ground, input_vdd }
    }

    #[test]
    fn always_passes() {
        assert!(true);
    }

    #[test]
    fn ground_is_low(state: &mut TestInput) {
        assert!(state.input_ground.is_low().unwrap());
    }

    #[test]
    fn vdd_is_high(state: &mut TestInput) {
        assert!(state.input_vdd.is_high().unwrap());
    }
}