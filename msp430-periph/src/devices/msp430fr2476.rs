//! MSP430FR2476
use crate::peripherals::*;

utils::device! {
    /// MSP430FR2476
    #[all:cfg_attr(not(feature = "MSP430FR2476-all"), non_exhaustive)]
    MSP430FR2476;
    /// SFR
    #[all:cfg(feature = "sfr_4")]
    SFR @ 0x0100: sfr_4::SFR;
    /// PMM
    #[all:cfg(feature = "pmm_445")]
    PMM @ 0x0120: pmm_445::PMM;
    /// SYS
    #[all:cfg(feature = "sys_10")]
    SYS @ 0x0140: sys_10::SYS;
    /// CS
    #[all:cfg(feature = "cs_445")]
    CS @ 0x0180: cs_445::CS;
    /// FRCTL
    #[all:cfg(feature = "frctl")]
    FRCTL @ 0x01a0: frctl::FRCTL;
    /// CRC
    #[all:cfg(feature = "crc16_2")]
    CRC @ 0x01c0: crc16_2::CRC16;
    /// WDT_A
    #[all:cfg(feature = "wdt_a")]
    WDT_A @ 0x01cc: wdt_a::WDT_A;
    /// PA
    #[all:cfg(feature = "portw_3i")]
    PA @ 0x0200: portw_3i::Port;
    /// P1
    #[all:cfg(feature = "portb_3i1")]
    P1 @ 0x0200: portb_3i1::Port;
    /// P2
    #[all:cfg(feature = "portb_3i2")]
    P2 @ 0x0201: portb_3i2::Port;
    /// PB
    #[all:cfg(feature = "portw_3i")]
    PB @ 0x0220: portw_3i::Port;
    /// P3
    #[all:cfg(feature = "portb_3i1")]
    P3 @ 0x0220: portb_3i1::Port;
    /// P4
    #[all:cfg(feature = "portb_3i2")]
    P4 @ 0x0221: portb_3i2::Port;
    /// PC
    #[all:cfg(feature = "portw_3i")]
    PC @ 0x0240: portw_3i::Port;
    /// P5
    #[all:cfg(feature = "portb_3i1")]
    P5 @ 0x0240: portb_3i1::Port;
    /// P6
    #[all:cfg(feature = "portb_3i2")]
    P6 @ 0x0241: portb_3i2::Port;
    /// PD
    #[all:cfg(feature = "portw_3i")]
    PD @ 0x0260: portw_3i::Port;
    /// P7
    #[all:cfg(feature = "portb_3i1")]
    P7 @ 0x0260: portb_3i1::Port;
    /// P8
    #[all:cfg(feature = "portb_3i2")]
    P8 @ 0x0261: portb_3i2::Port;
    /// PE
    #[all:cfg(feature = "portw_3i")]
    PE @ 0x0280: portw_3i::Port;
    /// P9
    #[all:cfg(feature = "portb_3i1")]
    P9 @ 0x0280: portb_3i1::Port;
    /// P10
    #[all:cfg(feature = "portb_3i_2720")]
    P10 @ 0x0281: portb_3i_2720::P1x0;
    /// RTC
    #[all:cfg(feature = "rtc_445")]
    RTC @ 0x0300: rtc_445::RTC;
    /// PJ
    #[all:cfg(feature = "portw_3_2720")]
    PJ @ 0x0320: portw_3_2720::Port;
    /// TA0
    #[all:cfg(feature = "ta_3")]
    TA0 @ 0x0380: ta_3::TA;
    /// TA1
    #[all:cfg(feature = "ta_3")]
    TA1 @ 0x03c0: ta_3::TA;
    /// TA2
    #[all:cfg(feature = "ta_3")]
    TA2 @ 0x0400: ta_3::TA;
    /// TA3
    #[all:cfg(feature = "ta_3")]
    TA3 @ 0x0440: ta_3::TA;
    /// TB0
    #[all:cfg(feature = "tb_7")]
    TB0 @ 0x0480: tb_7::TB;
    /// MPY32
    #[all:cfg(feature = "mpy32")]
    MPY32 @ 0x04c0: mpy32::MPY32;
    /// eUSCI_A0
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A0 @ 0x0500: eusci_a::eUSCI_A;
    /// eUSCI_A1
    #[all:cfg(feature = "eusci_a")]
    eUSCI_A1 @ 0x0520: eusci_a::eUSCI_A;
    /// eUSCI_B0
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B0 @ 0x0540: eusci_b::eUSCI_B;
    /// eUSCI_B1
    #[all:cfg(feature = "eusci_b")]
    eUSCI_B1 @ 0x0580: eusci_b::eUSCI_B;
    /// BKMEM
    #[all:cfg(feature = "backup_memory")]
    BKMEM @ 0x0660: backup_memory::BackupMemory;
    /// ADC
    #[all:cfg(feature = "adc_3")]
    ADC @ 0x0700: adc_3::ADC;
    /// eCOMP0
    #[all:cfg(feature = "ecomp")]
    eCOMP0 @ 0x08e0: ecomp::eCOMP;
}
