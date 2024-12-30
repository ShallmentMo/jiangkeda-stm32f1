#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = if 1 == 1 {
        rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr)
    } else {
        rcc.cfgr
            .use_hse(8.MHz())
            .sysclk(48.MHz())
            .pclk1(6.MHz())
            .freeze(&mut flash.acr)
    };

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb10;
    let sda = gpiob.pb11;

    let mut i2c = dp
        .I2C2
        //.remap(&mut afio.mapr) // add this if want to use PB8, PB9 instead
        .blocking_i2c(
            (scl, sda),
            Mode::Fast {
                frequency: 400.kHz(),
                duty_cycle: DutyCycle::Ratio16to9,
            },
            &clocks,
            1000,
            10,
            1000,
            1000,
        );
    rprintln!("i2c init");

    i2c.write(0x68, &[0x6B, 0x01]).unwrap();
    i2c.write(0x68, &[0x6C, 0x00]).unwrap();
    i2c.write(0x68, &[0x19, 0x09]).unwrap();
    i2c.write(0x68, &[0x1A, 0x06]).unwrap();
    i2c.write(0x68, &[0x1B, 0x18]).unwrap();
    i2c.write(0x68, &[0x1C, 0x18]).unwrap();

    let mut buffer = [0u8; 8];
    i2c.write_read(0x68, &[0x75], &mut buffer).unwrap();
    rprintln!("id: {:?}", buffer);

    loop {
        let mut buffer = [0u8; 14];
        // Read accelerometer and gyro data starting from ACCEL_XOUT_H (0x3B)
        i2c.write_read(0x68, &[0x3B], &mut buffer).unwrap();

        // Combine high and low bytes into 16-bit values
        let acc_x = (buffer[0] as i16) << 8 | buffer[1] as i16;
        let acc_y = (buffer[2] as i16) << 8 | buffer[3] as i16;
        let acc_z = (buffer[4] as i16) << 8 | buffer[5] as i16;

        let gyro_x = (buffer[8] as i16) << 8 | buffer[9] as i16;
        let gyro_y = (buffer[10] as i16) << 8 | buffer[11] as i16;
        let gyro_z = (buffer[12] as i16) << 8 | buffer[13] as i16;

        rprintln!("Acc X:{}, Y:{}, Z:{}", acc_x, acc_y, acc_z);
        rprintln!("Gyro X:{}, Y:{}, Z:{}", gyro_x, gyro_y, gyro_z);

        // Add a small delay
        cortex_m::asm::delay(8_000_000);
    }
}
