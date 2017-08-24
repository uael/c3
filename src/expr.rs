use std::fmt;
use ty::*;

#[derive(Clone)]
pub struct Expr {
    pub kind: Kind,
    pub loc: Loc,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// Top-level
    TyDecl(TyDecl),
    RecordDecl(RecordDecl),
    FunctionDecl(FunctionDecl, Box<Expr>),
    FunctionProtoDecl(FunctionDecl),
    VarDecl(VarDecl),

    /// Glue
    InitList(Vec<Expr>),
    TranslationUnit(TranslationUnit),
    ExternC(Vec<Expr>),

    /// Stmt and Expr
    Block(Block),
    Paren(Box<Expr>),
    Switch(Switch),
    Case(Case),
    For(For),
    While(While),
    If(If),
    Call(Call),
    Return(Option<Box<Expr>>),
    Break,
    Goto(String),
    Label(String, Box<Expr>),
    Continue,
    /// Regular variable used in code
    DeclRef(String),
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    SizeOf(SizeOf),
    OffsetOf(Ty, String),
    IntegerLiteral(u64, Option<Ty>),
    StringLiteral(String),
    FloatingLiteral(f64),
    CharacterLiteral(char),
    CompoundLiteral(CompoundLiteral),
    Cast(Cast),
    MemberRef(MemberRef),
    DesignatedInit(String, Box<Expr>),

    /// Special
    TransparentGroup(TransparentGroup),
    Invalid(String),
}

impl Kind {
    pub fn ty(&self) -> Option<&Ty> {
        Some(match *self {
            Kind::TyDecl(ref t) => &t.ty,
            Kind::FunctionDecl(ref t, _) => &t.ty,
            Kind::FunctionProtoDecl(ref t) => &t.ty,
            Kind::VarDecl(VarDecl{ty: Some(ref ty),..}) => ty,
            Kind::CompoundLiteral(ref t) => &t.ty,
            Kind::Cast(ref t) => &t.ty,
            Kind::Paren(ref t) => return t.kind.ty(),
            Kind::DesignatedInit(_, ref arg) => return arg.kind.ty(),
            Kind::If(ref t) => return t.body.kind.ty(),
            Kind::IntegerLiteral(_, Some(ref ty)) => return Some(ty),
            _ => return None,
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct TranslationUnit {
    pub name: String,
    pub items: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TyDecl {
    pub name: String,
    pub ty: Ty,
}

impl fmt::Debug for TranslationUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        for item in &self.items {
            write!(f, "{:?}\n", item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordDecl {}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub init: Option<Box<Expr>>,
    pub ty: Option<Ty>,
    pub storage: Storage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub args: Vec<Arg>,
    pub variadic: bool,
    pub ty: Ty,
    pub storage: Storage,
    pub abi: Abi,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOpKind {
    PostInc,
    PostDec,
    PreInc,
    PreDec,
    PostIncPtr,
    PostDecPtr,
    PreIncPtr,
    PreDecPtr,
    AddrOf,
    Deref,
    Plus,
    Minus,
    BinNot,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    AddPtr,
    SubPtr,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    MulAssign,
    DivAssign,
    RemAssign,
    AddAssign,
    SubAssign,
    AddPtrAssign,
    SubPtrAssign,
    ShlAssign,
    ShrAssign,
    BitAndAssign,
    BitXorAssign,
    BitOrAssign,
    Assign,
    Comma,

    ArrayIndex,
}

#[derive(Clone, PartialEq)]
pub struct BinaryOperator {
    pub kind: BinOpKind,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct UnaryOperator {
    pub kind: UnOpKind,
    pub arg: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SizeOf {
    pub arg: Option<Box<Expr>>,
    pub ty: Ty,
}


#[derive(Clone, PartialEq)]
pub struct FloatingLiteral {
    pub value: f64,
}

#[derive(Clone, PartialEq)]
pub struct CharacterLiteral {
    pub value: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundLiteral {
    pub items: Vec<Expr>,
    pub ty: Ty,
}

#[derive(Clone, PartialEq)]
pub struct StringLiteral {
    pub syntax: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LocPos {
    pub line: u32,
    pub col: u32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Loc {
    pub start: LocPos,
    pub end: LocPos,
}

#[derive(Clone, PartialEq)]
pub struct Cast {
    pub arg: Box<Expr>,
    pub ty: Ty,
    pub explicit: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberRef {
    pub arg: Box<Expr>,
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Switch {
    pub cond: Box<Expr>,
    pub items: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub cond: Box<Expr>,
    pub body: Box<Expr>,
    pub is_do: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct For {
    pub init: Option<Box<Expr>>,
    pub cond: Option<Box<Expr>>,
    pub inc: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct Case {
    pub cond: Option<Box<Expr>>, // None = Default
    pub items: Vec<Expr>,
}

#[derive(Clone, PartialEq)]
pub struct If {
    pub cond: Box<Expr>,
    pub body: Box<Expr>,
    pub alt: Option<Box<Expr>>,
    pub returns_value: bool,
}

#[derive(Clone, PartialEq)]
pub struct Block {
    pub items: Vec<Expr>,
    pub returns_value: bool,
}

#[derive(Clone, PartialEq)]
pub struct TransparentGroup {
    pub items: Vec<Expr>,
}


impl Kind {
    pub fn flat(mut self) -> Self {
        match self {
            Kind::TransparentGroup(TransparentGroup{ref mut items}) if items.len() == 1 => {
                items.remove(0).kind
            },
            _ => self,
        }
    }
}

fn fmt_items(f: &mut fmt::Formatter, items: &[Expr]) -> fmt::Result {
    f.debug_list().entries(items).finish()
}

impl fmt::Debug for TransparentGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.items.len() {
            0 => write!(f, ";"),
            1 => self.items[0].fmt(f),
            _ => fmt_items(f, &self.items)
        }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_items(f, &self.items)
    }
}

impl fmt::Debug for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:\n", self.cond)?;
        fmt_items(f, &self.items)
    }
}

impl fmt::Debug for Cast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})({:?})", self.ty, self.arg)
    }
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({:?})", self.callee, self.args)
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.loc, self.kind)
    }
}

impl fmt::Debug for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if {:?} {{\n{:?}\n}}\n", self.cond, self.body)?;
        if let Some(ref alt) = self.alt {
            write!(f, "else {{\n{:?}\n}}\n", alt)?;
        }
        Ok(())
    }
}

impl fmt::Debug for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?} {:?} {:?})", self.left, self.kind, self.right)
    }
}

impl fmt::Debug for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.arg)
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.start.line, self.start.col)?;
        if self.end.line != self.start.line {
            write!(f, "-{}:{}", self.end.line, self.end.col)?;
        } else if self.end.col != self.start.col {
            write!(f, "-{}", self.end.col)?;
        }
        Ok(())
    }
}
