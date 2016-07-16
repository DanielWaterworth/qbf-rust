use std::collections::HashMap;
use std::hash::Hash;

use problem;
use problem::Expression;
use problem::TRUE;
use problem::FALSE;

struct Substitutions<'r> {
    map: HashMap<*const (), &'r Expression<'r>>
}

fn get_clone<K, V>(m: &HashMap<K, V>, k: &K) -> Option<V>
    where K: Eq, K: Hash, V:Clone {
    match m.get(k) {
        Some(v) => Some(v.clone()),
        None => None
    }
}

fn substitute_and<'r, F, X>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr| {
        match expr {
            &Expression::False => f(subs1, &FALSE),
            &Expression::True => {
                let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| f(subs2, expr1);
                substitute_inner(subs1, b, variable, value, h)
            },
            _ => {
                let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| {
                    match expr1 {
                        &Expression::False => f(subs2, &FALSE),
                        &Expression::True => f(subs2, expr),
                        _ => {
                            let e = problem::and(expr, expr1);
                            f(subs2, &e)
                        }
                    }
                };
                substitute_inner(subs1, b, variable, value, h)
            }
        }
    };
    substitute_inner(subs, a, variable, value, g)
}

fn substitute_or<'r, F, X>(
        subs: Substitutions<'r>,
        a: &'r Expression<'r>,
        b: &'r Expression<'r>,
        variable: u64,
        value: bool,
        f: F) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr| {
        match expr {
            &Expression::True => f(subs1, &TRUE),
            &Expression::False => {
                let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| f(subs2, expr1);
                substitute_inner(subs1, b, variable, value, h)
            },
            _ => {
                let h: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs2, expr1| {
                    match expr1 {
                        &Expression::True => f(subs2, &TRUE),
                        &Expression::False => f(subs2, expr),
                        _ => {
                            let e = problem::or(expr, expr1);
                            f(subs2, &e)
                        }
                    }
                };
                substitute_inner(subs1, b, variable, value, h)
            }
        }
    };
    substitute_inner(subs, a, variable, value, g)
}

fn substitute_inner<'r, F, X>(
        subs: Substitutions<'r>,
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F) -> X
    where F : for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X {
    if !expr.has_var(variable) {
        return cb(subs, expr);
    };

    let expr_ptr = (expr as *const _) as *const ();
    match get_clone(&subs.map, &expr_ptr) {
        Some(expr1) => {
            return cb(subs, expr1);
        },
        None => {}
    };

    let f: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|mut subs1, expr1| {
        subs1.map.insert(expr_ptr, expr1);
        cb(subs1, expr1)
    };

    match expr {
        &Expression::True => f(subs, expr),
        &Expression::False => f(subs, expr),
        &Expression::Var(ref n) => {
            if *n == variable {
                if value {
                    f(subs, &TRUE)
                } else {
                    f(subs, &FALSE)
                }
            } else {
                f(subs, expr)
            }
        },
        &Expression::Not(ref a) => {
            let g: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|subs1, expr1| {
                match *expr1 {
                    Expression::True => f(subs1, &FALSE),
                    Expression::False => f(subs1, &TRUE),
                    _ => {
                        let e = Expression::Not(expr1);
                        f(subs1, &e)
                    }
                }
            };
            substitute_inner(subs, a, variable, value, g)
        },
        &Expression::Or(_, ref a, ref b) => substitute_or(subs, a, b, variable, value, f),
        &Expression::And(_, ref a, ref b) => substitute_and(subs, a, b, variable, value, f)
    }
}

pub fn substitute<'r, F, X>(
        expr: &'r Expression<'r>,
        variable: u64,
        value: bool,
        cb: F) -> X
    where F : for<'r1> Fn(&'r1 Expression<'r1>) -> X {
    let subs = Substitutions {map: HashMap::new()};
    let f: &for<'r1> Fn(Substitutions<'r1>, &'r1 Expression<'r1>) -> X = &|_, expr1| cb(expr1);
    substitute_inner(subs, expr, variable, value, f)
}
