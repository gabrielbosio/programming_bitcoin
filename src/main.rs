mod field_element;

use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);
    println!("{}", a + b == c);
}
