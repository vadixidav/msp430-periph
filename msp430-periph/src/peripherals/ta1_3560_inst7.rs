//! TA1

utils::periph! {
    /// TA1
    TA1;
    /// TimerAx Control Register
    rw TA1CTL @ 0x00: u16 = 0_0 {
        /// TimerA interrupt flag
        TAIFG: 0..0 = enum TAIFG {
            /// No interrupt pending
            TAIFG_0 = 0b0,
            /// Interrupt pending
            TAIFG_1 = 0b1,
        }
        /// TimerA interrupt enable
        TAIE: 1..1 = enum TAIE {
            /// Interrupt disabled
            TAIE_0 = 0b0,
            /// Interrupt enabled
            TAIE_1 = 0b1,
        }
        /// TimerA clear
        TACLR: 2 = struct TACLR(bool);
        /// Mode control
        MC: 4..5 = enum MC {
            /// Stop mode: Timer is halted
            STOP = 0b00,
            /// Up mode: Timer counts up to TAxCCR0
            UP = 0b01,
            /// Continuous mode: Timer counts up to 0FFFFh
            CONTINUOUS = 0b10,
            /// Up/down mode: Timer counts up to TAxCCR0 then down to 0000h
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
        /// TimerA clock source select
        TASSEL: 8..9 = enum TASSEL {
            /// TAxCLK
            TACLK = 0b00,
            /// ACLK
            ACLK = 0b01,
            /// SMCLK
            SMCLK = 0b10,
            /// INCLK
            INCLK = 0b11,
        }
    }
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL0 @ 0x02: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL0_CCIFG: 0..0 = enum TA1CCTL0_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL0_COV: 1..1 = enum TA1CCTL0_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL0_OUT: 2..2 = enum TA1CCTL0_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL0_CCI: 3 = struct TA1CCTL0_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL0_CCIE: 4..4 = enum TA1CCTL0_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL0_OUTMOD: 5..7 = enum TA1CCTL0_OUTMOD {
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
        TA1CCTL0_CAP: 8..8 = enum TA1CCTL0_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL0_SCCI: 10 = struct TA1CCTL0_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL0_SCS: 11..11 = enum TA1CCTL0_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL0_CCIS: 12..13 = enum TA1CCTL0_CCIS {
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
        TA1CCTL0_CM: 14..15 = enum TA1CCTL0_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL1 @ 0x04: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL1_CCIFG: 0..0 = enum TA1CCTL1_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL1_COV: 1..1 = enum TA1CCTL1_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL1_OUT: 2..2 = enum TA1CCTL1_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL1_CCI: 3 = struct TA1CCTL1_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL1_CCIE: 4..4 = enum TA1CCTL1_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL1_OUTMOD: 5..7 = enum TA1CCTL1_OUTMOD {
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
        TA1CCTL1_CAP: 8..8 = enum TA1CCTL1_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL1_SCCI: 10 = struct TA1CCTL1_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL1_SCS: 11..11 = enum TA1CCTL1_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL1_CCIS: 12..13 = enum TA1CCTL1_CCIS {
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
        TA1CCTL1_CM: 14..15 = enum TA1CCTL1_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL2 @ 0x06: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL2_CCIFG: 0..0 = enum TA1CCTL2_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL2_COV: 1..1 = enum TA1CCTL2_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL2_OUT: 2..2 = enum TA1CCTL2_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL2_CCI: 3 = struct TA1CCTL2_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL2_CCIE: 4..4 = enum TA1CCTL2_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL2_OUTMOD: 5..7 = enum TA1CCTL2_OUTMOD {
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
        TA1CCTL2_CAP: 8..8 = enum TA1CCTL2_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL2_SCCI: 10 = struct TA1CCTL2_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL2_SCS: 11..11 = enum TA1CCTL2_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL2_CCIS: 12..13 = enum TA1CCTL2_CCIS {
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
        TA1CCTL2_CM: 14..15 = enum TA1CCTL2_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL3 @ 0x08: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL3_CCIFG: 0..0 = enum TA1CCTL3_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL3_COV: 1..1 = enum TA1CCTL3_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL3_OUT: 2..2 = enum TA1CCTL3_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL3_CCI: 3 = struct TA1CCTL3_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL3_CCIE: 4..4 = enum TA1CCTL3_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL3_OUTMOD: 5..7 = enum TA1CCTL3_OUTMOD {
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
        TA1CCTL3_CAP: 8..8 = enum TA1CCTL3_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL3_SCCI: 10 = struct TA1CCTL3_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL3_SCS: 11..11 = enum TA1CCTL3_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL3_CCIS: 12..13 = enum TA1CCTL3_CCIS {
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
        TA1CCTL3_CM: 14..15 = enum TA1CCTL3_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL4 @ 0x0a: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL4_CCIFG: 0..0 = enum TA1CCTL4_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL4_COV: 1..1 = enum TA1CCTL4_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL4_OUT: 2..2 = enum TA1CCTL4_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL4_CCI: 3 = struct TA1CCTL4_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL4_CCIE: 4..4 = enum TA1CCTL4_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL4_OUTMOD: 5..7 = enum TA1CCTL4_OUTMOD {
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
        TA1CCTL4_CAP: 8..8 = enum TA1CCTL4_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL4_SCCI: 10 = struct TA1CCTL4_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL4_SCS: 11..11 = enum TA1CCTL4_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL4_CCIS: 12..13 = enum TA1CCTL4_CCIS {
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
        TA1CCTL4_CM: 14..15 = enum TA1CCTL4_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL5 @ 0x0c: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL5_CCIFG: 0..0 = enum TA1CCTL5_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL5_COV: 1..1 = enum TA1CCTL5_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL5_OUT: 2..2 = enum TA1CCTL5_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL5_CCI: 3 = struct TA1CCTL5_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL5_CCIE: 4..4 = enum TA1CCTL5_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL5_OUTMOD: 5..7 = enum TA1CCTL5_OUTMOD {
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
        TA1CCTL5_CAP: 8..8 = enum TA1CCTL5_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL5_SCCI: 10 = struct TA1CCTL5_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL5_SCS: 11..11 = enum TA1CCTL5_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL5_CCIS: 12..13 = enum TA1CCTL5_CCIS {
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
        TA1CCTL5_CM: 14..15 = enum TA1CCTL5_CM {
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
    /// Timer_A Capture/Compare Control Register
    rw TA1CCTL6 @ 0x0e: u16 = 0_0 {
        /// Capture/compare interrupt flag
        TA1CCTL6_CCIFG: 0..0 = enum TA1CCTL6_CCIFG {
            /// No interrupt pending
            CCIFG_0 = 0b0,
            /// Interrupt pending
            CCIFG_1 = 0b1,
        }
        /// Capture overflow
        TA1CCTL6_COV: 1..1 = enum TA1CCTL6_COV {
            /// No capture overflow occurred
            COV_0 = 0b0,
            /// Capture overflow occurred
            COV_1 = 0b1,
        }
        /// Output
        TA1CCTL6_OUT: 2..2 = enum TA1CCTL6_OUT {
            /// Output low
            LOW = 0b0,
            /// Output high
            HIGH = 0b1,
        }
        /// Capture/compare input
        TA1CCTL6_CCI: 3 = struct TA1CCTL6_CCI(bool);
        /// Capture/compare interrupt enable
        TA1CCTL6_CCIE: 4..4 = enum TA1CCTL6_CCIE {
            /// Interrupt disabled
            CCIE_0 = 0b0,
            /// Interrupt enabled
            CCIE_1 = 0b1,
        }
        /// Output mode
        TA1CCTL6_OUTMOD: 5..7 = enum TA1CCTL6_OUTMOD {
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
        TA1CCTL6_CAP: 8..8 = enum TA1CCTL6_CAP {
            /// Compare mode
            COMPARE = 0b0,
            /// Capture mode
            CAPTURE = 0b1,
        }
        /// Synchronized capture/compare input
        TA1CCTL6_SCCI: 10 = struct TA1CCTL6_SCCI(bool);
        /// Synchronize capture source
        TA1CCTL6_SCS: 11..11 = enum TA1CCTL6_SCS {
            /// Asynchronous capture
            ASYNC = 0b0,
            /// Synchronous capture
            SYNC = 0b1,
        }
        /// Capture/compare input select
        TA1CCTL6_CCIS: 12..13 = enum TA1CCTL6_CCIS {
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
        TA1CCTL6_CM: 14..15 = enum TA1CCTL6_CM {
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
    /// TimerA register
    rw TA1R @ 0x10: u16 = 0_0 {
        /// TimerA register
        TA1R: 0..15 = struct TA1RField(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR0 @ 0x12: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR0: 0..15 = struct TA1CCR0Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR1 @ 0x14: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR1: 0..15 = struct TA1CCR1Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR2 @ 0x16: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR2: 0..15 = struct TA1CCR2Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR3 @ 0x18: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR3: 0..15 = struct TA1CCR3Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR4 @ 0x1a: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR4: 0..15 = struct TA1CCR4Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR5 @ 0x1c: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR5: 0..15 = struct TA1CCR5Field(u16);
    }
    /// Timer_A Capture/Compare  Register
    rw TA1CCR6 @ 0x1e: u16 = 0_0 {
        /// Timer_A Capture/Compare  Register
        TA1CCR6: 0..15 = struct TA1CCR6Field(u16);
    }
    /// TimerAx Expansion 0 Register
    rw TA1EX0 @ 0x20: u16 = 0_0 {
        /// Input divider expansion
        TAIDEX: 0..2 = enum TAIDEX {
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
    /// TimerAx Interrupt Vector Register
    r TA1IV @ 0x2e: u16 = 0_0 {
        /// TimerA interrupt vector value
        TAIV: 0..15 = enum TAIV {
            /// No interrupt pending
            NONE = 0b0000000000000000,
            /// Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest
            TACCR1 = 0b0000000000000010,
            /// Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG
            TACCR2 = 0b0000000000000100,
            /// Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG
            TACCR3 = 0b0000000000000110,
            /// Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG
            TACCR4 = 0b0000000000001000,
            /// Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG
            TACCR5 = 0b0000000000001010,
            /// Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG
            TACCR6 = 0b0000000000001100,
            /// Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest
            TAIFG = 0b0000000000001110,
        }
    }
}