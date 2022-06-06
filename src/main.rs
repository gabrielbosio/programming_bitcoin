mod field_element;

use field_element::FieldElement;
use num_bigint::ToBigInt;

fn main() {
    let a = FieldElement::new(2.to_bigint().unwrap(), 19.to_bigint().unwrap());
    let b = FieldElement::new(7.to_bigint().unwrap(), 19.to_bigint().unwrap());
    let c = FieldElement::new(3.to_bigint().unwrap(), 19.to_bigint().unwrap());
    println!("{}", a / b == c);
}
