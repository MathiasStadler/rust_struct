// FROM HERE
// https://users.rust-lang.org/t/how-do-i-create-variable-shared-by-all-instances-of-a-struct/50674/13

use std::ops::Mul;
#[derive(PartialEq, Eq, Debug)]
struct PolynomialContext {
    n: i32,
    v: Vec<i32>,
}

struct Polynomial<'a> {
    ctx: &'a PolynomialContext,
    #[allow(dead_code)]
    whatever_internal_fields: i32,
}

impl<'a> Polynomial<'a> {

    fn multiply(&self, _other: &Self) -> Self {
        // debug_assert_eq!(self.ctx, other.ctx);
        println!("I can see: {:?}", (self.ctx.n, &self.ctx.v));
        Self {
            ctx: self.ctx,
            
            whatever_internal_fields: 0,
        }
    }
    
    fn new(ctx: &'a PolynomialContext) -> Self {
        Self {
            ctx,
            whatever_internal_fields: 0,
        }
    }
}
impl<'a> Mul<&Polynomial<'a>> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn mul(self, other: &Polynomial<'a>) -> Polynomial<'a> {
        self.multiply(other)
    }
}



fn main() {
    let ctx = PolynomialContext {
        n: 42,
        v: (1..10).collect(),
    };
    let p = Polynomial::new(&ctx);
   let _=  &p * &p;
}


// cargo run --example struct_wide_variable_1

// https://users.rust-lang.org/t/how-to-create-a-struct-with-fields-which-reference-each-other/78260/17

