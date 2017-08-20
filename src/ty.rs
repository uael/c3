use expr::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Default,
    Hidden,
}

#[derive(Clone, PartialEq)]
pub struct Ty {
    pub debug_name: String,
    pub kind: TyKind,
    pub is_const: bool,
}

#[derive(Debug, Clone)]
pub enum TyKind {
    Bool,
    Int,
    UInt,
    Short,
    UShort,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Int128,
    UInt128,
    Float,
    Double,
    LongDouble,
    SChar,
    UChar,
    WChar,
    Char16,
    Char32,
    Void,

    Auto,
    BlockPointer,
    Complex,
    ConstantArray(usize, Box<Ty>),
    VariableArray(Box<Expr>, Box<Ty>),
    Dependent,
    DependentSizedArray,
    Enum(String, Vec<EnumConstant>),
    Struct(String, Vec<Field>),
    Union(String, Vec<Field>),
    FunctionNoProto,
    FunctionProto,
    IncompleteArray(Box<Ty>),
    Pointer(Box<Ty>),
    Elaborated(String, Option<Box<Ty>>),
    Typedef(String),
    Record(Box<Expr>),

    /// Rusty
    Reference(Box<Ty>),
    Slice(Box<Ty>),

    // Not supported yet:
    // LValueReference,
    // MemberPointer,
    // NullPtr,
    // ObjCClass,
    // ObjCId,
    // ObjCInterface,
    // ObjCObjectPointer,
    // ObjCSel,
    // Overload,
    // RValueReference,
    // Typedef,
    // VariableArray,
    // Vector,
}

impl PartialEq for TyKind {
    fn eq(&self, other: &Self) -> bool {
        use TyKind::*;
        match (self, other) {
            (&Bool, &Bool) |
            (&Int, &Int) |
            (&UInt, &UInt) |
            (&Short, &Short) |
            (&UShort, &UShort) |
            (&Long, &Long) |
            (&ULong, &ULong) |
            (&LongLong, &LongLong) |
            (&ULongLong, &ULongLong) |
            (&Int128, &Int128) |
            (&UInt128, &UInt128) |
            (&Float, &Float) |
            (&Double, &Double) |
            (&LongDouble, &LongDouble) |
            (&SChar, &SChar) |
            (&UChar, &UChar) |
            (&WChar, &WChar) |
            (&Char16, &Char16) |
            (&Char32, &Char32) |
            (&Auto, &Auto) |
            (&BlockPointer, &BlockPointer) |
            (&Complex, &Complex) |
            (&Void, &Void) => true,
            (&ConstantArray(ref s1, ref ty1), &ConstantArray(ref s2, ref ty2)) if s1 == s2 && ty1 == ty2 => true,
            (&VariableArray(ref s1, ref ty1), &VariableArray(ref s2, ref ty2)) if s1 == s2 && ty1 == ty2 => true,
            (&Enum(ref s1, ref ty1), &Enum(ref s2, ref ty2)) if s1 == s2 && ty1 == ty2 => true,
            (&IncompleteArray(ref ty1), &IncompleteArray(ref ty2)) if ty1 == ty2 => true,
            (&Struct(ref s1, _), &Struct(ref s2, _)) if s1 == s2 => true,
            (&Union(ref s1, _), &Union(ref s2, _)) if s1 == s2 => true,
            (&Pointer(ref ty1), &Pointer(ref ty2)) if ty1 == ty2 => true,

            // // Dependent
            // // DependentSizedArray
            // // FunctionNoProto
            // // FunctionProto
            (&Elaborated(ref n1, _), &Elaborated(ref n2, _)) if n1 == n2 => true,
            (&Struct(ref n1, _), &Elaborated(ref n2, _)) if n1 == n2 => true,
            (&Union(ref n1, _), &Elaborated(ref n2, _)) if n1 == n2 => true,
            (&Enum(ref n1, _), &Elaborated(ref n2, _)) if n1 == n2 => true,
            (&Elaborated(ref n1, _), &Struct(ref n2, _)) if n1 == n2 => true,
            (&Elaborated(ref n1, _), &Union(ref n2, _)) if n1 == n2 => true,
            (&Elaborated(ref n1, _), &Enum(ref n2, _)) if n1 == n2 => true,
            (&Typedef(ref s1), &Typedef(ref s2)) if s1 == s2 => true,
            // // Record
            _ => false,
        }
    }
}

impl PartialEq for EnumConstant {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(Clone)]
pub struct EnumConstant {
    pub name: String,
    pub value: Option<Expr>,
}

impl fmt::Debug for EnumConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {:?}", self.name, self.value)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Storage {
    None,
    Static,
    Extern,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Abi {
    C,
    Rust,
    Stdcall,
    Fastcall,
    Vectorcall,
    Thiscall,
    Aapcs,
    Win64,
    SysV64,
}

#[derive(Clone)]
pub struct Arg {
    pub name: String,
    pub ty: Ty,
    pub loc: Loc,
}

impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) &&
        self.ty.eq(&other.ty)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub ty: Ty,
}

impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TyKind::*;
        if self.is_const {
            write!(f, "const ")?;
        }
        match self.kind {
            IncompleteArray(ref ty) => write!(f, "[{:?}; …]", ty),
            ConstantArray(ref sz, ref ty) => write!(f, "[{:?}; {:?}]", ty, sz),
            Pointer(ref po) => write!(f, "*{:?}", po),
            ref other => write!(f, "{:?}", other),
        }
    }
}

impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.name, self.ty)
    }
}
