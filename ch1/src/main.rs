#[derive(Debug,PartialEq,Eq)]
struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(n: u32, p: u32) -> FieldElement {
        FieldElement {
            num: n,
            prime: p,
        }
    }
}

fn main() {
    let fe = FieldElement::new(4, 5);
    let fe_2 = FieldElement::new(4, 5);
    println!("{}", fe.eq(&fe_2));
}
