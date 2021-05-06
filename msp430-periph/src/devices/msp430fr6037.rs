//! MSP430FR6037
use crate::peripherals::*;

utils::device! {
    /// MSP430FR6037
    #[cfg_attr(not(feature = "MSP430FR6037-all"), non_exhaustive)]
    MSP430FR6037;
    /// SFR
    #[cfg(feature = "sfr_2080")]
    SFR @ 0x0100: sfr_2080::SFR;
    /// PMM
    #[cfg(feature = "pmm_3670")]
    PMM @ 0x0120: pmm_3670::PMM;
    /// FRCTL_A
    #[cfg(feature = "frctl_a_3670")]
    FRCTL_A @ 0x0140: frctl_a_3670::FRCTL_A;
    /// CRC
    #[cfg(feature = "crc_2080")]
    CRC @ 0x0150: crc_2080::CRC;
    /// RAMCTL
    #[cfg(feature = "ramctl_3670_config2")]
    RAMCTL @ 0x0158: ramctl_3670_config2::RAMCTL;
    /// WDT_A
    #[cfg(feature = "wdt_a_3560")]
    WDT_A @ 0x015c: wdt_a_3560::WDT_A;
    /// CS
    #[cfg(feature = "cs_3670")]
    CS @ 0x0160: cs_3670::CS;
    /// SYS
    #[cfg(feature = "sys_2080")]
    SYS @ 0x0180: sys_2080::SYS;
    /// REF_A
    #[cfg(feature = "ref_a_3670")]
    REF_A @ 0x01b0: ref_a_3670::REF_A;
    /// PA
    #[cfg(feature = "pa_2720")]
    PA @ 0x0200: pa_2720::PA;
    /// P1
    #[cfg(feature = "p1_2720")]
    P1 @ 0x0200: p1_2720::P1;
    /// P2
    #[cfg(feature = "p2_2720")]
    P2 @ 0x0201: p2_2720::P2;
    /// PB
    #[cfg(feature = "pb_2720")]
    PB @ 0x0220: pb_2720::PB;
    /// P3
    #[cfg(feature = "p3_2720")]
    P3 @ 0x0220: p3_2720::P3;
    /// P4
    #[cfg(feature = "p4_2720")]
    P4 @ 0x0221: p4_2720::P4;
    /// PC
    #[cfg(feature = "pc_2720")]
    PC @ 0x0240: pc_2720::PC;
    /// P5
    #[cfg(feature = "p5_2720")]
    P5 @ 0x0240: p5_2720::P5;
    /// P6
    #[cfg(feature = "p6_2720")]
    P6 @ 0x0241: p6_2720::P6;
    /// PD
    #[cfg(feature = "pd_2720")]
    PD @ 0x0260: pd_2720::PD;
    /// P7
    #[cfg(feature = "p7_2720")]
    P7 @ 0x0260: p7_2720::P7;
    /// P8
    #[cfg(feature = "p8_2720")]
    P8 @ 0x0261: p8_2720::P8;
    /// PE
    #[cfg(feature = "pe_2720")]
    PE @ 0x0280: pe_2720::PE;
    /// P9
    #[cfg(feature = "p9_2720")]
    P9 @ 0x0280: p9_2720::P9;
    /// P10
    #[cfg(feature = "p10_2720")]
    P10 @ 0x0281: p10_2720::P10;
    /// PJ
    #[cfg(feature = "pj_2720")]
    PJ @ 0x0320: pj_2720::PJ;
    /// TA0
    #[cfg(feature = "ta0_3560_inst3")]
    TA0 @ 0x0340: ta0_3560_inst3::TA0;
    /// TA1
    #[cfg(feature = "ta1_3560_inst3")]
    TA1 @ 0x0380: ta1_3560_inst3::TA1;
    /// TB0
    #[cfg(feature = "tb0_3670_inst7")]
    TB0 @ 0x03c0: tb0_3670_inst7::TB0;
    /// TA2
    #[cfg(feature = "ta2_3560_inst2")]
    TA2 @ 0x0400: ta2_3560_inst2::TA2;
    /// CAPTIO0
    #[cfg(feature = "captio0_3670")]
    CAPTIO0 @ 0x043e: captio0_3670::CAPTIO0;
    /// TA3
    #[cfg(feature = "ta3_3560_inst2")]
    TA3 @ 0x0440: ta3_3560_inst2::TA3;
    /// CAPTIO1
    #[cfg(feature = "captio1_3670")]
    CAPTIO1 @ 0x047e: captio1_3670::CAPTIO1;
    /// RTC_C
    #[cfg(feature = "rtc_c_2080")]
    RTC_C @ 0x04a0: rtc_c_2080::RTC_C;
    /// MPY32
    #[cfg(feature = "mpy32_2080")]
    MPY32 @ 0x04c0: mpy32_2080::MPY32;
    /// MPU
    #[cfg(feature = "mpu_3670")]
    MPU @ 0x05a0: mpu_3670::MPU;
    /// eUSCI_A0
    #[cfg(feature = "eusci_a0_2080")]
    eUSCI_A0 @ 0x05c0: eusci_a0_2080::eUSCI_A0;
    /// eUSCI_A1
    #[cfg(feature = "eusci_a1_2080")]
    eUSCI_A1 @ 0x05e0: eusci_a1_2080::eUSCI_A1;
    /// eUSCI_A2
    #[cfg(feature = "eusci_a2_2080")]
    eUSCI_A2 @ 0x0600: eusci_a2_2080::eUSCI_A2;
    /// eUSCI_A3
    #[cfg(feature = "eusci_a3_2080")]
    eUSCI_A3 @ 0x0620: eusci_a3_2080::eUSCI_A3;
    /// eUSCI_B0
    #[cfg(feature = "eusci_b0_2080")]
    eUSCI_B0 @ 0x0640: eusci_b0_2080::eUSCI_B0;
    /// eUSCI_B1
    #[cfg(feature = "eusci_b1_2080")]
    eUSCI_B1 @ 0x0680: eusci_b1_2080::eUSCI_B1;
    /// TA4
    #[cfg(feature = "ta4_3560_inst2")]
    TA4 @ 0x07c0: ta4_3560_inst2::TA4;
    /// ADC12_B
    #[cfg(feature = "adc12_b_3670")]
    ADC12_B @ 0x0800: adc12_b_3670::ADC12_B;
    /// COMP_E
    #[cfg(feature = "comp_e_3670")]
    COMP_E @ 0x08c0: comp_e_3670::COMP_E;
    /// CRC32
    #[cfg(feature = "crc32_3670")]
    CRC32 @ 0x0980: crc32_3670::CRC32;
    /// AES256
    #[cfg(feature = "aes256_3670")]
    AES256 @ 0x09c0: aes256_3670::AES256;
    /// LCD_C
    #[cfg(feature = "lcd_c_2080")]
    LCD_C @ 0x0a00: lcd_c_2080::LCD_C;
    /// LEA
    #[cfg(feature = "lea_3670")]
    LEA @ 0x0a80: lea_3670::LEA;
    /// MTIF
    #[cfg(feature = "mtif_3670")]
    MTIF @ 0x0f00: mtif_3670::MTIF;
}
