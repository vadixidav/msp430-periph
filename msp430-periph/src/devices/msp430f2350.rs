//! MSP430F2350
use crate::peripherals::*;

utils::device! {
    /// MSP430F2350
    #[all:cfg_attr(not(feature = "MSP430F2350-all"), non_exhaustive)]
    MSP430F2350;
    /// Special Function
    #[all:cfg(feature = "special_function_11")]
    Special_Function @ 0x0000: special_function_11::SpecialFunction;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_3")]
    Port_3_4 @ 0x0010: port_3_4_3::Port34;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// System Clock
    #[all:cfg(feature = "system_clock_5")]
    System_Clock @ 0x0053: system_clock_5::SystemClock;
    /// Comparator A
    #[all:cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A0_UART @ 0x005d: usci_a_uart_1::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A0_SPI @ 0x0060: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B0_SPI @ 0x0068: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_1")]
    USCI_B0_I2C @ 0x0068: usci_b_i2c_1::USCI_B_I2C;
    /// Timer B3
    #[all:cfg(feature = "timer_b3_1")]
    Timer_B3 @ 0x011e: timer_b3_1::TimerB3;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_4")]
    Flash @ 0x0128: flash_4::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
