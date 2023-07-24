pub struct Reg<const T: usize>(u32);

#[rustfmt::skip]
impl Reg<8> {
    #[inline(always)]
    pub fn read(&self) -> u8 { unsafe { core::ptr::read_volatile(self.0 as *const _) } }
    #[inline(always)]
    pub fn write(&self, data: u8) { unsafe { core::ptr::write_volatile(self.0 as *mut _, data); } }
}
#[rustfmt::skip]
impl Reg<16> {
    #[inline(always)]
    pub fn read(&self) -> u16 { unsafe { core::ptr::read_volatile(self.0 as *const _) } }
    #[inline(always)]
    pub fn write(&self, data: u16) { unsafe { core::ptr::write_volatile(self.0 as *mut _, data); } }
}
#[rustfmt::skip]
impl Reg<32> {
    #[inline(always)]
    pub fn read(&self) -> u32 { unsafe { core::ptr::read_volatile(self.0 as *const _) } }
    #[inline(always)]
    pub fn write(&self, data: u32) { unsafe { core::ptr::write_volatile(self.0 as *mut _, data); } }
}

pub const OFS1: Reg<16> = Reg(0x0000_0404);

pub const FCACHEIV: Reg<16> = Reg(0x4001_C104);
pub const PRCR: Reg<16> = Reg(0x4001_E3FE);
pub const SCKDIVCR: Reg<32> = Reg(0x4001_E020);
pub const SCKSCR: Reg<8> = Reg(0x4001_E026);
pub const MEMWAIT: Reg<8> = Reg(0x4001_E031);
pub const HOCOCR: Reg<8> = Reg(0x4001_E036);
pub const HOCOCR2: Reg<8> = Reg(0x4001_E037);
pub const OSCSF: Reg<8> = Reg(0x4001_E03C);
pub const OPCCR: Reg<8> = Reg(0x4001_E0A0);
pub const DFLCTL: Reg<8> = Reg(0x407E_C090);
pub const FENTRYR: Reg<16> = Reg(0x407E_FFB2);
pub const FPR: Reg<8> = Reg(0x407E_C180);
pub const FPMCR: Reg<8> = Reg(0x407E_C100);
pub const FISR: Reg<8> = Reg(0x407E_C1D8);
pub const FRESETR: Reg<8> = Reg(0x407E_C124);
pub const FASR: Reg<8> = Reg(0x407E_C104);
pub const FCR: Reg<8> = Reg(0x407E_C114);
// pub const FEXCR: Reg<8> = Reg(0x407E_C1DC);
pub const FSARL: Reg<16> = Reg(0x407E_C108);
pub const FSARH: Reg<16> = Reg(0x407E_C110);
pub const FEARL: Reg<16> = Reg(0x407E_C118);
pub const FEARH: Reg<16> = Reg(0x407E_C120);
pub const FWBL0: Reg<16> = Reg(0x407E_C130);
pub const FWBH0: Reg<16> = Reg(0x407E_C138);
pub const FWBL1: Reg<16> = Reg(0x407E_C140);
pub const FWBH1: Reg<16> = Reg(0x407E_C144);
pub const FRBL0: Reg<16> = Reg(0x407E_C188);
pub const FRBH0: Reg<16> = Reg(0x407E_C190);
pub const FRBL1: Reg<16> = Reg(0x407E_C148);
pub const FRBH1: Reg<16> = Reg(0x407E_C14C);
pub const FSTATR00: Reg<16> = Reg(0x407E_C128);
pub const FSTATR01: Reg<16> = Reg(0x407E_C13C);
pub const FSTATR2: Reg<16> = Reg(0x407E_C1F0);
pub const FSTATR1: Reg<8> = Reg(0x407E_C12C);
pub const FEAMH: Reg<16> = Reg(0x407E_C1E8);
pub const FEAML: Reg<16> = Reg(0x407E_C1E0);
pub const FSCMR: Reg<16> = Reg(0x407E_C1C0);
pub const FAWSMR: Reg<16> = Reg(0x407E_C1C8);
pub const FAWEMR: Reg<16> = Reg(0x407E_C1D0);
pub const FLWAITR: Reg<8> = Reg(0x407E_FFC0);
