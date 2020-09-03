#![doc = "Peripheral access API for SKEAZN642 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn INT_FTMRH();
    fn INT_LVD_LVW();
    fn INT_IRQ();
    fn INT_I2C0();
    fn INT_SPI0();
    fn INT_SPI1();
    fn INT_UART0();
    fn INT_UART1();
    fn INT_UART2();
    fn INT_ADC0();
    fn INT_ACMP0();
    fn INT_FTM0();
    fn INT_FTM1();
    fn INT_FTM2();
    fn INT_RTC();
    fn INT_ACMP1();
    fn INT_PIT_CH0();
    fn INT_PIT_CH1();
    fn INT_KBI0();
    fn INT_KBI1();
    fn INT_ICS();
    fn INT_WATCHDOG();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 29] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: INT_FTMRH,
    },
    Vector {
        _handler: INT_LVD_LVW,
    },
    Vector { _handler: INT_IRQ },
    Vector { _handler: INT_I2C0 },
    Vector { _reserved: 0 },
    Vector { _handler: INT_SPI0 },
    Vector { _handler: INT_SPI1 },
    Vector {
        _handler: INT_UART0,
    },
    Vector {
        _handler: INT_UART1,
    },
    Vector {
        _handler: INT_UART2,
    },
    Vector { _handler: INT_ADC0 },
    Vector {
        _handler: INT_ACMP0,
    },
    Vector { _handler: INT_FTM0 },
    Vector { _handler: INT_FTM1 },
    Vector { _handler: INT_FTM2 },
    Vector { _handler: INT_RTC },
    Vector {
        _handler: INT_ACMP1,
    },
    Vector {
        _handler: INT_PIT_CH0,
    },
    Vector {
        _handler: INT_PIT_CH1,
    },
    Vector { _handler: INT_KBI0 },
    Vector { _handler: INT_KBI1 },
    Vector { _reserved: 0 },
    Vector { _handler: INT_ICS },
    Vector {
        _handler: INT_WATCHDOG,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "5 - INT_FTMRH"]
    INT_FTMRH = 5,
    #[doc = "6 - INT_LVD_LVW"]
    INT_LVD_LVW = 6,
    #[doc = "7 - INT_IRQ"]
    INT_IRQ = 7,
    #[doc = "8 - INT_I2C0"]
    INT_I2C0 = 8,
    #[doc = "10 - INT_SPI0"]
    INT_SPI0 = 10,
    #[doc = "11 - INT_SPI1"]
    INT_SPI1 = 11,
    #[doc = "12 - INT_UART0"]
    INT_UART0 = 12,
    #[doc = "13 - INT_UART1"]
    INT_UART1 = 13,
    #[doc = "14 - INT_UART2"]
    INT_UART2 = 14,
    #[doc = "15 - INT_ADC0"]
    INT_ADC0 = 15,
    #[doc = "16 - INT_ACMP0"]
    INT_ACMP0 = 16,
    #[doc = "17 - INT_FTM0"]
    INT_FTM0 = 17,
    #[doc = "18 - INT_FTM1"]
    INT_FTM1 = 18,
    #[doc = "19 - INT_FTM2"]
    INT_FTM2 = 19,
    #[doc = "20 - INT_RTC"]
    INT_RTC = 20,
    #[doc = "21 - INT_ACMP1"]
    INT_ACMP1 = 21,
    #[doc = "22 - INT_PIT_CH0"]
    INT_PIT_CH0 = 22,
    #[doc = "23 - INT_PIT_CH1"]
    INT_PIT_CH1 = 23,
    #[doc = "24 - INT_KBI0"]
    INT_KBI0 = 24,
    #[doc = "25 - INT_KBI1"]
    INT_KBI1 = 25,
    #[doc = "27 - INT_ICS"]
    INT_ICS = 27,
    #[doc = "28 - INT_Watchdog"]
    INT_WATCHDOG = 28,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Flash configuration field"]
