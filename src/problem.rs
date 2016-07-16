use vars::Vars;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Quantifier {
    Exists,
    ForAll
}

pub fn opposite_quantifier(q: Quantifier) -> Quantifier {
    match q {
        Quantifier::Exists => Quantifier::ForAll,
        Quantifier::ForAll => Quantifier::Exists
    }
}

#[derive(Debug)]
pub enum Expression<'r> {
    And(Vars, &'r Expression<'r>, &'r Expression<'r>),
    Or(Vars, &'r Expression<'r>, &'r Expression<'r>),
    Not(&'r Expression<'r>),
    Var(u64),
    True,
    False
}

impl<'r> Expression<'r> {
    pub fn has_var(&self, var: u64) -> bool {
        match self {
            &Expression::And(ref v, _, _) => v.get(var),
            &Expression::Or(ref v, _, _) => v.get(var),
            &Expression::Not(ref v) => v.has_var(var),
            &Expression::Var(v) => v == var,
            _ => false
        }
    }

    fn with_variables<F, X>(&self, f: F) -> X
        where F: for<'r1> FnOnce(&'r1 Vars) -> X {
        match self {
            &Expression::And(ref v, _, _) => f(v),
            &Expression::Or(ref v, _, _) => f(v),
            &Expression::Not(ref v) => v.with_variables(f),
            &Expression::Var(v) => {
                let mut output = Vars::new();
                output.add(v);
                f(&output)
            },
            _ => f(&Vars::new())
        }
    }

    fn variables(&self) -> Vars {
        self.with_variables(|v| v.clone())
    }
}

pub fn and<'r>(a: &'r Expression<'r>, b: &'r Expression<'r>) -> Expression<'r> {
    let mut v_a = a.variables();
    let mut v_b = b.variables();
    v_a.union(&mut v_b);
    Expression::And(v_a, a, b)
}

pub fn or<'r>(a: &'r Expression<'r>, b: &'r Expression<'r>) -> Expression<'r> {
    let mut v_a = a.variables();
    let mut v_b = b.variables();
    v_a.union(&mut v_b);
    Expression::Or(v_a, a, b)
}

pub fn not<'r>(a: &'r Expression<'r>) -> Expression<'r> {
    Expression::Not(a)
}

pub static TRUE: Expression<'static> = Expression::True;
pub static FALSE: Expression<'static> = Expression::False;

#[derive(Debug)]
pub struct QBF<'r> {
    pub quantifiers: &'r [Quantifier],
    pub expr: &'r Expression<'r>
}
