use super::scanner::TokenKind;

pub trait SExpression {
    fn is_empty(&self) -> bool;
}

pub struct Empty {}

pub trait Atom: SExpression {}

pub struct AtomList {}

impl SExpression for AtomList {
    fn is_empty(&self) -> bool {
        false
    }
}

impl SExpression for Empty {
    fn is_empty(&self) -> bool {
        true
    }
}

pub trait ExpressionVisitor {
    fn visit(&self);
}

pub trait Walker {
    fn walk(&self);
}
