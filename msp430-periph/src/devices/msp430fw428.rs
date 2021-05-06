//! MSP430FW428
use crate::peripherals::*;

utils::device! {
    /// MSP430FW428
    #[cfg_attr(not(feature = "MSP430FW428-all"), non_exhaustive)]
    MSP430FW428;
    /// Special Function
    #[cfg(feature = "special_function_6")]
    Special_Function @ 0x0000: special_function_6::SpecialFunction;
    /// Port 3/4
    #[cfg(feature = "port_3_4_1")]
    Port_3_4 @ 0x0018: port_3_4_1::Port34;
    /// Port 1/2
    #[cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// Port 5/6
    #[cfg(feature = "port_5_6_1")]
    Port_5_6 @ 0x0030: port_5_6_1::Port56;
    /// Basic Timer
    #[cfg(feature = "basic_timer_1")]
    Basic_Timer @ 0x0040: basic_timer_1::BasicTimer;
    /// System Clock FLLPLUS
    #[cfg(feature = "system_clock_fllplus_1")]
    System_Clock_FLLPLUS @ 0x0050: system_clock_fllplus_1::SystemClockFLLPLUS;
    /// Supply Voltage Supervisor
    #[cfg(feature = "supply_voltage_supervisor_2")]
    Supply_Voltage_Supervisor @ 0x0056: supply_voltage_supervisor_2::SupplyVoltageSupervisor;
    /// Comparator A
    #[cfg(feature = "comparator_a_1")]
    Comparator_A @ 0x0059: comparator_a_1::ComparatorA;
    /// LCD
    #[cfg(feature = "lcd_2")]
    LCD @ 0x0090: lcd_2::LCD;
    /// Timer1_A5
    #[cfg(feature = "timer1_a5_1")]
    Timer1_A5 @ 0x011e: timer1_a5_1::Timer1_A5;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer0_A3
    #[cfg(feature = "timer0_a3_1")]
    Timer0_A3 @ 0x012e: timer0_a3_1::Timer0_A3;
    /// Scan Interface
    #[cfg(feature = "scan_interface_1")]
    Scan_Interface @ 0x01b0: scan_interface_1::ScanInterface;
}
