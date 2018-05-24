pub struct Z80<'a> {
    pub name: &'a str
}

impl<'a> Z80<'a> {
    pub fn new(name: &'a str) -> Z80 {
        Z80 {
            name: name
        }
    }
}

