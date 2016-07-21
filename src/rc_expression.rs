use std::collections::HashSet;
use std::rc::Rc;

use problem::Quantifier;

#[derive(Debug)]
pub enum Exp {
    And(Rc<Exp>, Rc<Exp>),
    Not(Rc<Exp>),
    Var(u32),
    True,
    False
}

#[derive(Debug)]
pub struct QBF {
    pub first_quantifier: Quantifier,
    pub last_quantifier: Quantifier,
    pub quantifier_blocks: Vec<u32>,
    pub expr: Rc<Exp>
}

fn same_thing<X>(a: &X, b: &X) -> bool {
    (a as *const _) == (b as *const _)
}

fn implied(exp: Rc<Exp>) -> (HashSet<*const Exp>, HashSet<*const Exp>) {
    let mut trues = HashSet::new();
    let mut falses = HashSet::new();
    let mut to_visit = vec![exp];

    while let Some(x) = to_visit.pop() {
        let expr_ptr = &*x as *const _;
        trues.insert(expr_ptr);
        match &*x {
            &Exp::And(ref p, ref q) => {
                to_visit.push(p.clone());
                to_visit.push(q.clone());
            },
            &Exp::Not(ref u) => {
                falses.insert(&**u as *const _);
            },
            _ => {}
        }
    }

    (trues, falses)
}

impl Exp {
    pub fn not(a: Rc<Exp>) -> Rc<Exp> {
        match &*a {
            &Exp::True => Rc::new(Exp::False),
            &Exp::False => Rc::new(Exp::True),
            &Exp::Not(ref e) => e.clone(),
            _ => Rc::new(Exp::Not(a.clone()))
        }
    }

    pub fn or(a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        Exp::not(Exp::and(Exp::not(a), Exp::not(b)))
    }

    pub fn and(a: Rc<Exp>, b: Rc<Exp>) -> Rc<Exp> {
        let ref a1 = *a.clone();
        let ref b1 = *b.clone();
        match (a1, b1) {
            (&Exp::False, _) => return a.clone(),
            (_, &Exp::False) => return b.clone(),
            (&Exp::True, _) => return b.clone(),
            (_, &Exp::True) => return a.clone(),
            (&Exp::And(ref p, ref q), _) if same_thing(&**p, b1) || same_thing(&**q, b1) => return a.clone(),
            (_, &Exp::And(ref p, ref q)) if same_thing(&**p, a1) || same_thing(&**q, a1) => return b.clone(),
            (_, &Exp::Not(ref v)) => {
                let ref v1 = *v.clone();
                match v1 {
                    &Exp::And(ref q, ref p) => {
                        if same_thing(&**q, &*a) {
                            return Exp::and(a.clone(), Exp::not(p.clone()));
                        } else if same_thing(&**p, &*a) {
                            return Exp::and(a.clone(), Exp::not(q.clone()));
                        }
                    },
                    _ => {}
                }
            },
            (&Exp::Not(ref u), _) => {
                let ref u1 = *u.clone();
                match u1 {
                    &Exp::And(ref q, ref p) => {
                        if same_thing(&**q, &*b) {
                            return Exp::and(b.clone(), Exp::not(p.clone()));
                        } else if same_thing(&**p, &*b) {
                            return Exp::and(b.clone(), Exp::not(q.clone()));
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        let (a_implied_true, a_implied_false) = implied(a.clone());
        let (b_implied_true, b_implied_false) = implied(b.clone());

        if a_implied_true.intersection(&b_implied_false).next().is_some() ||
           a_implied_false.intersection(&b_implied_true).next().is_some() {
            Rc::new(Exp::False)
        } else {
            Rc::new(Exp::And(a, b))
        }
    }

    pub fn size(&self) -> usize {
        let mut visited = HashSet::new();
        let mut size = 0;

        let mut to_visit = vec![self];
        while let Some(node) = to_visit.pop() {
            let expr_ptr = node as (*const _);
            if !visited.contains(&expr_ptr) {
                visited.insert(expr_ptr);
                size += 1;
                match node {
                    &Exp::And(ref a, ref b) => {
                        to_visit.push(&*a);
                        to_visit.push(&*b);
                    },
                    &Exp::Not(ref a) => {
                        to_visit.push(&*a);
                    }
                    _ => {}
                }
            }
        }

        size
    }
}
