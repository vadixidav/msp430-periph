//! MSP430i2041
use crate::peripherals::*;

utils::device! {
    /// MSP430i2041
    #[all:cfg_attr(not(feature = "MSP430i2041-all"), non_exhaustive)]
    MSP430i2041;
    /// Special Function
    #[all:cfg(feature = "special_function_20")]
    Special_Function @ 0x0000: special_function_20::SpecialFunction;
    /// Port A
    #[all:cfg(feature = "port_a_4")]
    Port_A @ 0x0010: port_a_4::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_8")]
    Port_1_2 @ 0x0010: port_1_2_8::Port12;
    /// CS  Clock System
    #[all:cfg(feature = "cs_4")]
    CS @ 0x0050: cs_4::CS;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_5")]
    PMM @ 0x0060: pmm_5::PMM;
    /// SD24_A4
    #[all:cfg(feature = "sd24_a4_1")]
    SD24_A4 @ 0x00b0: sd24_a4_1::SD24_A4;
    /// Timer1_A3
    #[all:cfg(feature = "timer1_a3_2")]
    Timer1_A3 @ 0x011e: timer1_a3_2::Timer1_A3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_7")]
    Flash @ 0x0128: flash_7::Flash;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_1")]
    Timer0_A3 @ 0x012e: timer0_a3_1::Timer0_A3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_3")]
    USCI_A0_UART @ 0x0140: usci_a_uart_3::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_3")]
    USCI_A0_SPI @ 0x0140: usci_a_spi_3::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_3")]
    USCI_B0_SPI @ 0x01c0: usci_b_spi_3::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_5")]
    USCI_B0_I2C @ 0x01c0: usci_b_i2c_5::USCI_B_I2C;
}
