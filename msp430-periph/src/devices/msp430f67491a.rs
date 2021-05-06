//! MSP430F67491A
use crate::peripherals::*;

utils::device! {
    /// MSP430F67491A
    #[cfg_attr(not(feature = "MSP430F67491A-all"), non_exhaustive)]
    MSP430F67491A;
    /// SFR  Special Function Registers
    #[cfg(feature = "sfr__special_function_registers_1")]
    SFR__Special_Function_Registers @ 0x0100: sfr__special_function_registers_1::SFRSpecialFunctionRegisters;
    /// PMM  Power Management System
    #[cfg(feature = "pmm__power_management_system_2")]
    PMM__Power_Management_System @ 0x0120: pmm__power_management_system_2::PMMPowerManagementSystem;
    /// Flash
    #[cfg(feature = "flash_6")]
    Flash @ 0x0140: flash_6::Flash;
    /// CRC16
    #[cfg(feature = "crc16_2")]
    CRC16 @ 0x0150: crc16_2::CRC16;
    /// RC  RAM Control Module
    #[cfg(feature = "rc__ram_control_module_1")]
    RC__RAM_Control_Module @ 0x0158: rc__ram_control_module_1::RCRAMControlModule;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_2")]
    Watchdog_Timer @ 0x015c: watchdog_timer_2::WatchdogTimer;
    /// UCS  Unified System Clock
    #[cfg(feature = "ucs__unified_system_clock_3")]
    UCS__Unified_System_Clock @ 0x0160: ucs__unified_system_clock_3::UCSUnifiedSystemClock;
    /// SYS  System Module
    #[cfg(feature = "sys__system_module_2")]
    SYS__System_Module @ 0x0180: sys__system_module_2::SYSSystemModule;
    /// Shared Reference
    #[cfg(feature = "shared_reference_1")]
    Shared_Reference @ 0x01b0: shared_reference_1::SharedReference;
    /// Port Mapping Control
    #[cfg(feature = "port_mapping_control_1")]
    Port_Mapping_Control @ 0x01c0: port_mapping_control_1::PortMappingControl;
    /// Port Mapping Port 2
    #[cfg(feature = "port_mapping_port_2_1")]
    Port_Mapping_Port_2 @ 0x01d0: port_mapping_port_2_1::PortMappingPort2;
    /// Port Mapping Port 3
    #[cfg(feature = "port_mapping_port_3_1")]
    Port_Mapping_Port_3 @ 0x01d8: port_mapping_port_3_1::PortMappingPort3;
    /// Port Mapping Port 4
    #[cfg(feature = "port_mapping_port_4_1")]
    Port_Mapping_Port_4 @ 0x01e0: port_mapping_port_4_1::PortMappingPort4;
    /// Port A
    #[cfg(feature = "port_a_2")]
    Port_A @ 0x0200: port_a_2::PortA;
    /// Port 1/2
    #[cfg(feature = "port_1_2_6")]
    Port_1_2 @ 0x0200: port_1_2_6::Port12;
    /// Port B
    #[cfg(feature = "port_b_3")]
    Port_B @ 0x0220: port_b_3::PortB;
    /// Port 3/4
    #[cfg(feature = "port_3_4_6")]
    Port_3_4 @ 0x0220: port_3_4_6::Port34;
    /// Port C
    #[cfg(feature = "port_c_2")]
    Port_C @ 0x0240: port_c_2::PortC;
    /// Port 5/6
    #[cfg(feature = "port_5_6_4")]
    Port_5_6 @ 0x0240: port_5_6_4::Port56;
    /// Port D
    #[cfg(feature = "port_d_2")]
    Port_D @ 0x0260: port_d_2::PortD;
    /// Port 7/8
    #[cfg(feature = "port_7_8_4")]
    Port_7_8 @ 0x0260: port_7_8_4::Port78;
    /// Port E
    #[cfg(feature = "port_e_2")]
    Port_E @ 0x0280: port_e_2::PortE;
    /// Port 9/10
    #[cfg(feature = "port_9_10_4")]
    Port_9_10 @ 0x0280: port_9_10_4::Port910;
    /// Port F
    #[cfg(feature = "port_f_2")]
    Port_F @ 0x02a0: port_f_2::PortF;
    /// Port 11
    #[cfg(feature = "port_11_2")]
    Port_11 @ 0x02a0: port_11_2::Port11;
    /// Port J
    #[cfg(feature = "port_j_3")]
    Port_J @ 0x0320: port_j_3::PortJ;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_2")]
    Timer0_A3 @ 0x0340: timer0_a3_2::Timer0_A3;
    /// Timer1_A2
    #[cfg(feature = "timer1_a2_2")]
    Timer1_A2 @ 0x0380: timer1_a2_2::Timer1_A2;
    /// Timer2_A2
    #[cfg(feature = "timer2_a2_1")]
    Timer2_A2 @ 0x0400: timer2_a2_1::Timer2_A2;
    /// Timer3_A2
    #[cfg(feature = "timer3_a2_1")]
    Timer3_A2 @ 0x0440: timer3_a2_1::Timer3_A2;
    /// Backup RAM
    #[cfg(feature = "backup_ram_1")]
    Backup_RAM @ 0x0480: backup_ram_1::BackupRAM;
    /// MPY 16  Multiplier  16 Bit Mode
    #[cfg(feature = "mpy_16__multiplier__16_bit_mode_1")]
    MPY_16__Multiplier__16_Bit_Mode @ 0x04c0: mpy_16__multiplier__16_bit_mode_1::MPY16Multiplier16BitMode;
    /// MPY 32  Multiplier  32 Bit Mode
    #[cfg(feature = "mpy_32__multiplier__32_bit_mode_1")]
    MPY_32__Multiplier__32_Bit_Mode @ 0x04d0: mpy_32__multiplier__32_bit_mode_1::MPY32Multiplier32BitMode;
    /// DMA
    #[cfg(feature = "dma_17")]
    DMA @ 0x0500: dma_17::DMA;
    /// USCI_A0  UART Mode
    #[cfg(feature = "usci_a0__uart_mode_3")]
    USCI_A0__UART_Mode @ 0x05c0: usci_a0__uart_mode_3::USCI_A0UARTMode;
    /// USCI_A0  SPI Mode
    #[cfg(feature = "usci_a0__spi_mode_3")]
    USCI_A0__SPI_Mode @ 0x05c0: usci_a0__spi_mode_3::USCI_A0SPIMode;
    /// USCI_A1  UART Mode
    #[cfg(feature = "usci_a1__uart_mode_3")]
    USCI_A1__UART_Mode @ 0x05e0: usci_a1__uart_mode_3::USCI_A1UARTMode;
    /// USCI_A1  SPI Mode
    #[cfg(feature = "usci_a1__spi_mode_3")]
    USCI_A1__SPI_Mode @ 0x05e0: usci_a1__spi_mode_3::USCI_A1SPIMode;
    /// USCI_A2  UART Mode
    #[cfg(feature = "usci_a2__uart_mode_2")]
    USCI_A2__UART_Mode @ 0x0600: usci_a2__uart_mode_2::USCI_A2UARTMode;
    /// USCI_A2  SPI Mode
    #[cfg(feature = "usci_a2__spi_mode_2")]
    USCI_A2__SPI_Mode @ 0x0600: usci_a2__spi_mode_2::USCI_A2SPIMode;
    /// USCI_A3  UART Mode
    #[cfg(feature = "usci_a3__uart_mode_2")]
    USCI_A3__UART_Mode @ 0x0620: usci_a3__uart_mode_2::USCI_A3UARTMode;
    /// USCI_A3  SPI Mode
    #[cfg(feature = "usci_a3__spi_mode_2")]
    USCI_A3__SPI_Mode @ 0x0620: usci_a3__spi_mode_2::USCI_A3SPIMode;
    /// USCI_B0  SPI Mode
    #[cfg(feature = "usci_b0__spi_mode_3")]
    USCI_B0__SPI_Mode @ 0x0640: usci_b0__spi_mode_3::USCI_B0SPIMode;
    /// USCI_B0  I2C Mode
    #[cfg(feature = "usci_b0__i2c_mode_4")]
    USCI_B0__I2C_Mode @ 0x0640: usci_b0__i2c_mode_4::USCI_B0I2CMode;
    /// USCI_B1  SPI Mode
    #[cfg(feature = "usci_b1__spi_mode_3")]
    USCI_B1__SPI_Mode @ 0x0680: usci_b1__spi_mode_3::USCI_B1SPIMode;
    /// USCI_B1  I2C Mode
    #[cfg(feature = "usci_b1__i2c_mode_3")]
    USCI_B1__I2C_Mode @ 0x0680: usci_b1__i2c_mode_3::USCI_B1I2CMode;
    /// ADC10_A
    #[cfg(feature = "adc10_a_1")]
    ADC10_A @ 0x0740: adc10_a_1::ADC10_A;
    /// SD24_B4
    #[cfg(feature = "sd24_b4_1")]
    SD24_B4 @ 0x0800: sd24_b4_1::SD24_B4;
    /// Comparator B
    #[cfg(feature = "comparator_b_1")]
    Comparator_B @ 0x08c0: comparator_b_1::ComparatorB;
    /// Auxilary Supply
    #[cfg(feature = "auxilary_supply_1")]
    Auxilary_Supply @ 0x09e0: auxilary_supply_1::AuxilarySupply;
    /// LCD_C
    #[cfg(feature = "lcd_c_1")]
    LCD_C @ 0x0a00: lcd_c_1::LCD_C;
    /// RTC_CE  Real Time Clock
    #[cfg(feature = "rtc_ce__real_time_clock_2")]
    RTC_CE__Real_Time_Clock @ 0x0c80: rtc_ce__real_time_clock_2::RTC_CERealTimeClock;
}
