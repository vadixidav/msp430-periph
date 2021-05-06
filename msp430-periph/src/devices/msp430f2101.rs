//! MSP430F2101
use crate::peripherals::*;

utils::device! {
    /// MSP430F2101
    #[cfg_attr(not(feature = "MSP430F2101-all"), non_exhaustive)]
    MSP430F2101;
    /// Special Function
    #[cfg(feature = "special_function_10")]
    Special_Function @ 0x0000: special_function_10::SpecialFunction;
    /// Port 1/2
    #[cfg(feature = "port_1_2_2")]
    Port_1_2 @ 0x0020: port_1_2_2::Port12;
    /// System Clock
    #[cfg(feature = "system_clock_4")]
    System_Clock @ 0x0053: system_clock_4::SystemClock;
    /// Comparator A
    #[cfg(feature = "comparator_a_2")]
    Comparator_A @ 0x0059: comparator_a_2::ComparatorA;
    /// Watchdog Timer
    #[cfg(feature = "watchdog_timer_1")]
    Watchdog_Timer @ 0x0120: watchdog_timer_1::WatchdogTimer;
    /// Flash
    #[cfg(feature = "flash_4")]
    Flash @ 0x0128: flash_4::Flash;
    /// Timer A3
    #[cfg(feature = "timer_a3_1")]
    Timer_A3 @ 0x012e: timer_a3_1::TimerA3;
    /// Calibration Data
    #[cfg(feature = "calibration_data_1")]
    Calibration_Data @ 0x10f8: calibration_data_1::CalibrationData;
}
