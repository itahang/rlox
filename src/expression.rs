use crate::token::{Literal, Token};
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub fn parenthesize(expression: &Expr) -> String {
    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                operator.get_lexeme_string(),
                parenthesize(left),
                parenthesize(right)
            )
        }

        Expr::Grouping { expression } => {
            format!("(group {})", parenthesize(expression))
        }

        Expr::Literal { value } => match value {
            Literal::Nil => "nil".to_string(),
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Boolean(b) => match b {
                true => "True".to_string(),
                _ => "False".to_string(),
            },
        },

        Expr::Unary { operator, right } => {
            format!("({} {})", operator.get_lexeme_string(), parenthesize(right))
        }
    }
}

pub fn rpn(expression: &Expr) -> String {
    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "{} {} {}",
                rpn(left),
                rpn(right),
                operator.get_lexeme_string(),
            )
        }

        Expr::Grouping { expression } => {
            format!("{} group ", rpn(expression))
        }

        Expr::Literal { value } => match value {
            Literal::Nil => "nil".to_string(),
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Boolean(b) => match b {
                true => "True".to_string(),
                _ => "False".to_string(),
            },
        },

        Expr::Unary { operator, right } => {
            format!("{} {}",  rpn(right),operator.get_lexeme_string())
        }
    }
}
