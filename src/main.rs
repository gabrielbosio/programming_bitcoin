mod field_element;

use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(1, 13);
    println!("{}", a.pow(3) == b);
}
