//! SAC2

utils::periph! {
    /// SAC2
    SAC2;
    /// SAC OA Control Register
    rw SAC2OA @ 0x00: u16 = 0_0 {
        /// SAC OA Positive input source selection
        PSEL: 0..1 = enum PSEL {
            /// External source selected
            PSEL_0 = 0b00,
            /// 12-bit reference DAC source selected
            PSEL_1 = 0b01,
            /// Pair OA source selected
            PSEL_2 = 0b10,
        }
        /// SAC Positive input MUX control.
        PMUXEN: 3..3 = enum PMUXEN {
            /// All positive input sources are disconnected to OA positive port
            PMUXEN_0 = 0b0,
            /// All positive input sources are connected to OA positive port
            PMUXEN_1 = 0b1,
        }
        /// SAC OA Negative input source selection
        NSEL: 4..5 = enum NSEL {
            /// External source selected
            NSEL_0 = 0b00,
            /// PGA source selected
            NSEL_1 = 0b01,
            /// Device Specific
            NSEL_2 = 0b10,
        }
        /// SAC Negative input MUX controL
        NMUXEN: 7..7 = enum NMUXEN {
            /// All negative input sources are disconnected to OA negative port
            NMUXEN_0 = 0b0,
            /// All negative input sources are connected to OA negative port
            NMUXEN_1 = 0b1,
        }
        /// SAC OA Enable selection
        OAEN: 8..8 = enum OAEN {
            /// SAC OA is disabled, then the SAC OA output high impedance
            OAEN_0 = 0b0,
            /// SAC OA is enabled, normal mode
            OAEN_1 = 0b1,
        }
        /// SAC OA power mode selection
        OAPM: 9..9 = enum OAPM {
            /// High speed and high power
            OAPM_0 = 0b0,
            /// Llow speed and low power
            OAPM_1 = 0b1,
        }
        /// SAC Enable selection
        SACEN: 10..10 = enum SACEN {
            /// SAC all modules are disabled, then the SAC output high impedance
            SACEN_0 = 0b0,
            /// SAC all modules are enabled, normal mode
            SACEN_1 = 0b1,
        }
    }
    /// SAC PGA Control Register
    rw SAC2PGA @ 0x02: u16 = 0_0 {
        /// SAC PGA Mode Selection
        MSEL: 0..1 = enum MSEL {
            /// Inverting PGA mode (external pad IN- is selected)
            MSEL_0 = 0b00,
            /// Buffer mode (floating is selected )
            MSEL_1 = 0b01,
            /// Non-inverting mode
            MSEL_2 = 0b10,
            /// Cascade OA Inverting mode
            MSEL_3 = 0b11,
        }
        /// SAC PGA Gain configuration
        GAIN: 4..6 = struct GAIN(u16);
    }
    /// SAC DAC Control Register
    rw SAC2DAC @ 0x04: u16 = 0_0 {
        /// SAC DAC enable
        DACEN: 0..0 = enum DACEN {
            /// Disabled
            DACEN_0 = 0b0,
            /// Enabled
            DACEN_1 = 0b1,
        }
        /// SAC DAC interrupt enable
        DACIE: 1..1 = enum DACIE {
            /// Disabled
            DACIE_0 = 0b0,
            /// Enabled
            DACIE_1 = 0b1,
        }
        /// SAC DAC DMA request enable
        DACDMAE: 2..2 = enum DACDMAE {
            /// DMA request disabled
            DACDMAE_0 = 0b0,
            /// DMA request enabled
            DACDMAE_1 = 0b1,
        }
        /// SAC DAC load select. Selects the load trigger for the DAC latch.
        DACLSEL: 8..9 = enum DACLSEL {
            /// DAC latch loads when DACDAT written
            DACLSEL_0 = 0b00,
            /// Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal
            DACLSEL_2 = 0b10,
            /// Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal
            DACLSEL_3 = 0b11,
        }
        /// SAC DAC select reference voltage
        DACSREF: 12..12 = enum DACSREF {
            /// AVCC
            DACSREF_0 = 0b0,
            /// Alternative reference
            DACSREF_1 = 0b1,
        }
    }
    /// SAC DAC Data Register
    rw SAC2DAT @ 0x06: u16 = 0_0 {
        /// SAC DAC data in unsigned format.
        DACData: 0..11 = struct DACData(u16);
    }
    /// SAC DAC Status Register
    rw SAC2DACSTS @ 0x08: u16 = 0_0 {
        /// SAC DAC data update flag
        DACIFG: 0 = struct DACIFG(bool);
    }
    /// SAC Interrupt Vector Register
    r SAC2IV @ 0x0a: u16 = 0_0 {
        /// SAC Interrupt Vector Register
        SACIV2: 0..15 = enum SACIV2 {
            /// No interrupt pending
            SACIV_0 = 0b0000000000000000,
            /// S&H completed interrupt flag (Highest priority)
            SACIV_2 = 0b0000000000000010,
            /// DAC channel update interrupt flag
            SACIV_4 = 0b0000000000000100,
        }
    }
}