#![no_std]
#![no_main]

use flash_algorithm::*;
#[cfg(feature = "rtt")]
use rtt_target::{rprintln, rtt_init_print};
#[cfg(not(feature = "rtt"))]
macro_rules! rtt_init_print {
    () => {};
}
#[cfg(not(feature = "rtt"))]
macro_rules! rprintln {
    ($($arg:tt)*) => { let _ = ($($arg)*); };
}

#[allow(unused)]
mod reg;
use reg::*;
// We use custom version of SysTick delay to reduse size.
mod delay;

// Always use HOCO 'cause MOCO is not accurate enough.
const USE_HOCO: bool = true;

struct Algorithm {
    delay: delay::Delay,
}

impl Algorithm {
    const ERASE_CHIP_FAIL: ErrorCode = unsafe { ErrorCode::new_unchecked(10001) };
    const ERASE_SECTOR_FAIL: ErrorCode = unsafe { ErrorCode::new_unchecked(10002) };
    const PROGRAM_PAGE_FAIL: ErrorCode = unsafe { ErrorCode::new_unchecked(10003) };
    const ERROR_INVALID_PARAM: ErrorCode = unsafe { ErrorCode::new_unchecked(10004) };

    #[inline(never)]
    fn prog_mode(&mut self) {
        // rprintln!("FENTRYR = 0x{:X}", FENTRYR.read());
        FENTRYR.write(0xAA01);
        // rprintln!("FENTRYR = 0x{:X}", FENTRYR.read());

        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        FPR.write(0xA5);
        FPMCR.write(0x12);
        FPMCR.write(!0x12);
        FPMCR.write(0x12);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        // wait tDIS = 2 us
        self.delay.delay_us(2);
        FPR.write(0xA5);
        FPMCR.write(0x92);
        FPMCR.write(!0x92);
        FPMCR.write(0x92);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        FPR.write(0xA5);
        FPMCR.write(0x82);
        FPMCR.write(!0x82);
        FPMCR.write(0x82);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        // wait tMS = 5 us for High-speed and 720 ns for Middle-speed
        self.delay.delay_us(if USE_HOCO { 5 } else { 1 });

        // rprintln!("FISR = 0x{:X}", FISR.read());
        if USE_HOCO {
            // Set PCKA to 23 when FCLK is at 24MHz
            FISR.write(24 - 1);
        } else {
            // Set PCKA to 7 when FCLK is at 8MHz
            FISR.write(8 - 1);
        }
        // rprintln!("FISR = 0x{:X}", FISR.read());

        FRESETR.write(1);
        FRESETR.write(0);
    }

    #[inline(never)]
    fn read_mode(&mut self) {
        // rprintln!("FENTRYR = 0x{:X}", FENTRYR.read());
        // FENTRYR.write(0xAA01);
        // rprintln!("FENTRYR = 0x{:X}", FENTRYR.read());

        FRESETR.write(1);
        FRESETR.write(0);

        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        FPR.write(0xA5);
        FPMCR.write(0x92);
        FPMCR.write(!0x92);
        FPMCR.write(0x92);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        // wait tDIS = 2 us
        self.delay.delay_us(2);
        FPR.write(0xA5);
        FPMCR.write(0x12);
        FPMCR.write(!0x12);
        FPMCR.write(0x12);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        FPR.write(0xA5);
        FPMCR.write(0x08);
        FPMCR.write(!0x08);
        FPMCR.write(0x08);
        // rprintln!("FPMCR = 0x{:X}", FPMCR.read());
        // wait tMS = 5 us for High-speed and 720 ns for Middle-speed
        self.delay.delay_us(if USE_HOCO { 5 } else { 1 });

        FENTRYR.write(0xAA00);
        while FENTRYR.read() != 0x0000 {}
    }
}

