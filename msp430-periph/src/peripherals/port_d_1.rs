//! Port D

utils::periph! {
    /// Port D
    PortD;
    /// Port D Input
    rw PDIN @ 0x00: u16 = 0_0 {
        /// PDIN0
        PDIN0: 0 = struct PDIN0(bool);
        /// PDIN1
        PDIN1: 1 = struct PDIN1(bool);
        /// PDIN2
        PDIN2: 2 = struct PDIN2(bool);
        /// PDIN3
        PDIN3: 3 = struct PDIN3(bool);
        /// PDIN4
        PDIN4: 4 = struct PDIN4(bool);
        /// PDIN5
        PDIN5: 5 = struct PDIN5(bool);
        /// PDIN6
        PDIN6: 6 = struct PDIN6(bool);
        /// PDIN7
        PDIN7: 7 = struct PDIN7(bool);
        /// PDIN8
        PDIN8: 8 = struct PDIN8(bool);
        /// PDIN9
        PDIN9: 9 = struct PDIN9(bool);
        /// PDIN10
        PDIN10: 10 = struct PDIN10(bool);
        /// PDIN11
        PDIN11: 11 = struct PDIN11(bool);
        /// PDIN12
        PDIN12: 12 = struct PDIN12(bool);
        /// PDIN13
        PDIN13: 13 = struct PDIN13(bool);
        /// PDIN14
        PDIN14: 14 = struct PDIN14(bool);
        /// PDIN15
        PDIN15: 15 = struct PDIN15(bool);
    }
    /// Port D Output
    rw PDOUT @ 0x02: u16 = 0_0 {
        /// PDOUT0
        PDOUT0: 0 = struct PDOUT0(bool);
        /// PDOUT1
        PDOUT1: 1 = struct PDOUT1(bool);
        /// PDOUT2
        PDOUT2: 2 = struct PDOUT2(bool);
        /// PDOUT3
        PDOUT3: 3 = struct PDOUT3(bool);
        /// PDOUT4
        PDOUT4: 4 = struct PDOUT4(bool);
        /// PDOUT5
        PDOUT5: 5 = struct PDOUT5(bool);
        /// PDOUT6
        PDOUT6: 6 = struct PDOUT6(bool);
        /// PDOUT7
        PDOUT7: 7 = struct PDOUT7(bool);
        /// PDOUT8
        PDOUT8: 8 = struct PDOUT8(bool);
        /// PDOUT9
        PDOUT9: 9 = struct PDOUT9(bool);
        /// PDOUT10
        PDOUT10: 10 = struct PDOUT10(bool);
        /// PDOUT11
        PDOUT11: 11 = struct PDOUT11(bool);
        /// PDOUT12
        PDOUT12: 12 = struct PDOUT12(bool);
        /// PDOUT13
        PDOUT13: 13 = struct PDOUT13(bool);
        /// PDOUT14
        PDOUT14: 14 = struct PDOUT14(bool);
        /// PDOUT15
        PDOUT15: 15 = struct PDOUT15(bool);
    }
    /// Port D Direction
    rw PDDIR @ 0x04: u16 = 0_0 {
        /// PDDIR0
        PDDIR0: 0 = struct PDDIR0(bool);
        /// PDDIR1
        PDDIR1: 1 = struct PDDIR1(bool);
        /// PDDIR2
        PDDIR2: 2 = struct PDDIR2(bool);
        /// PDDIR3
        PDDIR3: 3 = struct PDDIR3(bool);
        /// PDDIR4
        PDDIR4: 4 = struct PDDIR4(bool);
        /// PDDIR5
        PDDIR5: 5 = struct PDDIR5(bool);
        /// PDDIR6
        PDDIR6: 6 = struct PDDIR6(bool);
        /// PDDIR7
        PDDIR7: 7 = struct PDDIR7(bool);
        /// PDDIR8
        PDDIR8: 8 = struct PDDIR8(bool);
        /// PDDIR9
        PDDIR9: 9 = struct PDDIR9(bool);
        /// PDDIR10
        PDDIR10: 10 = struct PDDIR10(bool);
        /// PDDIR11
        PDDIR11: 11 = struct PDDIR11(bool);
        /// PDDIR12
        PDDIR12: 12 = struct PDDIR12(bool);
        /// PDDIR13
        PDDIR13: 13 = struct PDDIR13(bool);
        /// PDDIR14
        PDDIR14: 14 = struct PDDIR14(bool);
        /// PDDIR15
        PDDIR15: 15 = struct PDDIR15(bool);
    }
    /// Port D Resistor Enable
    rw PDREN @ 0x06: u16 = 0_0 {
        /// PDREN0
        PDREN0: 0 = struct PDREN0(bool);
        /// PDREN1
        PDREN1: 1 = struct PDREN1(bool);
        /// PDREN2
        PDREN2: 2 = struct PDREN2(bool);
        /// PDREN3
        PDREN3: 3 = struct PDREN3(bool);
        /// PDREN4
        PDREN4: 4 = struct PDREN4(bool);
        /// PDREN5
        PDREN5: 5 = struct PDREN5(bool);
        /// PDREN6
        PDREN6: 6 = struct PDREN6(bool);
        /// PDREN7
        PDREN7: 7 = struct PDREN7(bool);
        /// PDREN8
        PDREN8: 8 = struct PDREN8(bool);
        /// PDREN9
        PDREN9: 9 = struct PDREN9(bool);
        /// PDREN10
        PDREN10: 10 = struct PDREN10(bool);
        /// PDREN11
        PDREN11: 11 = struct PDREN11(bool);
        /// PDREN12
        PDREN12: 12 = struct PDREN12(bool);
        /// PDREN13
        PDREN13: 13 = struct PDREN13(bool);
        /// PDREN14
        PDREN14: 14 = struct PDREN14(bool);
        /// PDREN15
        PDREN15: 15 = struct PDREN15(bool);
    }
    /// Port D Drive Strenght
    rw PDDS @ 0x08: u16 = 0_0 {
        /// PDDS0
        PDDS0: 0 = struct PDDS0(bool);
        /// PDDS1
        PDDS1: 1 = struct PDDS1(bool);
        /// PDDS2
        PDDS2: 2 = struct PDDS2(bool);
        /// PDDS3
        PDDS3: 3 = struct PDDS3(bool);
        /// PDDS4
        PDDS4: 4 = struct PDDS4(bool);
        /// PDDS5
        PDDS5: 5 = struct PDDS5(bool);
        /// PDDS6
        PDDS6: 6 = struct PDDS6(bool);
        /// PDDS7
        PDDS7: 7 = struct PDDS7(bool);
        /// PDDS8
        PDDS8: 8 = struct PDDS8(bool);
        /// PDDS9
        PDDS9: 9 = struct PDDS9(bool);
        /// PDDS10
        PDDS10: 10 = struct PDDS10(bool);
        /// PDDS11
        PDDS11: 11 = struct PDDS11(bool);
        /// PDDS12
        PDDS12: 12 = struct PDDS12(bool);
        /// PDDS13
        PDDS13: 13 = struct PDDS13(bool);
        /// PDDS14
        PDDS14: 14 = struct PDDS14(bool);
        /// PDDS15
        PDDS15: 15 = struct PDDS15(bool);
    }
    /// Port D Selection
    rw PDSEL @ 0x0a: u16 = 0_0 {
        /// PDSEL0
        PDSEL0: 0 = struct PDSEL0(bool);
        /// PDSEL1
        PDSEL1: 1 = struct PDSEL1(bool);
        /// PDSEL2
        PDSEL2: 2 = struct PDSEL2(bool);
        /// PDSEL3
        PDSEL3: 3 = struct PDSEL3(bool);
        /// PDSEL4
        PDSEL4: 4 = struct PDSEL4(bool);
        /// PDSEL5
        PDSEL5: 5 = struct PDSEL5(bool);
        /// PDSEL6
        PDSEL6: 6 = struct PDSEL6(bool);
        /// PDSEL7
        PDSEL7: 7 = struct PDSEL7(bool);
        /// PDSEL8
        PDSEL8: 8 = struct PDSEL8(bool);
        /// PDSEL9
        PDSEL9: 9 = struct PDSEL9(bool);
        /// PDSEL10
        PDSEL10: 10 = struct PDSEL10(bool);
        /// PDSEL11
        PDSEL11: 11 = struct PDSEL11(bool);
        /// PDSEL12
        PDSEL12: 12 = struct PDSEL12(bool);
        /// PDSEL13
        PDSEL13: 13 = struct PDSEL13(bool);
        /// PDSEL14
        PDSEL14: 14 = struct PDSEL14(bool);
        /// PDSEL15
        PDSEL15: 15 = struct PDSEL15(bool);
    }
}
