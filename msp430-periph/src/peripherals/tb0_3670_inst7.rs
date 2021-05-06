//! TB0

utils::periph! {
    /// TB0
    TB0;
    /// Timer_B Control Register
    rw TB0CTL @ 0x00: u16 = 0_0 {
        /// TimerB interrupt flag
        TBIFG: 0..0 = enum TBIFG {
            /// No interrupt pending
            TBIFG_0 = 0b0,
            /// Interrupt pending
            TBIFG_1 = 0b1,
        }
        /// TimerB interrupt enable
        TBIE: 1..1 = enum TBIE {
            /// Interrupt disabled
            TBIE_0 = 0b0,
            /// Interrupt enabled
            TBIE_1 = 0b1,
        }
        /// TimerB clear
        TBCLR: 2 = struct TBCLR(bool);
        /// Mode control
        MC: 4..5 = enum MC {
            /// Stop mode: Timer is halted
            STOP = 0b00,
            /// Up mode: Timer counts up to TBxCL0
            UP = 0b01,
            /// Continuous mode: Timer counts up to the value set by CNTL
            CONTINUOUS = 0b10,
            /// Up/down mode: Timer counts up to TBxCL0 then down to 0000h
            UPDOWN = 0b11,
        }
        /// Input divider
        ID: 6..7 = enum ID {
            /// /1
            _1 = 0b00,
            /// /2
            _2 = 0b01,
            /// /4
            _4 = 0b10,
            /// /8
            _8 = 0b11,
        }
        /// TimerB clock source select
        TBSSEL: 8..9 = enum TBSSEL {
            /// TBxCLK
            TBCLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// INCLK
            INCLK = 0b11,
        }
        /// Counter length
        CNTL: 11..12 = enum CNTL {
            /// 16-bit, TBxR(max) = 0FFFFh
            _16 = 0b00,
            /// 12-bit, TBxR(max) = 0FFFh
            _12 = 0b01,
            /// 10-bit, TBxR(max) = 03FFh
            _10 = 0b10,
            /// 8-bit, TBxR(max) = 0FFh
            _8 = 0b11,
        }
        /// TBxCLn group
        TBCLGRP: 13..14 = enum TBCLGRP {
            /// Each TBxCLn latch loads independently
            TBCLGRP_0 = 0b00,
            /// TBxCL1+TBxCL2 (TBxCCR1 CLLD bits control the update); TBxCL3+TBxCL4 (TBxCCR3 CLLD bits control the update); TBxCL5+TBxCL6 (TBxCCR5 CLLD bits control the update); TBxCL0 independent
            TBCLGRP_1 = 0b01,
            /// TBxCL1+TBxCL2+TBxCL3 (TBxCCR1 CLLD bits control the update); TBxCL4+TBxCL5+TBxCL6 (TBxCCR4 CLLD bits control the update); TBxCL0 independent
            TBCLGRP_2 = 0b10,
            /// TBxCL0+TBxCL1+TBxCL2+TBxCL3+TBxCL4+TBxCL5+TBxCL6 (TBxCCR1 CLLD bits control the update)
            TBCLGRP_3 = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL0_CCIFG: 0..0 = enum TB0CCTL0_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL0_COV: 1..1 = enum TB0CCTL0_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL0_OUT: 2..2 = enum TB0CCTL0_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL0_CCI: 3 = struct TB0CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL0_CCIE: 4..4 = enum TB0CCTL0_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL0_OUTMOD: 5..7 = enum TB0CCTL0_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL0_CAP: 8..8 = enum TB0CCTL0_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL0_CLLD: 9..10 = enum TB0CCTL0_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL0_SCS: 11..11 = enum TB0CCTL0_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL0_CCIS: 12..13 = enum TB0CCTL0_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL0_CM: 14..15 = enum TB0CCTL0_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL1_CCIFG: 0..0 = enum TB0CCTL1_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL1_COV: 1..1 = enum TB0CCTL1_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL1_OUT: 2..2 = enum TB0CCTL1_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL1_CCI: 3 = struct TB0CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL1_CCIE: 4..4 = enum TB0CCTL1_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL1_OUTMOD: 5..7 = enum TB0CCTL1_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL1_CAP: 8..8 = enum TB0CCTL1_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL1_CLLD: 9..10 = enum TB0CCTL1_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL1_SCS: 11..11 = enum TB0CCTL1_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL1_CCIS: 12..13 = enum TB0CCTL1_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL1_CM: 14..15 = enum TB0CCTL1_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL2_CCIFG: 0..0 = enum TB0CCTL2_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL2_COV: 1..1 = enum TB0CCTL2_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL2_OUT: 2..2 = enum TB0CCTL2_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL2_CCI: 3 = struct TB0CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL2_CCIE: 4..4 = enum TB0CCTL2_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL2_OUTMOD: 5..7 = enum TB0CCTL2_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL2_CAP: 8..8 = enum TB0CCTL2_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL2_CLLD: 9..10 = enum TB0CCTL2_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL2_SCS: 11..11 = enum TB0CCTL2_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL2_CCIS: 12..13 = enum TB0CCTL2_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL2_CM: 14..15 = enum TB0CCTL2_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL3_CCIFG: 0..0 = enum TB0CCTL3_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL3_COV: 1..1 = enum TB0CCTL3_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL3_OUT: 2..2 = enum TB0CCTL3_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL3_CCI: 3 = struct TB0CCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL3_CCIE: 4..4 = enum TB0CCTL3_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL3_OUTMOD: 5..7 = enum TB0CCTL3_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL3_CAP: 8..8 = enum TB0CCTL3_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL3_CLLD: 9..10 = enum TB0CCTL3_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL3_SCS: 11..11 = enum TB0CCTL3_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL3_CCIS: 12..13 = enum TB0CCTL3_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL3_CM: 14..15 = enum TB0CCTL3_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL4_CCIFG: 0..0 = enum TB0CCTL4_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL4_COV: 1..1 = enum TB0CCTL4_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL4_OUT: 2..2 = enum TB0CCTL4_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL4_CCI: 3 = struct TB0CCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL4_CCIE: 4..4 = enum TB0CCTL4_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL4_OUTMOD: 5..7 = enum TB0CCTL4_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL4_CAP: 8..8 = enum TB0CCTL4_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL4_CLLD: 9..10 = enum TB0CCTL4_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL4_SCS: 11..11 = enum TB0CCTL4_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL4_CCIS: 12..13 = enum TB0CCTL4_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL4_CM: 14..15 = enum TB0CCTL4_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL5 @ 0x0c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL5_CCIFG: 0..0 = enum TB0CCTL5_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL5_COV: 1..1 = enum TB0CCTL5_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL5_OUT: 2..2 = enum TB0CCTL5_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL5_CCI: 3 = struct TB0CCTL5_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL5_CCIE: 4..4 = enum TB0CCTL5_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL5_OUTMOD: 5..7 = enum TB0CCTL5_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL5_CAP: 8..8 = enum TB0CCTL5_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL5_CLLD: 9..10 = enum TB0CCTL5_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL5_SCS: 11..11 = enum TB0CCTL5_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL5_CCIS: 12..13 = enum TB0CCTL5_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL5_CM: 14..15 = enum TB0CCTL5_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B Capture/Compare Control Register
    rw TB0CCTL6 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TB0CCTL6_CCIFG: 0..0 = enum TB0CCTL6_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TB0CCTL6_COV: 1..1 = enum TB0CCTL6_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TB0CCTL6_OUT: 2..2 = enum TB0CCTL6_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TB0CCTL6_CCI: 3 = struct TB0CCTL6_CCI(bool);
        /// Capture/compare interrupt enable
        TB0CCTL6_CCIE: 4..4 = enum TB0CCTL6_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TB0CCTL6_OUTMOD: 5..7 = enum TB0CCTL6_OUTMOD {
            /// OUT bit value
            OUTMOD_0 = 0b000,
            /// Set
            OUTMOD_1 = 0b001,
            /// Toggle/reset
            OUTMOD_2 = 0b010,
            /// Set/reset
            OUTMOD_3 = 0b011,
            /// Toggle
            OUTMOD_4 = 0b100,
            /// Reset
            OUTMOD_5 = 0b101,
            /// Toggle/set
            OUTMOD_6 = 0b110,
            /// Reset/set
            OUTMOD_7 = 0b111,
        }
        /// Capture mode
        TB0CCTL6_CAP: 8..8 = enum TB0CCTL6_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Compare latch load
        TB0CCTL6_CLLD: 9..10 = enum TB0CCTL6_CLLD {
            /// TBxCLn loads on write to TBxCCRn
            CLLD_0 = 0b00,
            /// TBxCLn loads when TBxR counts to 0
            CLLD_1 = 0b01,
            /// TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode).
            CLLD_2 = 0b10,
            /// TBxCLn loads when TBxR counts to TBxCLn
            CLLD_3 = 0b11,
        }
        /// Synchronize capture source
        TB0CCTL6_SCS: 11..11 = enum TB0CCTL6_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TB0CCTL6_CCIS: 12..13 = enum TB0CCTL6_CCIS {
            /// CCIxA
            CCIA = 0b00,
            /// CCIxB
            CCIB = 0b01,
            /// GND
            GND = 0b10,
            /// VCC
            VCC = 0b11,
        }
        /// Capture mode
        TB0CCTL6_CM: 14..15 = enum TB0CCTL6_CM {
            /// No capture
            NONE = 0b00,
            /// Capture on rising edge
            RISING = 0b01,
            /// Capture on falling edge
            FALLING = 0b10,
            /// Capture on both rising and falling edges
            BOTH = 0b11,
        }
    }
    /// Timer_B count register
    rw TB0R @ 0x10: u16 = 0_0 {
        /// Timer_B count register
        TB0R: 0..15 = struct TB0RField(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR0 @ 0x12: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR0: 0..15 = struct TB0CCR0Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR1 @ 0x14: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR1: 0..15 = struct TB0CCR1Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR2 @ 0x16: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR2: 0..15 = struct TB0CCR2Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR3 @ 0x18: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR3: 0..15 = struct TB0CCR3Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR4: 0..15 = struct TB0CCR4Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR5 @ 0x1c: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR5: 0..15 = struct TB0CCR5Field(u16);
    }
    /// Timer_B Capture/Compare  Register
    rw TB0CCR6 @ 0x1e: u16 = 0_0 {
        /// Timer_B Capture/Compare  Register
        TB0CCR6: 0..15 = struct TB0CCR6Field(u16);
    }
    /// Timer_Bx Expansion Register 0
    rw TB0EX0 @ 0x20: u16 = 0_0 {
        /// Input divider expansion
        TBIDEX: 0..2 = enum TBIDEX {
            /// Divide by 1
            _1 = 0b000,
            /// Divide by 2
            _2 = 0b001,
            /// Divide by 3
            _3 = 0b010,
            /// Divide by 4
            _4 = 0b011,
            /// Divide by 5
            _5 = 0b100,
            /// Divide by 6
            _6 = 0b101,
            /// Divide by 7
            _7 = 0b110,
            /// Divide by 8
            _8 = 0b111,
        }
    }
    /// Timer_Bx Interrupt Vector Register
    r TB0IV @ 0x2e: u16 = 0_0 {
        /// Timer_B interrupt vector value
        TBIV: 0..15 = enum TBIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest
            TBCCR1 = 0b0000000000000010,
            /// Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG
            TBCCR2 = 0b0000000000000100,
            /// Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG
            TBCCR3 = 0b0000000000000110,
            /// Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG
            TBCCR4 = 0b0000000000001000,
            /// Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG
            TBCCR5 = 0b0000000000001010,
            /// Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG
            TBCCR6 = 0b0000000000001100,
            /// Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest
            TBIFG = 0b0000000000001110,
        }
    }
}