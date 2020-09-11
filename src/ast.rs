use core::fmt;
use core::fmt::Error;
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Expr {
    fn label(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Op(_, opcode, _) => opcode.label().to_owned(),
        }
    }

    pub fn eval(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Op(exp1, opcode, exp2) => match opcode {
                Opcode::Mul => exp1.eval() * exp2.eval(),
                Opcode::Div => exp1.eval() / exp2.eval(),
                Opcode::Add => exp1.eval() + exp2.eval(),
                Opcode::Sub => exp1.eval() - exp2.eval(),
            },
        }
    }
}

impl Opcode {
    fn label(&self) -> &'static str {
        match self {
            Opcode::Mul => "*",
            Opcode::Div => "/",
            Opcode::Add => "+",
            Opcode::Sub => "-",
        }
    }
}

struct FmtCtx<'a> {
    id: i32,
    ids: HashMap<&'a Expr, i32>,
}

impl<'a> FmtCtx<'a> {
    pub fn new() -> Self {
        FmtCtx {
            id: 0,
            ids: HashMap::new(),
        }
    }

    fn get_id(&mut self, expr: &'a Expr, formatter: &mut fmt::Formatter) -> Result<i32, Error> {
        Ok(match self.ids.get(&expr) {
            Some(expr_id) => *expr_id,
            None => {
                let exp_id = self.id;
                self.id = exp_id + 1;

                writeln!(formatter, "exp_{}[label=\"{}\"];", &exp_id, expr.label())?;
                self.ids.insert(expr, exp_id);
                exp_id
            }
        })
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut ctx = FmtCtx::new();

        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(exp) = queue.pop_front() {
            let exp_id = ctx.get_id(exp, formatter)?;
            if let Expr::Op(exp1, _, exp2) = exp {
                let exp1_id = ctx.get_id(exp1.as_ref(), formatter)?;
                let exp2_id = ctx.get_id(exp2.as_ref(), formatter)?;
                writeln!(formatter, "exp_{} -> exp_{}", exp1_id, exp_id)?;
                writeln!(formatter, "exp_{} -> exp_{}", exp2_id, exp_id)?;
                queue.push_back(exp1);
                queue.push_back(exp2);
            }
        }
        Ok(())
    }
}
