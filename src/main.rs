mod field_element;

use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(10, 13);
    println!("{}", a * b == c);
}
