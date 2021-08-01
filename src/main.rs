trait Switchable {
    fn turn_on(&self) {}

    fn turn_off(&self) {}
}

struct LightBulb();
impl Switchable for LightBulb {
    fn turn_on(&self) {
        println!("Lightbulb: turned on...")
    }

    fn turn_off(&self) {
        println!("Lightbulb: turned off...")
    }
}

struct Fan();
impl Switchable for Fan {
    fn turn_on(&self) {
        println!("Fan: turned on...")
    }

    fn turn_off(&self) {
        println!("Fan: turned off...")
    }
}

struct ElectricPowerSwitch<T> {
    client: T,
    on: bool,
}

impl<T: Switchable> ElectricPowerSwitch<T> {
    fn new(c: T) -> Self {
        Self {
            client: c,
            on: false,
        }
    }

    fn press(&mut self) {
        if self.on {
            self.client.turn_off();
            self.on = false;
        } else {
            self.client.turn_on();
            self.on = true;
        }
    }
}

fn main() {
    let l = LightBulb();
    let f = Fan();
    let mut switch1 = ElectricPowerSwitch::new(l);
    let mut switch2 = ElectricPowerSwitch::new(f);
    switch1.press();
    switch2.press();
    switch1.press();
    switch2.press();
}
