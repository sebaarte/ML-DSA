pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> std::fmt::Binary for Bytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for byte in self.0 {
            std::fmt::Binary::fmt(byte, f)?;
            write!(f, ",")?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}
