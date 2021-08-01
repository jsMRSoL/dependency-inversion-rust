struct LightBulb ();
impl LightBulb {
    fn turn_on(&self) {
        println!("Lightbulb: turned on...")
    }

    fn turn_off(&self) {
        println!("Lightbulb: turned off...")
    }
}

struct ElectricPowerSwitch {
    lightbulb: LightBulb,
    on: bool,
}

impl ElectricPowerSwitch {
    fn new(l: LightBulb) -> Self {
	Self {
	    lightbulb: l,
	    on: false,
	}
    }

    fn press(&mut self) {
	if self.on {
	    self.lightbulb.turn_off();
	    self.on = false;
	} else {
	    self.lightbulb.turn_on();
	    self.on = true;
	}
	
    }
}

fn main() {
    let l = LightBulb();
    let mut switch = ElectricPowerSwitch::new(l);
    switch.press();
    switch.press();
}
