#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430F5529 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
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
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn RTC();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn USCI_B1();
    fn USCI_A1();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn USB_UBM();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_B();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 63] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC },
    Vector { _handler: PORT2 },
    Vector {
        _handler: TIMER2_A1,
    },
    Vector {
        _handler: TIMER2_A0,
    },
    Vector { _handler: USCI_B1 },
    Vector { _handler: USCI_A1 },
    Vector { _handler: PORT1 },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector { _handler: DMA },
    Vector { _handler: USB_UBM },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: ADC12 },
    Vector { _handler: USCI_B0 },
    Vector { _handler: USCI_A0 },
    Vector { _handler: WDT },
    Vector {
        _handler: TIMER0_B1,
    },
    Vector {
        _handler: TIMER0_B0,
    },
    Vector { _handler: COMP_B },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "41 - 0xFFD2 RTC"]
    RTC = 41,
    #[doc = "42 - 0xFFD4 Port 2"]
    PORT2 = 42,
    #[doc = "43 - 0xFFD6 Timer2_A5 CC1-4, TA"]
    TIMER2_A1 = 43,
    #[doc = "44 - 0xFFD8 Timer2_A5 CC0"]
    TIMER2_A0 = 44,
    #[doc = "45 - 0xFFDA USCI B1 Receive/Transmit"]
    USCI_B1 = 45,
    #[doc = "46 - 0xFFDC USCI A1 Receive/Transmit"]
    USCI_A1 = 46,
    #[doc = "47 - 0xFFDE Port 1"]
    PORT1 = 47,
    #[doc = "48 - 0xFFE0 Timer1_A3 CC1-2, TA1"]
    TIMER1_A1 = 48,
    #[doc = "49 - 0xFFE2 Timer1_A3 CC0"]
    TIMER1_A0 = 49,
    #[doc = "50 - 0xFFE4 DMA"]
    DMA = 50,
    #[doc = "51 - 0xFFE6 USB Timer / cable event / USB reset"]
    USB_UBM = 51,
    #[doc = "52 - 0xFFE8 Timer0_A5 CC1-4, TA"]
    TIMER0_A1 = 52,
    #[doc = "53 - 0xFFEA Timer0_A5 CC0"]
    TIMER0_A0 = 53,
    #[doc = "54 - 0xFFEC ADC"]
    ADC12 = 54,
    #[doc = "55 - 0xFFEE USCI B0 Receive/Transmit"]
    USCI_B0 = 55,
    #[doc = "56 - 0xFFF0 USCI A0 Receive/Transmit"]
    USCI_A0 = 56,
    #[doc = "57 - 0xFFF2 Watchdog Timer"]
    WDT = 57,
    #[doc = "58 - 0xFFF4 Timer0_B7 CC1-6, TB"]
    TIMER0_B1 = 58,
    #[doc = "59 - 0xFFF6 Timer0_B7 CC0"]
    TIMER0_B0 = 59,
    #[doc = "60 - 0xFFF8 Comparator B"]
    COMP_B = 60,
    #[doc = "61 - 0xFFFA User Non-maskable"]
    UNMI = 61,
    #[doc = "62 - 0xFFFC System Non-maskable"]
    SYSNMI = 62,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Port Mapping Port 4"]
