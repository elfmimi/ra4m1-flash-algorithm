/*
 * This file is borrowed from cotex-m crate.
 * https://github.com/rust-embedded/cortex-m/blob/v0.7.7/src/delay.rs
 */
//! A delay driver based on SysTick.

use cortex_m::peripheral::{syst::SystClkSource, SYST};

/// System timer (SysTick) as a delay provider.
pub struct Delay {
    syst: SYST,
    frequency: u32,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider.
    ///
    /// `ahb_frequency` is a frequency of the AHB bus in Hz.
    #[inline]
    pub fn new(syst: SYST, ahb_frequency: u32) -> Self {
        Self::with_source(syst, ahb_frequency, SystClkSource::Core)
    }

    /// Configures the system timer (SysTick) as a delay provider
    /// with a clock source.
    ///
    /// `frequency` is the frequency of your `clock_source` in Hz.
    #[inline]
    pub fn with_source(mut syst: SYST, frequency: u32, clock_source: SystClkSource) -> Self {
        syst.set_clock_source(clock_source);

        Delay { syst, frequency }
    }

    /// Releases the system timer (SysTick) resource.
    #[allow(unused)]
    #[inline]
    pub fn free(self) -> SYST {
        self.syst
    }

    /// Delay using the Cortex-M systick for a certain duration, in us.
    #[allow(clippy::missing_inline_in_public_items)]
    pub fn delay_us(&mut self, us: u32) {
        // let ticks = (u64::from(us)) * (u64::from(self.frequency)) / 1_000_000;
        // let ticks = us * (self.frequency / 1_000_000);
        let ticks = us * ((self.frequency + 999) / 1_000) / 1_000;

        let full_cycles = ticks >> 24;
        if full_cycles > 0 {
            self.syst.set_reload(0xffffff);
            self.syst.clear_current();
            self.syst.enable_counter();

            for _ in 0..full_cycles {
                while !self.syst.has_wrapped() {}
            }
        }

        let ticks = (ticks & 0xffffff) as u32;
        if ticks > 1 {
            self.syst.set_reload(ticks - 1);
            self.syst.clear_current();
            self.syst.enable_counter();

            while !self.syst.has_wrapped() {}
        }

        self.syst.disable_counter();
    }

    /// Delay using the Cortex-M systick for a certain duration, in ms.
    #[allow(unused)]
    #[inline]
    pub fn delay_ms(&mut self, mut ms: u32) {
        // 67_000 is the maximum acceptable by our delay_us() when ICK = 64MHz
        while ms > 67 {
            self.delay_us(67_000);
            ms -= 67;
        }
        self.delay_us(ms * 1_000);
    }
}
