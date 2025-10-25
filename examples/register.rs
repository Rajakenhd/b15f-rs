use std::thread::sleep;
use std::time::Duration;
use b15f::b15f::B15F;
use b15f::atmega1284p;
fn main() -> Result<(), b15f::B15F::Error> {
    let mut drv = B15F::new()?;
    let mut inputs = drv.get_register(atmega1284p::DataRegister::DDRB)?;
    inputs |= 1<<0;
    drv.set_register(atmega1284p::DataRegister::DDRB, inputs)?;

    let mut tmp = drv.get_register(atmega1284p::DataRegister::PortB)?;
    drv.set_register(atmega1284p::DataRegister::PortB, tmp|(1<<0))?;

    sleep(Duration::from_millis(300));

    tmp = drv.get_register(atmega1284p::DataRegister::PortB)?;
    tmp &= !1;
    Ok(drv.set_register(atmega1284p::DataRegister::PortB, tmp&!1)?)
}