algorithm!(Algorithm, {
    flash_address: 0x0,
    flash_size: 0x40000,
    page_size: 0x800,
    empty_value: 0xFF,
    sectors: [{
        size: 0x800,
        address: 0x0,
    }]
});

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");

        rprintln!("OPCCR = 0x{:X}", OPCCR.read());
        rprintln!("SCKSCR = 0x{:X}", SCKSCR.read());
        rprintln!("SCKDIVCR = 0x{:X}", SCKDIVCR.read());

        // let mut core_peripherals = cortex_m::Peripherals::take().unwrap();
        let core_peripherals = unsafe { cortex_m::Peripherals::steal() };
        // We will set ICLK to 48MHz or 8MHz below.
        let ick_herz = if USE_HOCO { 48_000_000 } else { 8_000_000 };
        let delay = delay::Delay::new(core_peripherals.SYST, ick_herz);

        // rprintln!("PRCR = 0x{:X}", PRCR.read());
        // PRC0 = 1, PRC1 = 1
        PRCR.write(0xA503);
        // rprintln!("PRCR = 0x{:X}", PRCR.read());

        // rprintln!("OPCCR = 0x{:X}", OPCCR.read());
        if USE_HOCO {
            // Set to High-speed mode
            OPCCR.write(0x00);
        } else {
            // Set to Middle-speed mode
            OPCCR.write(0x01);
        }
        while OPCCR.read() & 0x10 != 0 {}
        // rprintln!("OPCCR = 0x{:X}", OPCCR.read());

        // rprintln!("SCKDIVCR = 0x{:X}", SCKDIVCR.read());
        if USE_HOCO {
            SCKSCR.write(0x01); // select MOCO
            // dividing by 1, we get 8MHz
            SCKDIVCR.write(0x0000_0000);
            HOCOCR.write(0x01); // Stop HOCO
            HOCOCR2.write(0x20); // Set HOCO to 48MHz
            HOCOCR.write(0x00); // Start HOCO
            while OSCSF.read() & 0x01 == 0 {}
            // dividing by 2, we get 24MHz. except for ICLK.
            SCKDIVCR.write(0x1001_1111);
            // May not need this as we will only be accessing SRAM and registers.
            MEMWAIT.write(0x01); // MEMWAIT = 1
            SCKSCR.write(0x00); // select HOCO
        } else {
            SCKSCR.write(0x01); // select MOCO
            // dividing by 1, we get 8MHz
            SCKDIVCR.write(0x0000_0000);
        }
        // rprintln!("SCKDIVCR = 0x{:X}", SCKDIVCR.read());

        Ok(Self { delay })
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");

        self.prog_mode();

        // If it is a target device with multiple flash macrocells,
        // command must be issued to each macrocell separately.

        let addr = 0x0000_0000;
        FSARL.write(addr as u16);
        FSARH.write((addr >> 16) as u16);

        let end_addr = 0x0004_0000;
        FEARL.write(end_addr as u16);
        FEARH.write((end_addr >> 16) as u16);

        FCR.write(0x86); // Chip Erase Command
        while FSTATR1.read() & 0x40 == 0 {} // Wait until FRDY = 1
        FCR.write(0x00);
        while FSTATR1.read() & 0x40 != 0 {} // Wait while FRDY = 1

        let fstatr2 = FSTATR2.read();

        self.read_mode();

        if fstatr2 == 0 {
            Ok(())
        } else {
            rprintln!("ADDR = 0x{:X} FSTATR2 = 0x{:X}", addr, fstatr2);
            Err(Self::ERASE_CHIP_FAIL)
        }
    }

    fn erase_sector(&mut self, addr: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector addr:{}", addr);

        self.prog_mode();

        FSARL.write(addr as u16);
        FSARH.write((addr >> 16) as u16);

        let end_addr = addr;
        FEARL.write(end_addr as u16);
        FEARH.write((end_addr >> 16) as u16);

        FCR.write(0x84); // Block Erase Command
        while FSTATR1.read() & 0x40 == 0 {} // Wait until FRDY = 1
        FCR.write(0x00);
        while FSTATR1.read() & 0x40 != 0 {} // Wait while FRDY = 1

        let fstatr2 = FSTATR2.read();

        self.read_mode();

        if fstatr2 == 0 {
            Ok(())
        } else {
            rprintln!("ADDR = 0x{:X} FSTATR2 = 0x{:X}", addr, fstatr2);
            Err(Self::ERASE_SECTOR_FAIL)
        }
    }

    fn program_page(&mut self, addr: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page addr:{} size:{}", addr, data.len());

        if data.len() % 8 != 0 {
            return Err(Self::ERROR_INVALID_PARAM);
        }

        self.prog_mode();

        let end_addr = addr + data.len() as u32;

        let mut addr = addr;
        let mut data_ptr = data.as_ptr() as *const u32;
        while addr != end_addr {
            FSARL.write(addr as u16);
            FSARH.write((addr >> 16) as u16);

            let data0 = unsafe { *data_ptr };
            data_ptr = data_ptr.wrapping_add(1);
            let data1 = unsafe { *data_ptr };
            data_ptr = data_ptr.wrapping_add(1);
            // rprintln!("data0 = 0x{:X}", data0);
            // rprintln!("data1 = 0x{:X}", data1);

            if data0 != 0xFFFF_FFFF || data1 != 0xFFFF_FFFF {
                FWBL0.write(data0 as u16);
                FWBH0.write((data0 >> 16) as u16);
                FWBL1.write(data1 as u16);
                FWBH1.write((data1 >> 16) as u16);

                FCR.write(0x81); // Program Command
                while FSTATR1.read() & 0x40 == 0 {} // Wait until FRDY = 1
                FCR.write(0x00);
                while FSTATR1.read() & 0x40 != 0 {} // Wait while FRDY = 1

                if FSTATR2.read() != 0 {
                    break;
                }
            }

            addr += 8;
        }

        let fstatr2 = FSTATR2.read();

        self.read_mode();

        if fstatr2 == 0 {
            Ok(())
        } else {
            rprintln!("ADDR = 0x{:X} FSTATR2 = 0x{:X}", addr, fstatr2);
            Err(Self::PROGRAM_PAGE_FAIL)
        }
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // Add code here to uninitialize the flash algorithm.
        SCKSCR.write(0x01); // select MOCO
        SCKDIVCR.write(0x0000_0000);
        MEMWAIT.write(0x00); // MEMWAIT = 0
        let ofs1 = OFS1.read();
        HOCOCR.write(0x01); // Stop HOCO
        HOCOCR2.write(((ofs1 >> 8) & 0x38) as u8); // Set HOCO frequency
        #[cfg(feature = "enable_hoco_after")]
        {
            if ofs1 & 0x01 == 0 {
                HOCOCR.write(0x00); // Start HOCO
                while OSCSF.read() & 0x01 == 0 {}
            } else {
                // Set to Middle-speed mode
                OPCCR.write(0x01);
            }
        }
        #[cfg(not(feature = "enable_hoco_after"))]
        {
            // Set to Middle-speed mode
            OPCCR.write(0x01);
        }
        // PRC0 = 0, PRC1 = 0
        PRCR.write(0xA500);
    }
}
