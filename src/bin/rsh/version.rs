

macro_rules! VERSION {
    ( $major:literal, $minor:literal, $patch:literal ) => {
        #[allow(dead_code)]
        pub const MAJOR: u8 = $major;
        #[allow(dead_code)]
        pub const MINOR: u8 = $minor;
        #[allow(dead_code)]
        pub const PATCH: u8 = $patch;
        #[allow(dead_code)]
        pub const STR: &'static str = concat!($major,".",$minor,".",$patch);
    };
}
VERSION!(0,0,0);
