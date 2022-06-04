mod field_element;

use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(7, 13).unwrap();
    let b = FieldElement::new(6, 13).unwrap();
    println!("{}", a == b);
    println!("{}", a == a);
}
