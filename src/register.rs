pub struct Register<'a> {
    pub num: u32,
    pub register_name: &'a str,
    pub chip: super::z80::Z80<'a>
}

impl<'a> Register<'a> {
    pub fn new(num: u32, name: &'a str) -> Register {
        Register {
            num: num,
            register_name: name,
            chip: super::z80::Z80::new(name)
        }
    } 
}

