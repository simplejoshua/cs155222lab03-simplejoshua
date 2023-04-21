fn main() {}

use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    use Expr::*;

    match expr {
        ArithExpr(arithexpr) => IntValue(eval_arith_expr(arithexpr)),
        BoolExpr(boolexpr) => BoolValue(eval_bool_expr(boolexpr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    use ArithExpr::*;

    match arith_expr {
        BinArithExpr { left, right, op } => {
            use BinArithOp::*;
            match op {
                AddOp => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp => eval_arith_expr(*left) / eval_arith_expr(*right),
            }
        }
        IntLit(num) => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    use BoolExpr::*;

    match bool_expr {
        ArithCmpExpr { left, right, op } => {
            use ArithCmpOp::*;
            match op {
                LtOp => eval_arith_expr(*left) < eval_arith_expr(*right),
                LteOp => eval_arith_expr(*left) <= eval_arith_expr(*right),
                GtOp => eval_arith_expr(*left) > eval_arith_expr(*right),
                GteOp => eval_arith_expr(*left) >= eval_arith_expr(*right),
                ArithEqOp => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithNeqOp => eval_arith_expr(*left) != eval_arith_expr(*right),
            }
        }
        BinBoolExpr { left, right, op } => {
            use BinLogicOp::*;

            match op {
                AndOp => eval_bool_expr(*left) && eval_bool_expr(*right),
                OrOp => eval_bool_expr(*left) || eval_bool_expr(*right),
                BoolEqOp => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        }
        NotExpr(bool_expr) => !(eval_bool_expr(*bool_expr)),
        BoolLit(bool) => bool,
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use super::*;

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer); // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }

    #[test]
    fn test_arithexpr_plus() {
        let expr = ArithExpr(BinArithExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: AddOp,
        });

        let answer = IntValue(100);
        assert_eq!(eval(expr), answer); // 56 + 44 = 100
    }

    #[test]
    fn test_arithexpr_minus() {
        let expr = ArithExpr(BinArithExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: SubOp,
        });

        let answer = IntValue(12);
        assert_eq!(eval(expr), answer); // 56 - 44 = 12
    }

    #[test]
    fn test_arithexpr_mul() {
        let expr = ArithExpr(BinArithExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: MulOp,
        });

        let answer = IntValue(2464);
        assert_eq!(eval(expr), answer); // 56 * 44 = 2464
    }
    #[test]
    fn test_arithexpr_div() {
        let expr = ArithExpr(BinArithExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: IntDivOp,
        });

        let answer = IntValue(1);
        assert_eq!(eval(expr), answer); // 56 / 44 = 1
    }

    #[test]
    fn test_arithcmp_lt() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: LtOp,
        });

        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer); // 56 < 44 -> false
    }

    #[test]
    fn test_arithcmp_lte() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: LteOp,
        });

        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer); // 56 <= 44 -> false
    }

    #[test]
    fn test_arithcmp_gt() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: GtOp,
        });

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // 56 > 44 -> true
    }

    #[test]
    fn test_arithcmp_gte() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: GteOp,
        });

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // 56 >= 44 -> false
    }

    #[test]
    fn test_arithcmp_aritheq() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: ArithEqOp,
        });

        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer); // 56 == 44 -> false
    }

    #[test]
    fn test_arithcmp_arithneq() {
        let expr = BoolExpr(ArithCmpExpr {
            left: Box::new(IntLit(56)),
            right: Box::new(IntLit(44)),
            op: ArithNeqOp,
        });

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // 56 != 44 -> true
    }

    #[test]
    fn test_binbool_and() {
        let expr = BoolExpr(BinBoolExpr {
            left: Box::new(BoolLit(true)),
            right: Box::new(BoolLit(false)),
            op: AndOp,
        });

        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer); // true && false -> false
    }

    #[test]
    fn test_binbool_or() {
        let expr = BoolExpr(BinBoolExpr {
            left: Box::new(BoolLit(false)),
            right: Box::new(BoolLit(true)),
            op: OrOp,
        });

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // true || false -> true
    }

    #[test]
    fn test_binbool_eq() {
        let expr = BoolExpr(BinBoolExpr {
            left: Box::new(BoolLit(true)),
            right: Box::new(BoolLit(false)),
            op: BoolEqOp,
        });

        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer); // true == false -> false
    }

    #[test]
    fn test_binbool_neq() {
        let expr = BoolExpr(BinBoolExpr {
            left: Box::new(BoolLit(true)),
            right: Box::new(BoolLit(false)),
            op: BoolNeqOp,
        });

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // true != false -> true
    }

    #[test]
    fn test_notexpr() {
        let expr = BoolExpr(NotExpr(Box::new(BoolLit(false))));

        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer); // !false -> true
    }
}
