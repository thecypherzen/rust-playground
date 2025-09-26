/*
 * Implements a temperature data structure.
 * Default Temperature scale on instatiation is Kelvin.
 * Other scales can be supplied during createion like Celcius and
 * Fahrenheit
 */
use std::fmt::*;
use crate::TempScale::*;

enum TempScale {
	Celcius, Fahrenheit, Kelvin
}

struct Temperature {
	kel: f64,
	cel: f64,
	fah: f64
}

impl Display for Temperature {
	fn fmt(&self, buffer: &mut Formatter) -> Result {
		write!(buffer, "{}°K", self.kel)
	}
}

impl Temperature {
	fn ctof(ctemp: f64) -> f64 {
		Self::to_precision((ctemp * 1.8) + 32.0, 2)
	}

	fn ctok(ctemp: f64) -> f64 {
		Self::to_precision(ctemp + 273.15, 2)
	}

	fn ftoc(ftemp: f64) -> f64 {
		Self::to_precision((ftemp - 32.0) / 1.8, 2)
	}

	fn ftok(ftemp: f64) -> f64 {
		Self::ctok(Self::ftoc(ftemp))
	}

	fn ktoc(ktemp: f64) -> f64 {
		Self::to_precision(ktemp - 273.15, 2)
	}

	fn ktof(ktemp: f64) -> f64 {
		Self::ctof(Self::ktoc(ktemp))
	}

	fn new(value: f64) -> Temperature {
		Temperature::new_from(value, Celcius)
	}

	fn new_from(value: f64, scale: TempScale) -> Temperature {
		match scale {
			Celcius => Temperature {
				cel: Self::to_precision(value, 2),
				fah: Self::ctof(value),
				kel: Self::ctok(value),
			},
			Fahrenheit => Temperature {
				cel: Self::ftoc(value),
				fah: Self::to_precision(value, 2),
				kel: Self::ftok(value)
			},
			_ => Temperature {
				cel: Self::ktoc(value),
				fah: Self::ktof(value),
				kel: Self::to_precision(value, 2)
			}
		}
	}

	fn print(&self, scale: TempScale) {
		match scale {
			Celcius => println!("{}°C", self.cel),
			Fahrenheit => println!("{}°F", self.fah),
			_ => println!("{}°K", self.kel),
		}
	}

	fn to_precision(num: f64, prec: i32) -> f64 {
		let factor = 10f64.powi(prec);
		(num * factor).round() / factor
	}
}

fn main() {
	let temp = Temperature::new(272.21);
	let temp2 = Temperature::new_from(127.96, Kelvin);
	let temp3 = Temperature::new_from(123.22, Fahrenheit);
	temp.print(Celcius);
	temp2.print(Kelvin);
	temp3.print(Fahrenheit);
}