pub struct FTMRH_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTMRH_FLASHCONFIG {}
impl FTMRH_FLASHCONFIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftmrh_flash_config::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for FTMRH_FLASHCONFIG {
    type Target = ftmrh_flash_config::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTMRH_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftmrh_flash_config;
#[doc = "Flash Memory"]
pub struct FTMRH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTMRH {}
impl FTMRH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftmrh::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for FTMRH {
    type Target = ftmrh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTMRH::ptr() }
    }
}
#[doc = "Flash Memory"]
pub mod ftmrh;
#[doc = "Interrupt"]
pub struct IRQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IRQ {}
impl IRQ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const irq::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for IRQ {
    type Target = irq::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IRQ::ptr() }
    }
}
#[doc = "Interrupt"]
pub mod irq;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pit::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit;
#[doc = "FlexTimer Module"]
pub struct FTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM0 {}
impl FTM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for FTM0 {
    type Target = ftm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM0::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm0;
#[doc = "FlexTimer Module"]
pub struct FTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM1 {}
impl FTM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm1::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for FTM1 {
    type Target = ftm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM1::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm1;
#[doc = "FlexTimer Module"]
pub struct FTM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM2 {}
impl FTM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm2::RegisterBlock {
        0x4003_a000 as *const _
    }
}
impl Deref for FTM2 {
    type Target = ftm2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM2::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm2;
#[doc = "Analog-to-digital converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4003_b000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc;
#[doc = "Real-time counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4003_d000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time counter"]
pub mod rtc;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sim::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Port control and interrupts"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Port control and interrupts"]
pub mod port;
#[doc = "Watchdog timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        0x4005_2000 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Watchdog timer"]
pub mod wdog;
#[doc = "Clock management"]
pub struct ICS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICS {}
impl ICS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ics::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for ICS {
    type Target = ics::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICS::ptr() }
    }
}
#[doc = "Clock management"]
pub mod ics;
#[doc = "Oscillator"]
pub struct OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC {}
impl OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc::RegisterBlock {
        0x4006_5000 as *const _
    }
}
impl Deref for OSC {
    type Target = osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4006_b000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub mod uart1;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART)"]
pub mod uart2;
#[doc = "Analog comparator"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp0::RegisterBlock {
        0x4007_3000 as *const _
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACMP0::ptr() }
    }
}
#[doc = "Analog comparator"]
pub mod acmp0;
#[doc = "Analog comparator"]
pub struct ACMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP1 {}
impl ACMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp1::RegisterBlock {
        0x4007_4000 as *const _
    }
}
impl Deref for ACMP1 {
    type Target = acmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACMP1::ptr() }
    }
}
#[doc = "Analog comparator"]
pub mod acmp1;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4007_6000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4007_7000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Keyboard interrupts"]
pub struct KBI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KBI0 {}
impl KBI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const kbi0::RegisterBlock {
        0x4007_9000 as *const _
    }
}
impl Deref for KBI0 {
    type Target = kbi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*KBI0::ptr() }
    }
}
#[doc = "Keyboard interrupts"]
pub mod kbi0;
#[doc = "Keyboard interrupts"]
pub struct KBI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KBI1 {}
impl KBI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const kbi1::RegisterBlock {
        0x4007_a000 as *const _
    }
}
impl Deref for KBI1 {
    type Target = kbi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*KBI1::ptr() }
    }
}
#[doc = "Keyboard interrupts"]
pub mod kbi1;
#[doc = "Power management"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x4007_d000 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power management"]
pub mod pmc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x400f_f040 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rom::RegisterBlock {
        0xf000_2000 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcm::RegisterBlock {
        0xf000_3000 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioa::RegisterBlock {
        0xf800_0000 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiob::RegisterBlock {
        0xf800_0040 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTMRH_FLASHCONFIG"]
    pub FTMRH_FLASHCONFIG: FTMRH_FLASHCONFIG,
    #[doc = "FTMRH"]
    pub FTMRH: FTMRH,
    #[doc = "IRQ"]
    pub IRQ: IRQ,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "FTM0"]
    pub FTM0: FTM0,
    #[doc = "FTM1"]
    pub FTM1: FTM1,
    #[doc = "FTM2"]
    pub FTM2: FTM2,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "ICS"]
    pub ICS: ICS,
    #[doc = "OSC"]
    pub OSC: OSC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "ACMP1"]
    pub ACMP1: ACMP1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "KBI0"]
    pub KBI0: KBI0,
    #[doc = "KBI1"]
    pub KBI1: KBI1,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTMRH_FLASHCONFIG: FTMRH_FLASHCONFIG {
                _marker: PhantomData,
            },
            FTMRH: FTMRH {
                _marker: PhantomData,
            },
            IRQ: IRQ {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            PIT: PIT {
                _marker: PhantomData,
            },
            FTM0: FTM0 {
                _marker: PhantomData,
            },
            FTM1: FTM1 {
                _marker: PhantomData,
            },
            FTM2: FTM2 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            ICS: ICS {
                _marker: PhantomData,
            },
            OSC: OSC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            ACMP1: ACMP1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            KBI0: KBI0 {
                _marker: PhantomData,
            },
            KBI1: KBI1 {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            ROM: ROM {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            FGPIOA: FGPIOA {
                _marker: PhantomData,
            },
            FGPIOB: FGPIOB {
                _marker: PhantomData,
            },
        }
    }
}
