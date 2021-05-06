//! Port 7

utils::periph! {
    /// Port 7
    Port7;
    /// Port 7 Input
    rw P7IN @ 0x00: u8 = 0_0 {
        /// P7IN0
        P7IN0: 0 = struct P7IN0(bool);
        /// P7IN1
        P7IN1: 1 = struct P7IN1(bool);
        /// P7IN2
        P7IN2: 2 = struct P7IN2(bool);
        /// P7IN3
        P7IN3: 3 = struct P7IN3(bool);
        /// P7IN4
        P7IN4: 4 = struct P7IN4(bool);
        /// P7IN5
        P7IN5: 5 = struct P7IN5(bool);
        /// P7IN6
        P7IN6: 6 = struct P7IN6(bool);
        /// P7IN7
        P7IN7: 7 = struct P7IN7(bool);
    }
    /// Port 7 Output
    rw P7OUT @ 0x02: u8 = 0_0 {
        /// P7OUT0
        P7OUT0: 0 = struct P7OUT0(bool);
        /// P7OUT1
        P7OUT1: 1 = struct P7OUT1(bool);
        /// P7OUT2
        P7OUT2: 2 = struct P7OUT2(bool);
        /// P7OUT3
        P7OUT3: 3 = struct P7OUT3(bool);
        /// P7OUT4
        P7OUT4: 4 = struct P7OUT4(bool);
        /// P7OUT5
        P7OUT5: 5 = struct P7OUT5(bool);
        /// P7OUT6
        P7OUT6: 6 = struct P7OUT6(bool);
        /// P7OUT7
        P7OUT7: 7 = struct P7OUT7(bool);
    }
    /// Port 7 Direction
    rw P7DIR @ 0x04: u8 = 0_0 {
        /// P7DIR0
        P7DIR0: 0 = struct P7DIR0(bool);
        /// P7DIR1
        P7DIR1: 1 = struct P7DIR1(bool);
        /// P7DIR2
        P7DIR2: 2 = struct P7DIR2(bool);
        /// P7DIR3
        P7DIR3: 3 = struct P7DIR3(bool);
        /// P7DIR4
        P7DIR4: 4 = struct P7DIR4(bool);
        /// P7DIR5
        P7DIR5: 5 = struct P7DIR5(bool);
        /// P7DIR6
        P7DIR6: 6 = struct P7DIR6(bool);
        /// P7DIR7
        P7DIR7: 7 = struct P7DIR7(bool);
    }
    /// Port 7 Resistor Enable
    rw P7REN @ 0x06: u8 = 0_0 {
        /// P7REN0
        P7REN0: 0 = struct P7REN0(bool);
        /// P7REN1
        P7REN1: 1 = struct P7REN1(bool);
        /// P7REN2
        P7REN2: 2 = struct P7REN2(bool);
        /// P7REN3
        P7REN3: 3 = struct P7REN3(bool);
        /// P7REN4
        P7REN4: 4 = struct P7REN4(bool);
        /// P7REN5
        P7REN5: 5 = struct P7REN5(bool);
        /// P7REN6
        P7REN6: 6 = struct P7REN6(bool);
        /// P7REN7
        P7REN7: 7 = struct P7REN7(bool);
    }
    /// Port 7 Drive Strenght
    rw P7DS @ 0x08: u8 = 0_0 {
        /// P7DS0
        P7DS0: 0 = struct P7DS0(bool);
        /// P7DS1
        P7DS1: 1 = struct P7DS1(bool);
        /// P7DS2
        P7DS2: 2 = struct P7DS2(bool);
        /// P7DS3
        P7DS3: 3 = struct P7DS3(bool);
        /// P7DS4
        P7DS4: 4 = struct P7DS4(bool);
        /// P7DS5
        P7DS5: 5 = struct P7DS5(bool);
        /// P7DS6
        P7DS6: 6 = struct P7DS6(bool);
        /// P7DS7
        P7DS7: 7 = struct P7DS7(bool);
    }
    /// Port 7 Selection
    rw P7SEL @ 0x0a: u8 = 0_0 {
        /// P7SEL0
        P7SEL0: 0 = struct P7SEL0(bool);
        /// P7SEL1
        P7SEL1: 1 = struct P7SEL1(bool);
        /// P7SEL2
        P7SEL2: 2 = struct P7SEL2(bool);
        /// P7SEL3
        P7SEL3: 3 = struct P7SEL3(bool);
        /// P7SEL4
        P7SEL4: 4 = struct P7SEL4(bool);
        /// P7SEL5
        P7SEL5: 5 = struct P7SEL5(bool);
        /// P7SEL6
        P7SEL6: 6 = struct P7SEL6(bool);
        /// P7SEL7
        P7SEL7: 7 = struct P7SEL7(bool);
    }
}