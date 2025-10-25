#[derive(Debug)]
#[repr(u8)]
pub enum DataRegister {
    DDRA    = 0x21,
    PortA   = 0x22,
    DDRB    = 0x24,
    PortB   = 0x25,
    DDRC    = 0x27,
    PortC   = 0x28,
    DDRD    = 0x2a,
    PortD   = 0x2b,
}