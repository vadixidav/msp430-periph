//! MSP430F2619
use crate::peripherals::*;

utils::device! {
    /// MSP430F2619
    #[all:cfg_attr(not(feature = "MSP430F2619-all"), non_exhaustive)]
    MSP430F2619;
    /// Special Function
    #[all:cfg(feature = "special_function_12")]
    Special_Function @ 0x0000: special_function_12::SpecialFunction;
    /// Port 3/4
    #[all:cfg(feature = "port_3_4_3")]
    Port_3_4 @ 0x0010: port_3_4_3::Port34;
    /// Port 5/6
    #[all:cfg(feature = "port_5_6_2")]
    Port_5_6 @ 0x0012: port_5_6_2::Port56;
    /// Port 7/8
    #[all:cfg(feature = "port_7_8_2")]
    Port_7_8 @ 0x0014: port_7_8_2::Port78;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// System Clock
    #[all:cfg(feature = "system_clock_5")]
    System_Clock @ 0x0053: system_clock_5::SystemClock;
    /// Supply Voltage Supervisor
    #[all:cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0055: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
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
    /// ADC12
    #[all:cfg(feature = "adc12_1")]
    ADC12 @ 0x0080: adc12_1::ADC12;
    /// USCI_A1  UART Mode
    #[all:cfg(feature = "usci_a_uart_1")]
    USCI_A1_UART @ 0x00cd: usci_a_uart_1::USCI_A_UART;
    /// USCI_A1  SPI Mode
    #[all:cfg(feature = "usci_a_spi_1")]
    USCI_A1_SPI @ 0x00d0: usci_a_spi_1::USCI_A_SPI;
    /// USCI_B1  SPI Mode
    #[all:cfg(feature = "usci_b_spi_1")]
    USCI_B1_SPI @ 0x00d8: usci_b_spi_1::USCI_B_SPI;
    /// USCI_B1  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_3")]
    USCI_B1_I2C @ 0x00d8: usci_b_i2c_3::USCI_B_I2C;
    /// Timer B7
    #[all:cfg(feature = "timer_b7_1")]
    Timer_B7 @ 0x011e: timer_b7_1::TimerB7;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// DMA
    #[all:cfg(feature = "dma_3")]
    DMA @ 0x0122: dma_3::DMA;
    /// Flash
    #[all:cfg(feature = "flash_5")]
    Flash @ 0x0128: flash_5::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Multiplier
    #[all:cfg(feature = "multiplier")]
    Multiplier @ 0x0130: multiplier::Multiplier;
    /// DAC12
    #[all:cfg(feature = "dac12_1")]
    DAC12 @ 0x01c0: dac12_1::DAC12;
    /// TLV Calibration Data
    #[all:cfg(feature = "tlv_calibration_data_2")]
    TLV_Calibration_Data @ 0x10c0: tlv_calibration_data_2::TLVCalibrationData;
    /// Calibration Data
    #[all:cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
