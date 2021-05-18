//! RF430F5155
use crate::peripherals::*;

utils::device! {
    /// RF430F5155
    #[all:cfg_attr(not(feature = "RF430F5155-all"), non_exhaustive)]
    RF430F5155;
    /// SFR  Special Function Registers
    #[all:cfg(feature = "sfr_1")]
    SFR @ 0x0100: sfr_1::SFR;
    /// PMM  Power Management System
    #[all:cfg(feature = "pmm_2")]
    PMM @ 0x0120: pmm_2::PMM;
    /// Flash
    #[all:cfg(feature = "flash_6")]
    Flash @ 0x0140: flash_6::Flash;
    /// CRC16
    #[all:cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// RC  RAM Control Module
    #[all:cfg(feature = "rc_1")]
    RC @ 0x0158: rc_1::RC;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// UCS  Unified System Clock
    #[all:cfg(feature = "ucs_5")]
    UCS @ 0x0160: ucs_5::UCS;
    /// SYS  System Module
    #[all:cfg(feature = "sys_2")]
    SYS @ 0x0180: sys_2::SYS;
    /// Shared Reference
    #[all:cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port Mapping Control
    #[all:cfg(feature = "port_mapping_control")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control::PortMappingControl;
    /// Port Mapping Port 1
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_1 @ 0x01c8: port_mapping::PortMapping;
    /// Port Mapping Port 2
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping::PortMapping;
    /// Port Mapping Port 3
    #[all:cfg(feature = "port_mapping")]
    Port_Mapping_Port_3 @ 0x01d8: port_mapping::PortMapping;
    /// Port A
    #[all:cfg(feature = "port_a_1")]
    Port_A @ 0x0200: port_a_1::PortA;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_5")]
    Port_1_2 @ 0x0200: port_1_2_5::Port12;
    /// Port B
    #[all:cfg(feature = "port_b_1")]
    Port_B @ 0x0220: port_b_1::PortB;
    /// Port 3
    #[all:cfg(feature = "port_3_2")]
    Port_3 @ 0x0220: port_3_2::Port3;
    /// Port J
    #[all:cfg(feature = "port_j_2")]
    Port_J @ 0x0320: port_j_2::PortJ;
    /// Timer0_A3
    #[all:cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x03c0: timer0_a3_2::Timer0_A3;
    /// MPY 16  Multiplier  16 Bit Mode
    #[all:cfg(feature = "mpy_16")]
    MPY_16 @ 0x04c0: mpy_16::MPY16;
    /// MPY 32  Multiplier  32 Bit Mode
    #[all:cfg(feature = "mpy_32")]
    MPY_32 @ 0x04d0: mpy_32::MPY32;
    /// DMA
    #[all:cfg(feature = "dma_17")]
    DMA @ 0x0500: dma_17::DMA;
    /// USCI_A0  UART Mode
    #[all:cfg(feature = "usci_a_uart_2")]
    USCI_A0_UART @ 0x05c0: usci_a_uart_2::USCI_A_UART;
    /// USCI_A0  SPI Mode
    #[all:cfg(feature = "usci_a_spi_2")]
    USCI_A0_SPI @ 0x05c0: usci_a_spi_2::USCI_A_SPI;
    /// USCI_B0  SPI Mode
    #[all:cfg(feature = "usci_b_spi_2")]
    USCI_B0_SPI @ 0x05e0: usci_b_spi_2::USCI_B_SPI;
    /// USCI_B0  I2C Mode
    #[all:cfg(feature = "usci_b_i2c_4")]
    USCI_B0_I2C @ 0x05e0: usci_b_i2c_4::USCI_B_I2C;
    /// ADC10_A
    #[all:cfg(feature = "adc10_ab")]
    ADC10_A @ 0x0740: adc10_ab::ADC10_AB;
    /// Comparator B
    #[all:cfg(feature = "comparator_b")]
    Comparator_B @ 0x08c0: comparator_b::ComparatorB;
    /// Timer0_D3
    #[all:cfg(feature = "timer0_d3_1")]
    Timer0_D3 @ 0x0b00: timer0_d3_1::Timer0_D3;
    /// Timer1_D3
    #[all:cfg(feature = "timer1_d3_1")]
    Timer1_D3 @ 0x0b40: timer1_d3_1::Timer1_D3;
    /// Timer_Event_Control
    #[all:cfg(feature = "timer_event_control")]
    Timer_Event_Control @ 0x0c00: timer_event_control::Timer_Event_Control;
    /// Timer_Event_Control
    #[all:cfg(feature = "timer_event_control")]
    Timer_Event_Control1 @ 0x0c20: timer_event_control::Timer_Event_Control;
}
