//! Port E

utils::periph! {
    /// Port E
    PortE;
    /// Port E Input
    rw PEIN @ 0x00: u16 = 0_0 {
        /// PEIN0
        PEIN0: 0 = struct PEIN0(bool);
        /// PEIN1
        PEIN1: 1 = struct PEIN1(bool);
        /// PEIN2
        PEIN2: 2 = struct PEIN2(bool);
        /// PEIN3
        PEIN3: 3 = struct PEIN3(bool);
        /// PEIN4
        PEIN4: 4 = struct PEIN4(bool);
        /// PEIN5
        PEIN5: 5 = struct PEIN5(bool);
        /// PEIN6
        PEIN6: 6 = struct PEIN6(bool);
        /// PEIN7
        PEIN7: 7 = struct PEIN7(bool);
        /// PEIN8
        PEIN8: 8 = struct PEIN8(bool);
        /// PEIN9
        PEIN9: 9 = struct PEIN9(bool);
        /// PEIN10
        PEIN10: 10 = struct PEIN10(bool);
        /// PEIN11
        PEIN11: 11 = struct PEIN11(bool);
        /// PEIN12
        PEIN12: 12 = struct PEIN12(bool);
        /// PEIN13
        PEIN13: 13 = struct PEIN13(bool);
        /// PEIN14
        PEIN14: 14 = struct PEIN14(bool);
        /// PEIN15
        PEIN15: 15 = struct PEIN15(bool);
    }
    /// Port E Output
    rw PEOUT @ 0x02: u16 = 0_0 {
        /// PEOUT0
        PEOUT0: 0 = struct PEOUT0(bool);
        /// PEOUT1
        PEOUT1: 1 = struct PEOUT1(bool);
        /// PEOUT2
        PEOUT2: 2 = struct PEOUT2(bool);
        /// PEOUT3
        PEOUT3: 3 = struct PEOUT3(bool);
        /// PEOUT4
        PEOUT4: 4 = struct PEOUT4(bool);
        /// PEOUT5
        PEOUT5: 5 = struct PEOUT5(bool);
        /// PEOUT6
        PEOUT6: 6 = struct PEOUT6(bool);
        /// PEOUT7
        PEOUT7: 7 = struct PEOUT7(bool);
        /// PEOUT8
        PEOUT8: 8 = struct PEOUT8(bool);
        /// PEOUT9
        PEOUT9: 9 = struct PEOUT9(bool);
        /// PEOUT10
        PEOUT10: 10 = struct PEOUT10(bool);
        /// PEOUT11
        PEOUT11: 11 = struct PEOUT11(bool);
        /// PEOUT12
        PEOUT12: 12 = struct PEOUT12(bool);
        /// PEOUT13
        PEOUT13: 13 = struct PEOUT13(bool);
        /// PEOUT14
        PEOUT14: 14 = struct PEOUT14(bool);
        /// PEOUT15
        PEOUT15: 15 = struct PEOUT15(bool);
    }
    /// Port E Direction
    rw PEDIR @ 0x04: u16 = 0_0 {
        /// PEDIR0
        PEDIR0: 0 = struct PEDIR0(bool);
        /// PEDIR1
        PEDIR1: 1 = struct PEDIR1(bool);
        /// PEDIR2
        PEDIR2: 2 = struct PEDIR2(bool);
        /// PEDIR3
        PEDIR3: 3 = struct PEDIR3(bool);
        /// PEDIR4
        PEDIR4: 4 = struct PEDIR4(bool);
        /// PEDIR5
        PEDIR5: 5 = struct PEDIR5(bool);
        /// PEDIR6
        PEDIR6: 6 = struct PEDIR6(bool);
        /// PEDIR7
        PEDIR7: 7 = struct PEDIR7(bool);
        /// PEDIR8
        PEDIR8: 8 = struct PEDIR8(bool);
        /// PEDIR9
        PEDIR9: 9 = struct PEDIR9(bool);
        /// PEDIR10
        PEDIR10: 10 = struct PEDIR10(bool);
        /// PEDIR11
        PEDIR11: 11 = struct PEDIR11(bool);
        /// PEDIR12
        PEDIR12: 12 = struct PEDIR12(bool);
        /// PEDIR13
        PEDIR13: 13 = struct PEDIR13(bool);
        /// PEDIR14
        PEDIR14: 14 = struct PEDIR14(bool);
        /// PEDIR15
        PEDIR15: 15 = struct PEDIR15(bool);
    }
    /// Port E Resistor Enable
    rw PEREN @ 0x06: u16 = 0_0 {
        /// PEREN0
        PEREN0: 0 = struct PEREN0(bool);
        /// PEREN1
        PEREN1: 1 = struct PEREN1(bool);
        /// PEREN2
        PEREN2: 2 = struct PEREN2(bool);
        /// PEREN3
        PEREN3: 3 = struct PEREN3(bool);
        /// PEREN4
        PEREN4: 4 = struct PEREN4(bool);
        /// PEREN5
        PEREN5: 5 = struct PEREN5(bool);
        /// PEREN6
        PEREN6: 6 = struct PEREN6(bool);
        /// PEREN7
        PEREN7: 7 = struct PEREN7(bool);
        /// PEREN8
        PEREN8: 8 = struct PEREN8(bool);
        /// PEREN9
        PEREN9: 9 = struct PEREN9(bool);
        /// PEREN10
        PEREN10: 10 = struct PEREN10(bool);
        /// PEREN11
        PEREN11: 11 = struct PEREN11(bool);
        /// PEREN12
        PEREN12: 12 = struct PEREN12(bool);
        /// PEREN13
        PEREN13: 13 = struct PEREN13(bool);
        /// PEREN14
        PEREN14: 14 = struct PEREN14(bool);
        /// PEREN15
        PEREN15: 15 = struct PEREN15(bool);
    }
    /// Port E Drive Strenght
    rw PEDS @ 0x08: u16 = 0_0 {
        /// PEDS0
        PEDS0: 0 = struct PEDS0(bool);
        /// PEDS1
        PEDS1: 1 = struct PEDS1(bool);
        /// PEDS2
        PEDS2: 2 = struct PEDS2(bool);
        /// PEDS3
        PEDS3: 3 = struct PEDS3(bool);
        /// PEDS4
        PEDS4: 4 = struct PEDS4(bool);
        /// PEDS5
        PEDS5: 5 = struct PEDS5(bool);
        /// PEDS6
        PEDS6: 6 = struct PEDS6(bool);
        /// PEDS7
        PEDS7: 7 = struct PEDS7(bool);
        /// PEDS8
        PEDS8: 8 = struct PEDS8(bool);
        /// PEDS9
        PEDS9: 9 = struct PEDS9(bool);
        /// PEDS10
        PEDS10: 10 = struct PEDS10(bool);
        /// PEDS11
        PEDS11: 11 = struct PEDS11(bool);
        /// PEDS12
        PEDS12: 12 = struct PEDS12(bool);
        /// PEDS13
        PEDS13: 13 = struct PEDS13(bool);
        /// PEDS14
        PEDS14: 14 = struct PEDS14(bool);
        /// PEDS15
        PEDS15: 15 = struct PEDS15(bool);
    }
    /// Port E Selection
    rw PESEL @ 0x0a: u16 = 0_0 {
        /// PESEL0
        PESEL0: 0 = struct PESEL0(bool);
        /// PESEL1
        PESEL1: 1 = struct PESEL1(bool);
        /// PESEL2
        PESEL2: 2 = struct PESEL2(bool);
        /// PESEL3
        PESEL3: 3 = struct PESEL3(bool);
        /// PESEL4
        PESEL4: 4 = struct PESEL4(bool);
        /// PESEL5
        PESEL5: 5 = struct PESEL5(bool);
        /// PESEL6
        PESEL6: 6 = struct PESEL6(bool);
        /// PESEL7
        PESEL7: 7 = struct PESEL7(bool);
        /// PESEL8
        PESEL8: 8 = struct PESEL8(bool);
        /// PESEL9
        PESEL9: 9 = struct PESEL9(bool);
        /// PESEL10
        PESEL10: 10 = struct PESEL10(bool);
        /// PESEL11
        PESEL11: 11 = struct PESEL11(bool);
        /// PESEL12
        PESEL12: 12 = struct PESEL12(bool);
        /// PESEL13
        PESEL13: 13 = struct PESEL13(bool);
        /// PESEL14
        PESEL14: 14 = struct PESEL14(bool);
        /// PESEL15
        PESEL15: 15 = struct PESEL15(bool);
    }
}
