mod field_element;

use field_element::FieldElement;
use num_bigint::ToBigInt;

fn main() {
    let a = FieldElement::new(3.to_bigint().unwrap(), 13.to_bigint().unwrap());
    let b = FieldElement::new(1.to_bigint().unwrap(), 13.to_bigint().unwrap());
    println!("{}", a.pow(3.to_bigint().unwrap()) == b);
}
