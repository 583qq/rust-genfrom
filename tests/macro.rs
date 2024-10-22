

#[cfg(test)]
mod tests {
    use gen_from::GenFrom;

    #[derive(Debug)]
    struct MyStructB {
        v1: u64,
        v2: u64,
        v3: u64,
    }

    #[derive(Debug, GenFrom)]
    #[into(MyStructB)]
    struct MyStructA {
        #[into(name="v1")]
        short_tag: u16,
        #[into(name="v2")]
        value: u8,
        #[into(name="v3")]
        data: u64,
    }

    
    #[derive(Debug)]
    struct SA {
        v1: u64,
        v2: u64,
    }

    #[derive(Debug, GenFrom)]
    #[into(SA)]
    struct SB {
        #[into(name="v1")]
        short_tag: u16,
        v2: u16,
    }

    #[derive(Debug)]
    struct SX {
        v1: u64,
        v2: u64,
    }

    #[derive(Debug, GenFrom)]
    #[into(SX)]
    struct SY {
        #[into(name="v1")]
        short_tag: u16,
        v2: u64,
    }

    #[derive(Debug)]
    struct ST {
        v1: u64,
    }

    #[derive(Debug, GenFrom)]
    #[into(ST)]
    struct SF {
        #[into(name="v1")]
        short_tag: u16,
        #[into(skip)]
        v2: u64,
    }

    #[test]
    fn test_conversion() {
        let my_struct_a = MyStructA {
            short_tag: 5,
            value: 10,
            data: 15,
        };

        let my_struct_b: MyStructB = my_struct_a.into();

        assert_eq!(my_struct_b.v1, 5_u64); // short_tag should convert to v1
        assert_eq!(my_struct_b.v2, 10_u64); // value should convert to v2
        assert_eq!(my_struct_b.v3, 15); // data should convert as-is to v3
    }

    #[test]
    fn test_edge_cases() {
        let my_struct_a = MyStructA {
            short_tag: u16::MAX, // maximum value for short_tag
            value: u8::MAX,      // maximum value for value
            data: u64::MAX,      // maximum value for data
        };

        let my_struct_b: MyStructB = my_struct_a.into();

        assert_eq!(my_struct_b.v1, u64::from(u16::MAX)); // Check conversion
        assert_eq!(my_struct_b.v2, u64::from(u8::MAX)); // Check conversion
        assert_eq!(my_struct_b.v3, u64::MAX); // Check conversion
    }

    #[test]
    fn test_zero_initialization() {
        let my_struct_a = MyStructA {
            short_tag: 0,
            value: 0,
            data: 0,
        };

        let my_struct_b: MyStructB = my_struct_a.into();

        assert_eq!(my_struct_b.v1, 0);
        assert_eq!(my_struct_b.v2, 0);
        assert_eq!(my_struct_b.v3, 0);
    }

    #[test]
    fn test_zero_initialization2() {
        let my_struct_ok = SB {
            short_tag: 0,
            v2: 0,
        };

        let my_struct_o: SA = my_struct_ok.into();

        assert_eq!(my_struct_o.v1, 0);
        assert_eq!(my_struct_o.v2, 0);
    }

    #[test]
    fn test_zero_initialization3() {
        let my_struct_ok = SY {
            short_tag: 0,
            v2: 0,
        };

        let my_struct_o: SX = my_struct_ok.into();

        assert_eq!(my_struct_o.v1, 0);
        assert_eq!(my_struct_o.v2, 0);
    }

    #[test]
    fn test_zero_initialization4() {
        let my_struct_sf = SF {
            short_tag: 0,
            v2: 0,
        };

        let my_struct_st: ST = my_struct_sf.into();

        assert_eq!(my_struct_st.v1, 0);
    }
}