pub struct PORT_MAPPING_PORT_4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_MAPPING_PORT_4 {}
impl PORT_MAPPING_PORT_4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_mapping_port_4::RegisterBlock {
        0x01e0 as *const _
    }
}
impl Deref for PORT_MAPPING_PORT_4 {
    type Target = port_mapping_port_4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_MAPPING_PORT_4::ptr() }
    }
}
#[doc = "Port Mapping Port 4"]
pub mod port_mapping_port_4;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "Port 3/4"]
pub struct PORT_3_4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3_4 {}
impl PORT_3_4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_3_4::RegisterBlock {
        0x0220 as *const _
    }
}
impl Deref for PORT_3_4 {
    type Target = port_3_4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_3_4::ptr() }
    }
}
#[doc = "Port 3/4"]
pub mod port_3_4;
#[doc = "Port 5/6"]
pub struct PORT_5_6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_5_6 {}
impl PORT_5_6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_5_6::RegisterBlock {
        0x0240 as *const _
    }
}
impl Deref for PORT_5_6 {
    type Target = port_5_6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_5_6::ptr() }
    }
}
#[doc = "Port 5/6"]
pub mod port_5_6;
#[doc = "Port 7/8"]
pub struct PORT_7_8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_7_8 {}
impl PORT_7_8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_7_8::RegisterBlock {
        0x0260 as *const _
    }
}
impl Deref for PORT_7_8 {
    type Target = port_7_8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_7_8::ptr() }
    }
}
#[doc = "Port 7/8"]
pub mod port_7_8;
#[doc = "RTC Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x04a0 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC Real Time Clock"]
pub mod rtc;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        0x05e0 as *const _
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        0x05e0 as *const _
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "USCI_A1 UART Mode"]
pub struct USCI_A1_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_UART_MODE {}
impl USCI_A1_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_uart_mode::RegisterBlock {
        0x0600 as *const _
    }
}
impl Deref for USCI_A1_UART_MODE {
    type Target = usci_a1_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A1_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A1 UART Mode"]
pub mod usci_a1_uart_mode;
#[doc = "USCI_A1 SPI Mode"]
pub struct USCI_A1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_SPI_MODE {}
impl USCI_A1_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_spi_mode::RegisterBlock {
        0x0600 as *const _
    }
}
impl Deref for USCI_A1_SPI_MODE {
    type Target = usci_a1_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A1_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A1 SPI Mode"]
pub mod usci_a1_spi_mode;
#[doc = "USCI_B1 I2C Mode"]
pub struct USCI_B1_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B1_I2C_MODE {}
impl USCI_B1_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b1_i2c_mode::RegisterBlock {
        0x0620 as *const _
    }
}
impl Deref for USCI_B1_I2C_MODE {
    type Target = usci_b1_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B1_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B1 I2C Mode"]
pub mod usci_b1_i2c_mode;
#[doc = "USCI_B1 SPI Mode"]
pub struct USCI_B1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B1_SPI_MODE {}
impl USCI_B1_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b1_spi_mode::RegisterBlock {
        0x0620 as *const _
    }
}
impl Deref for USCI_B1_SPI_MODE {
    type Target = usci_b1_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B1_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B1 SPI Mode"]
pub mod usci_b1_spi_mode;
#[doc = "ADC12"]
pub struct ADC12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12 {}
impl ADC12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc12::RegisterBlock {
        0x0700 as *const _
    }
}
impl Deref for ADC12 {
    type Target = adc12::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12::ptr() }
    }
}
#[doc = "ADC12"]
pub mod adc12;
#[doc = "USB Control"]
pub struct USB_CONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_CONTROL {}
impl USB_CONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_control::RegisterBlock {
        0x0900 as *const _
    }
}
impl Deref for USB_CONTROL {
    type Target = usb_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB_CONTROL::ptr() }
    }
}
#[doc = "USB Control"]
pub mod usb_control;
#[doc = "USB Operation"]
pub struct USB_OPERATION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OPERATION {}
impl USB_OPERATION {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_operation::RegisterBlock {
        0x1c00 as *const _
    }
}
impl Deref for USB_OPERATION {
    type Target = usb_operation::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB_OPERATION::ptr() }
    }
}
#[doc = "USB Operation"]
pub mod usb_operation;
#[doc = "SFR Special Function Registers"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        0x0100 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR Special Function Registers"]
pub mod sfr;
#[doc = "PMM Power Management System"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM Power Management System"]
pub mod pmm;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x0140 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "CRC16"]
pub struct CRC16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC16 {}
impl CRC16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc16::RegisterBlock {
        0x0150 as *const _
    }
}
impl Deref for CRC16 {
    type Target = crc16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC16::ptr() }
    }
}
#[doc = "CRC16"]
pub mod crc16;
#[doc = "RC RAM Control Module"]
pub struct RC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RC {}
impl RC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rc::RegisterBlock {
        0x0158 as *const _
    }
}
impl Deref for RC {
    type Target = rc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RC::ptr() }
    }
}
#[doc = "RC RAM Control Module"]
pub mod rc;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        0x015c as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "UCS Unified System Clock"]
pub struct UCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UCS {}
impl UCS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ucs::RegisterBlock {
        0x0160 as *const _
    }
}
impl Deref for UCS {
    type Target = ucs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UCS::ptr() }
    }
}
#[doc = "UCS Unified System Clock"]
pub mod ucs;
#[doc = "SYS System Module"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        0x0180 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS System Module"]
pub mod sys;
#[doc = "Shared Reference"]
pub struct SHARED_REFERENCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHARED_REFERENCE {}
impl SHARED_REFERENCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const shared_reference::RegisterBlock {
        0x01b0 as *const _
    }
}
impl Deref for SHARED_REFERENCE {
    type Target = shared_reference::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHARED_REFERENCE::ptr() }
    }
}
#[doc = "Shared Reference"]
pub mod shared_reference;
#[doc = "Port Mapping Control"]
pub struct PORT_MAPPING_CONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_MAPPING_CONTROL {}
impl PORT_MAPPING_CONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_mapping_control::RegisterBlock {
        0x01c0 as *const _
    }
}
impl Deref for PORT_MAPPING_CONTROL {
    type Target = port_mapping_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_MAPPING_CONTROL::ptr() }
    }
}
#[doc = "Port Mapping Control"]
pub mod port_mapping_control;
#[doc = "Port J"]
pub struct PORT_J {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_J {}
impl PORT_J {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_j::RegisterBlock {
        0x0320 as *const _
    }
}
impl Deref for PORT_J {
    type Target = port_j::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_J::ptr() }
    }
}
#[doc = "Port J"]
pub mod port_j;
#[doc = "Timer0_A5"]
pub struct TIMER_0_A5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_A5 {}
impl TIMER_0_A5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_a5::RegisterBlock {
        0x0340 as *const _
    }
}
impl Deref for TIMER_0_A5 {
    type Target = timer_0_a5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_0_A5::ptr() }
    }
}
#[doc = "Timer0_A5"]
pub mod timer_0_a5;
#[doc = "Timer1_A3"]
pub struct TIMER_1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_1_A3 {}
impl TIMER_1_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_1_a3::RegisterBlock {
        0x0380 as *const _
    }
}
impl Deref for TIMER_1_A3 {
    type Target = timer_1_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_1_A3::ptr() }
    }
}
#[doc = "Timer1_A3"]
pub mod timer_1_a3;
#[doc = "Timer0_B7"]
pub struct TIMER_0_B7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_B7 {}
impl TIMER_0_B7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_b7::RegisterBlock {
        0x03c0 as *const _
    }
}
impl Deref for TIMER_0_B7 {
    type Target = timer_0_b7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_0_B7::ptr() }
    }
}
#[doc = "Timer0_B7"]
pub mod timer_0_b7;
#[doc = "Timer2_A3"]
pub struct TIMER_2_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_2_A3 {}
impl TIMER_2_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_2_a3::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for TIMER_2_A3 {
    type Target = timer_2_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_2_A3::ptr() }
    }
}
#[doc = "Timer2_A3"]
pub mod timer_2_a3;
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub struct MPY_16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_16 {}
impl MPY_16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_16::RegisterBlock {
        0x04c0 as *const _
    }
}
impl Deref for MPY_16 {
    type Target = mpy_16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY_16::ptr() }
    }
}
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub mod mpy_16;
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub struct MPY_32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_32 {}
impl MPY_32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_32::RegisterBlock {
        0x04d0 as *const _
    }
}
impl Deref for MPY_32 {
    type Target = mpy_32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY_32::ptr() }
    }
}
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub mod mpy_32;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "Comparator B"]
pub struct COMPARATOR_B {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMPARATOR_B {}
impl COMPARATOR_B {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comparator_b::RegisterBlock {
        0x08c0 as *const _
    }
}
impl Deref for COMPARATOR_B {
    type Target = comparator_b::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMPARATOR_B::ptr() }
    }
}
#[doc = "Comparator B"]
pub mod comparator_b;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORT_MAPPING_PORT_4"]
    pub PORT_MAPPING_PORT_4: PORT_MAPPING_PORT_4,
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "PORT_3_4"]
    pub PORT_3_4: PORT_3_4,
    #[doc = "PORT_5_6"]
    pub PORT_5_6: PORT_5_6,
    #[doc = "PORT_7_8"]
    pub PORT_7_8: PORT_7_8,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "USCI_A1_UART_MODE"]
    pub USCI_A1_UART_MODE: USCI_A1_UART_MODE,
    #[doc = "USCI_A1_SPI_MODE"]
    pub USCI_A1_SPI_MODE: USCI_A1_SPI_MODE,
    #[doc = "USCI_B1_I2C_MODE"]
    pub USCI_B1_I2C_MODE: USCI_B1_I2C_MODE,
    #[doc = "USCI_B1_SPI_MODE"]
    pub USCI_B1_SPI_MODE: USCI_B1_SPI_MODE,
    #[doc = "ADC12"]
    pub ADC12: ADC12,
    #[doc = "USB_CONTROL"]
    pub USB_CONTROL: USB_CONTROL,
    #[doc = "USB_OPERATION"]
    pub USB_OPERATION: USB_OPERATION,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "CRC16"]
    pub CRC16: CRC16,
    #[doc = "RC"]
    pub RC: RC,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "UCS"]
    pub UCS: UCS,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "SHARED_REFERENCE"]
    pub SHARED_REFERENCE: SHARED_REFERENCE,
    #[doc = "PORT_MAPPING_CONTROL"]
    pub PORT_MAPPING_CONTROL: PORT_MAPPING_CONTROL,
    #[doc = "PORT_J"]
    pub PORT_J: PORT_J,
    #[doc = "TIMER_0_A5"]
    pub TIMER_0_A5: TIMER_0_A5,
    #[doc = "TIMER_1_A3"]
    pub TIMER_1_A3: TIMER_1_A3,
    #[doc = "TIMER_0_B7"]
    pub TIMER_0_B7: TIMER_0_B7,
    #[doc = "TIMER_2_A3"]
    pub TIMER_2_A3: TIMER_2_A3,
    #[doc = "MPY_16"]
    pub MPY_16: MPY_16,
    #[doc = "MPY_32"]
    pub MPY_32: MPY_32,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "COMPARATOR_B"]
    pub COMPARATOR_B: COMPARATOR_B,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
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
            PORT_MAPPING_PORT_4: PORT_MAPPING_PORT_4 {
                _marker: PhantomData,
            },
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            PORT_3_4: PORT_3_4 {
                _marker: PhantomData,
            },
            PORT_5_6: PORT_5_6 {
                _marker: PhantomData,
            },
            PORT_7_8: PORT_7_8 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_A1_UART_MODE: USCI_A1_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A1_SPI_MODE: USCI_A1_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B1_I2C_MODE: USCI_B1_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B1_SPI_MODE: USCI_B1_SPI_MODE {
                _marker: PhantomData,
            },
            ADC12: ADC12 {
                _marker: PhantomData,
            },
            USB_CONTROL: USB_CONTROL {
                _marker: PhantomData,
            },
            USB_OPERATION: USB_OPERATION {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            CRC16: CRC16 {
                _marker: PhantomData,
            },
            RC: RC {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            UCS: UCS {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            SHARED_REFERENCE: SHARED_REFERENCE {
                _marker: PhantomData,
            },
            PORT_MAPPING_CONTROL: PORT_MAPPING_CONTROL {
                _marker: PhantomData,
            },
            PORT_J: PORT_J {
                _marker: PhantomData,
            },
            TIMER_0_A5: TIMER_0_A5 {
                _marker: PhantomData,
            },
            TIMER_1_A3: TIMER_1_A3 {
                _marker: PhantomData,
            },
            TIMER_0_B7: TIMER_0_B7 {
                _marker: PhantomData,
            },
            TIMER_2_A3: TIMER_2_A3 {
                _marker: PhantomData,
            },
            MPY_16: MPY_16 {
                _marker: PhantomData,
            },
            MPY_32: MPY_32 {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            COMPARATOR_B: COMPARATOR_B {
                _marker: PhantomData,
            },
        }
    }
}
