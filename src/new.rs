use expr::*;
use ty::*;

impl Loc {
    pub fn dummy() -> Self {
        Self {
            start: LocPos {col:0,line:0},
            end: LocPos {col:0,line:0},
        }
    }
}

impl Kind {
    pub fn int(n: u64) -> Self {
        Kind::IntegerLiteral(n, None)
    }

    pub fn declref<S: Into<String>>(name: S) -> Self {
        Kind::DeclRef(name.into())
    }
}

impl VarDecl {
    pub fn new<S: Into<String>>(name: S, init: Option<Box<Expr>>, ty: Option<Ty>) -> Self {
        Self {
            name: name.into(),
            ty,
            init,
            storage: Storage::None,
        }
    }
}

impl UnOpKind {
    pub fn new<E: Into<Box<Expr>>>(self, arg: E) -> UnaryOperator {
        UnaryOperator::new(self, arg)
    }
}

impl UnaryOperator {
    pub fn new<E: Into<Box<Expr>>>(kind: UnOpKind, arg: E) -> Self {
        Self {
            kind,
            arg: arg.into(),
        }
    }
}

impl BinOpKind {
    pub fn new<E: Into<Box<Expr>>, E2: Into<Box<Expr>>>(self, left: E, right: E2) -> BinaryOperator {
        BinaryOperator::new(self, left, right)
    }
}

impl BinaryOperator {
    pub fn new<E: Into<Box<Expr>>, E2: Into<Box<Expr>>>(kind: BinOpKind, left: E, right: E2) ->  Self {
        Self {
            kind,
            left: left.into(),
            right: right.into(),
        }
    }
}
