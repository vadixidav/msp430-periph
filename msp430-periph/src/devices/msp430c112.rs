//! MSP430C112
use crate::peripherals::*;

utils::device! {
    /// MSP430C112
    #[all:cfg_attr(not(feature = "MSP430C112-all"), non_exhaustive)]
    MSP430C112;
    /// Special Function
    #[all:cfg(feature = "special_function_1")]
    Special_Function @ 0x0000: special_function_1::SpecialFunction;
    /// Port 1/2
    #[all:cfg(feature = "port_1_2_1")]
    Port_1_2 @ 0x0020: port_1_2_1::Port12;
    /// EPROM
    #[all:cfg(feature = "eprom")]
    EPROM @ 0x0054: eprom::EPROM;
    /// System Clock
    #[all:cfg(feature = "system_clock_1")]
    System_Clock @ 0x0056: system_clock_1::SystemClock;
    /// Watchdog Timer
    #[all:cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[all:cfg(feature = "flash_1")]
    Flash @ 0x0128: flash_1::Flash;
    /// Timer A3
    #[all:cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
}
