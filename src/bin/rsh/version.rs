
macro_rules! def_version {
    ( $major:literal, $minor:literal, $patch:literal) => {
        #[allow(dead_code)]
        pub const VERSION_MAJOR: u8 = $major;
        #[allow(dead_code)]
        pub const VERSION_MINOR: u8 = $minor;
        #[allow(dead_code)]
        pub const VERSION_PATCH: u8 = $patch;
        #[allow(dead_code)]
        pub const VERSION_STR: &'static str = concat!($major,".",$minor,".",$patch);
    };
}
def_version!(0,0,0);

