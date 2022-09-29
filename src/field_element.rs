use modulo::Mod;

#[derive(Debug,PartialEq,Eq)]
struct FieldElement {
    num: i128,
    prime: i128,
}

impl FieldElement {
    pub fn new(n: i128, p: i128) -> FieldElement {
        FieldElement {
            num: n,
            prime: p,
        }
    }

    pub fn add(self, other: &FieldElement) -> i128 {
        let num = (self.num + other.num).modulo(self.prime);
        return num;
    }

    pub fn sub(self, other: &FieldElement) -> i128 {
        let num = (self.num - other.num).modulo(self.prime);
        return num;
    }

    pub fn mul(self, other: &FieldElement) -> i128 {
        let num = (self.num * other.num).modulo(self.prime);
        return num;
    }

    pub fn pow(self, exponent: u32) -> i128 {
        let num = self.num.pow(exponent).modulo(self.prime);
        return num;
    }
}

#[cfg(test)]
mod tests {
    use crate::field_element::FieldElement;
    #[test]
    fn field_element_eq() {
        let fe = FieldElement::new(4, 5);
        let fe_2 = FieldElement::new(4, 5);
        let fe_3 = FieldElement::new(4, 6);
        assert_eq!(fe.eq(&fe_2), true);
        assert_eq!(fe.eq(&fe_3), false)
    }

    #[test]
    fn field_element_add() {
        let fe = FieldElement::new(4, 5);
        let fe_2 = FieldElement::new(4, 5);
        assert_eq!(fe.add(&fe_2), 3);
    }

    #[test]
    fn field_element_sub() {
        let fe = FieldElement::new(9, 57);
        let fe_2 = FieldElement::new(29, 57);
        assert_eq!(fe.sub(&fe_2), 37);
    }

    #[test]
    fn field_element_mul() {
        let fe = FieldElement::new(3, 13);
        let fe_2 = FieldElement::new(12, 13);
        let fe_3 = FieldElement::new(10, 13);
        assert_eq!(fe.mul(&fe_2), 10);
        assert_eq!(fe_2.mul(&fe_3), 3);
    }

    #[test]
    fn field_element_pow() {
        let fe_3 = FieldElement::new(17, 31);
        assert_eq!(fe_3.pow(27), 29);
    }
}