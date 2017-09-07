#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate c3_clang_extensions;
extern crate clang_sys;

pub mod expr;
pub mod ty;

mod bindgen { pub(crate) mod clang; }
mod clang_extensions;
mod error;

pub use error::*;
use expr::*;
use ty::*;
use std::path::Path;

use c3_clang_extensions::*;
use clang_sys::*;
use bindgen::clang;
use bindgen::clang::{UnsavedFile, Cursor};
use clang_extensions::{MapResult, CursorExt};

struct TyOptions {
    typerefs: bool,
}

impl TyOptions {
    pub fn typerefs(typerefs: bool) -> Self {
        Self {
            typerefs,
        }
    }
}

pub struct C3 {
    translation_unit: Option<clang::TranslationUnit>,
}

impl C3 {
    pub fn new() -> Self {
        Self {
            translation_unit: None,
        }
    }

    pub fn parse_file(&mut self, file_path: &Path, compiler_flags: &[String]) -> Res<Expr> {
        self.parse(file_path, &vec![], compiler_flags)
    }

    pub fn parse_source(&mut self, code: &str, fname: &Path, compiler_flags: &[String]) -> Res<Expr> {
        self.parse(fname, &vec![UnsavedFile::new(fname.to_str().ok_or("non-utf8 filename")?, code)], compiler_flags)
    }

    fn parse(&mut self, file_path: &Path, unsaved: &[UnsavedFile], compiler_flags: &[String]) -> Res<Expr> {
        let file_path = file_path.to_str().ok_or("non-utf8 filename")?;
        let ix = clang::Index::new(false, true);
        self.translation_unit = Some(clang::TranslationUnit::parse(&ix, file_path, compiler_flags, unsaved, CXTranslationUnit_Flags::empty()).ok_or("clang parse error")?);
        let cur = self.translation_unit.as_ref().ok_or("clang err")?.cursor();
        let res = self.tu_from_cursor(cur);
        self.translation_unit = None;
        res
    }

    pub fn dump_ty(&self, depth: usize, label: &str, ty: clang::Type) {
        if ty.kind() == CXType_Invalid {
            return;
        }
        let indent = "  ".repeat(depth);
        if depth > 19 {
            println!("{}+{}…", indent, label);
        }
        println!("{}+{}{:?}", indent, label, ty);
        if let Some(elem) = ty.elem_type() {
            self.dump_ty(depth+1, "elem=", elem);
        }
    }

    pub fn dump(&self, label: &str, cur: Cursor) {
        println!("{}: ", label);
        self.dump_cur(1, cur);
        println!();
    }

    fn dump_cur(&self, depth: usize, cur: Cursor) {
        let indent = "  ".repeat(depth);
        println!("{}• {:?}", indent, cur);

        self.dump_ty(depth+1, "", cur.cur_type());
        if let Some(elem) = cur.typedef_type() {
            self.dump_ty(depth+1, "typedef=", elem);
        }

        for child in cur.collect_children() {
            if depth > 18 {
                println!("{} …", indent);
                return;
            }
            self.dump_cur(depth+1, child);
        }
    }

    fn type_name_cur(&self, cur: Cursor) -> Res<String> {
        if !cur.is_anon() {
            let name = cur.spelling();
            if name != "" {
                return Ok(name);
            }
        }
        Ok(format!("anon {}", cur.usr().unwrap_or("anon".to_owned())))
    }

    fn type_name(&self, ty: clang::Type) -> Res<String> {
        let name = ty.spelling();
        let name = if name == "" {
            self.type_name_cur(ty.declaration().ok_or("anon type without declaration")?)?
        } else {
            name
        };

        if name.contains(' ') {
            Ok(name.split_whitespace().filter_map(|w| match w {
                "const" | "struct" | "*" => None,
                w => Some(w),
            }).collect::<Vec<_>>().join("_"))
        } else {
            Ok(name)
        }
    }

    fn tu_from_cursor(&self, cur: Cursor) -> Res<Expr> {
        // self.dump("tu",cur);
        match cur.kind() {
            CXCursor_TranslationUnit => Ok(Expr {
                loc: cur.loc(),
                kind: Kind::TranslationUnit(TranslationUnit {
                    name: cur.spelling(),
                    items: self.exprs_from_children(cur)
                        .map_err(|err| err.context("error when parsing top-level item"))?
                }),
            }),
            _ => Err("unexpected type")?,
        }
    }

    fn expr_from_child(&self, cur: Cursor) -> Res<Option<Box<Expr>>> {
        Ok(if let Some(ch) = cur.first_child() {
            Some(Box::new(self.expr_from_cur(ch)?))
        } else {
            None
        })
    }

    fn expr_from_iter<I: IntoIterator<Item=Cursor>>(&self, iter: I) -> Res<Option<Box<Expr>>> {
        let mut iter = iter.into_iter();
        Ok(if let Some(ch) = iter.next() {
            Some(Box::new(self.expr_from_cur(ch)?))
        } else {
            None
        })
    }

    fn exprs_from_children(&self, cur: Cursor) -> Res<Vec<Expr>> {
        cur.map_children_err(|c| self.expr_from_cur(c))
    }

    fn is_pointer(cur: Cursor) -> bool {
        match cur.cur_type().kind() {
            CXType_IncompleteArray |
            CXType_ConstantArray |
            CXType_VariableArray |

            CXType_Pointer |
            CXType_RValueReference |
            CXType_LValueReference |
            CXType_MemberPointer |
            CXType_ObjCObjectPointer => true,
            _ => false,
        }
    }

    fn to_boolean_context(&self, cur: Cursor, expr: Expr) -> Res<Box<Expr>> {
        Ok(Box::new(if Self::is_pointer(cur) {
            let is_boolean = match expr.kind {
                Kind::UnaryOperator(expr::UnaryOperator{kind:UnOpKind::IsNull,..}) => true,
                _ => false,
            };
            if !is_boolean {
                Expr {
                    loc: expr.loc,
                    kind: Kind::UnaryOperator(UnaryOperator {
                        kind: UnOpKind::Not,
                        arg: Box::new(Expr {
                            loc: expr.loc,
                            kind: Kind::UnaryOperator(UnaryOperator {
                                kind: UnOpKind::IsNull,
                                arg: Box::new(expr),
                            }),
                        }),
                    }),
                }
            } else {
                expr
            }
        } else {
            expr
        }))
    }

    fn expr_from_cur(&self, cur: Cursor) -> Res<Expr> {
        if !cur.is_valid() {
            Err("invalid cursor")?;
        }

        if let Some(source_file) = cur.file().name() {
            if source_file.starts_with("/Applications/Xcode.app/") || source_file.starts_with("/usr/include/") {
                return Ok(Expr {
                    loc: cur.loc(),
                    kind: Kind::TransparentGroup(TransparentGroup {
                        items: vec![],
                    }),
                });
            }
        }

        Ok(Expr{
            loc: cur.loc(),
            kind: match cur.kind() {

            // Top-level Items
            CXCursor_TypedefDecl => {
                Kind::TyDecl(TyDecl {
                    ty: self.ty_from_ty(cur.typedef_type().ok_or("typedef")?, cur.collect_children())?,
                    name: self.type_name_cur(cur)?,
                })
            },
            CXCursor_EnumDecl | CXCursor_StructDecl | CXCursor_UnionDecl => {
                Kind::TyDecl(TyDecl {
                    ty: self.ty_from_cur(cur)?,
                    name: self.type_name_cur(cur)?,
                })
            }
            CXCursor_ForStmt => {
                self.for_from_cur(cur)?
            },
            CXCursor_WhileStmt => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err("while args")?;
                }
                let cond = self.expr_from_cur(ch[0])?;
                Kind::While(While {
                    cond: self.to_boolean_context(ch[0], cond)?,
                    body: Box::new(self.expr_from_cur(ch[1])?),
                    is_do: false,
                })
            },
            CXCursor_DoStmt => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err("do while args")?;
                }
                let body = Box::new(self.expr_from_cur(ch[0])?);
                let cond = self.expr_from_cur(ch[1])?;
                Kind::While(While {
                    body,
                    cond: self.to_boolean_context(ch[1], cond)?,
                    is_do: true,
                })
            },
            CXCursor_BreakStmt => {
                Kind::Break
            },
            CXCursor_GotoStmt => {
                Kind::Goto(cur.first_child().ok_or("no goto label")?.spelling())
            },
            CXCursor_ContinueStmt => {
                Kind::Continue
            },
            CXCursor_FunctionDecl => {
                self.fn_from_cur(cur)
                    .map_err(|err| err.context(format!("error in function {}", cur.spelling())))?
            },
            CXCursor_VarDecl => {
                self.var_from_cur(cur)
                    .map_err(|err| err.context(format!("error when parsing variable {}", cur.spelling())))?
            },
            CXCursor_StmtExpr => {
                Kind::Block(Block {
                    items: self.exprs_from_children(cur)?,
                    returns_value: true,
                })
            },
            CXCursor_CompoundStmt => {
                Kind::Block(Block {
                    items: self.exprs_from_children(cur)?,
                    returns_value: false,
                })
            },
            CXCursor_ReturnStmt => {
                Kind::Return(self.expr_from_child(cur)?)
            },
            CXCursor_SwitchStmt => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err("unexpected switch children")?;
                }
                Kind::Switch(Switch {
                    cond: Box::new(self.expr_from_cur(ch[0])?),
                    items: self.exprs_from_children(ch[1])?,
                })
            },
            CXCursor_CaseStmt => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err("unexpected case children")?;
                }
                Kind::Case(Case {
                    conds: vec![self.expr_from_cur(ch[0])?],
                    items: vec![self.expr_from_cur(ch[1])?],
                })
            },
            CXCursor_DefaultStmt => {
                Kind::Case(Case {
                    conds: vec![],
                    items: self.exprs_from_children(cur)?,
                })
            },
            CXCursor_CallExpr => {
                let mut args = self.exprs_from_children(cur)
                    .map_err(|err|err.context(format!("while calling fn {}", cur.spelling())))?;
                if args.is_empty() {
                    Err("missing call args")?;
                }
                let callee = Box::new(args.remove(0));
                Kind::Call(Call {
                    callee,
                    args,
                })
            },
            CXCursor_ConditionalOperator => {
                let ch = cur.collect_children();
                if ch.len() != 3 {
                    Err(format!("weird ternary operator {:?}", cur))?;
                }
                let cond = self.expr_from_cur(ch[0])?;
                Kind::If(If {
                    cond: self.to_boolean_context(ch[0], cond)?,
                    body: Box::new(self.expr_from_cur(ch[1])?),
                    alt: Some(Box::new(self.expr_from_cur(ch[2])?)),
                    returns_value: true,
                })
            },
            CXCursor_IfStmt => {
                self.if_from_cur(cur)?
            },
            CXCursor_CompoundLiteralExpr => {
                let mut child_iter = cur.collect_children().into_iter();
                let ty = self.ty_from_ty(cur.cur_type(), &mut child_iter)?;
                let value = child_iter.filter(|ch| ch.kind() == CXCursor_InitListExpr)
                    .next().ok_or("compound w/o value")?;
                let value = self.expr_from_cur(value)
                    .map_err(|err| {
                        self.dump("CompoundLiteral", cur);
                        err.context("when getting expr base of CompoundLiteral")
                    })?;
                Kind::CompoundLiteral(CompoundLiteral{
                    ty,
                    items: match value.kind {
                        Kind::InitList(items) => items,
                        _ => Err("expected initlist")?,
                    },
                })
            },
            CXCursor_BinaryOperator | CXCursor_CompoundAssignOperator => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err(format!("Bad args for bin op {:?}", ch))?;
                }
                let mut left = Box::new(self.expr_from_cur(ch[0])?);
                let mut right = Box::new(self.expr_from_cur(ch[1])?);

                use self::BinaryOperatorKind::*;
                let kind = match cur.binary_opcode() {
                    BO_Add => if Self::is_pointer(ch[0]) {BinOpKind::AddPtr} else {BinOpKind::Add},
                    BO_Sub => if Self::is_pointer(ch[0]) {BinOpKind::SubPtr} else {BinOpKind::Sub},
                    BO_Mul => BinOpKind::Mul,
                    BO_Div => BinOpKind::Div,
                    BO_Rem => BinOpKind::Rem,
                    BO_LAnd => {
                        left = self.to_boolean_context(ch[0], *left)?;
                        right = self.to_boolean_context(ch[1], *right)?;
                        BinOpKind::And
                    },
                    BO_LOr => {
                        left = self.to_boolean_context(ch[0], *left)?;
                        right = self.to_boolean_context(ch[1], *right)?;
                        BinOpKind::Or
                    },
                    BO_Xor => BinOpKind::BitXor,
                    BO_And => BinOpKind::BitAnd,
                    BO_Or => BinOpKind::BitOr,
                    BO_Shl => BinOpKind::Shl,
                    BO_Shr => BinOpKind::Shr,
                    BO_EQ => BinOpKind::Eq,
                    BO_LT => BinOpKind::Lt,
                    BO_LE => BinOpKind::Le,
                    BO_NE => BinOpKind::Ne,
                    BO_GE => BinOpKind::Ge,
                    BO_GT => BinOpKind::Gt,
                    BO_MulAssign => BinOpKind::MulAssign,
                    BO_DivAssign => BinOpKind::DivAssign,
                    BO_RemAssign => BinOpKind::RemAssign,
                    BO_AddAssign => if Self::is_pointer(ch[0]) {BinOpKind::AddPtrAssign} else {BinOpKind::AddAssign},
                    BO_SubAssign => if Self::is_pointer(ch[0]) {BinOpKind::SubPtrAssign} else {BinOpKind::SubAssign},
                    BO_ShlAssign => BinOpKind::ShlAssign,
                    BO_ShrAssign => BinOpKind::ShrAssign,
                    BO_AndAssign => BinOpKind::BitAndAssign,
                    BO_XorAssign => BinOpKind::BitXorAssign,
                    BO_OrAssign  => BinOpKind::BitOrAssign,
                    BO_Assign => BinOpKind::Assign,
                    BO_Comma => BinOpKind::Comma,
                    x => Err(format!("unknown opcode {:?}", x))?,
                };
                Kind::BinaryOperator(BinaryOperator {left, right, kind})
            },
            CXCursor_ArraySubscriptExpr => {
                let ch = cur.collect_children();
                if ch.len() != 2 {
                    Err(format!("Bad args for arr idx {:?}", ch))?;
                }
                let left = Box::new(self.expr_from_cur(ch[0])?);
                let right = Box::new(self.expr_from_cur(ch[1])?);
                Kind::BinaryOperator(BinaryOperator {left, right, kind: BinOpKind::ArrayIndex})
            },
            CXCursor_UnaryExpr => {
                let mut ch = cur.collect_children().into_iter();
                Kind::SizeOf(SizeOf {
                    ty: self.ty_from_ty(cur.sizeof_arg_type().ok_or("no type of sizeof")?, &mut ch)?,
                    arg: self.expr_from_iter(ch)?,
                })
            },
            CXCursor_UnaryOperator => {
                let ch = cur.first_child().ok_or("no arg for unary")?;
                let arg = Box::new(self.expr_from_cur(ch)?);
                use self::UnaryOperatorKind::*;
                Kind::UnaryOperator(UnaryOperator {
                    arg,
                    kind: match cur.unary_opcode() {
                        UO_PostInc => if Self::is_pointer(ch) {UnOpKind::PostIncPtr} else {UnOpKind::PostInc},
                        UO_PostDec => if Self::is_pointer(ch) {UnOpKind::PostDecPtr} else {UnOpKind::PostDec},
                        UO_PreInc => if Self::is_pointer(ch) {UnOpKind::PreIncPtr} else {UnOpKind::PreInc},
                        UO_PreDec => if Self::is_pointer(ch) {UnOpKind::PreDecPtr} else {UnOpKind::PreDec},
                        UO_AddrOf => {
                            let is_const = cur.cur_type().pointee_type()
                                .map(|t|t.is_const()).unwrap_or(false);
                            UnOpKind::AddrOf(is_const)
                        },
                        UO_Deref => UnOpKind::Deref,
                        UO_Plus => UnOpKind::Plus,
                        UO_Minus => UnOpKind::Minus,
                        UO_Not => if Self::is_pointer(ch) {UnOpKind::IsNull} else {UnOpKind::BinNot},
                        UO_LNot => if Self::is_pointer(ch) {UnOpKind::IsNull} else {UnOpKind::Not},
                        UO_Real | UO_Imag | UO_Coawait | UO_Extension => Err("not implemented")?,
                    }
                })
            },
            CXCursor_DeclRefExpr => {
                Kind::DeclRef(self.type_name_cur(cur)?)
            },
            CXCursor_IntegerLiteral => {
                Kind::IntegerLiteral(cur.get_int_value(), None)
            },
            CXCursor_FloatingLiteral => {
                Kind::FloatingLiteral(cur.get_float_value())
            },
            CXCursor_StringLiteral => {
                Kind::StringLiteral(Self::parse_c_str(cur.spelling()))
            },
            CXCursor_CharacterLiteral => {
                Kind::CharacterLiteral(::std::char::from_u32(cur.get_int_value() as u32).ok_or("char cast")?)
            },
            CXCursor_NullStmt => {
                Kind::TransparentGroup(TransparentGroup {
                    items: vec![],
                })
            }
            CXCursor_DeclStmt => {
                Kind::TransparentGroup(TransparentGroup {
                    items: self.exprs_from_children(cur)
                        .map_err(|err| err.context("error when parsing variable declarations"))?,
                }).flat()
            },
            CXCursor_ParenExpr => {
                Kind::Paren(self.expr_from_child(cur)?.ok_or("no arg for paren")?)
            },
            CXCursor_LabelStmt => {
                Kind::Label(
                    cur.spelling(),
                    self.expr_from_child(cur)?.ok_or("no arg for paren")?,
                )
            },
            CXCursor_InitListExpr => {
                Kind::InitList(self.exprs_from_children(cur)
                        .map_err(|err| err.context("init list"))?)
            },
            CXCursor_CStyleCastExpr => {
                Kind::Cast(Cast {
                    ty: self.ty_from_cur(cur)?,
                    arg: Box::new(self.expr_from_cur(cur.sub_expr().ok_or("missing cast expr")?)?),
                    explicit: true,
                })
            }
            CXCursor_MemberRefExpr => {
                Kind::MemberRef(MemberRef {
                    name: cur.spelling(),
                    arg: self.expr_from_child(cur)
                        .map_err(|err| err.context(format!("while getting base of ->{}", cur.spelling())))?
                        .ok_or("no arg for ->")?,
                })
            },
            CXCursor_UnexposedExpr => match cur.extended_kind() {
                // FIXME: fudge for PredefinedExpr, but not sure what that really is
                CusorKindExt::PredefinedExpr => {
                    self.expr_from_child(cur)?.ok_or("PredefinedExpr?")?.kind
                },
                CusorKindExt::ImplicitCastExpr => {
                    // It does not always put typerefs in child nodes?
                    let ty = self.ty_from_cur_opts(cur, TyOptions::typerefs(false))
                            .map_err(|err| err.context("type of ImplicitCastExpr"))?;
                    let sub = cur.sub_expr().ok_or("missing cast expr")?;
                    Kind::Cast(Cast {
                        ty,
                        arg: Box::new(self.expr_from_cur(sub)
                            .map_err(|err| err.context(format!("when getting expr base of ImplicitCastExpr {:?} -> {:?}", cur, sub)))?),
                        explicit: false,
                    })
                },
                CusorKindExt::OffsetOfExpr => {
                    let mut ch = cur.collect_children().into_iter();
                    Kind::OffsetOf(
                        self.ty_from_cur(ch.next().ok_or("missing offsetof type")?)?,
                        ch.last().ok_or("missing offsetof field")?.spelling(),
                    )
                },
                CusorKindExt::OpaqueValueExpr => {
                    Err(format!("error: opaque type without definition; probably header missing in C; {:?}", cur))?
                },
                CusorKindExt::DesignatedInitExpr => {
                    let ch = cur.collect_children();
                    if ch.len() != 2 {
                        Err("unexpected len of designated init")?;
                    }
                    Kind::DesignatedInit(
                        ch[0].spelling(),
                        Box::new(self.expr_from_cur(ch[1])?),
                    )
                },
                CusorKindExt::RanOutOfIdeas => {
                    Err(format!("bummer; totally unknown kind of expression {:?}", cur))?
                },
                kind => Err(format!("Unexposed Clang expression found {:?} = {:?}", kind, cur))?,
            },
            CXCursor_AsmStmt => {
                eprintln!("warning: __asm__ not supported");
                Kind::TransparentGroup(TransparentGroup{items:vec![]})
            },
            CXCursor_UnexposedDecl => {
                // shrug
                Kind::TransparentGroup(TransparentGroup{items:vec![]})
            },
            kind => {
                self.dump("Not implemented", cur);
                Err(format!("Unsupported type of expression {:?} found in Clang AST: {:?}\n  (it's in {:?})", kind, cur, cur.lexical_parent()))?
            }
        }})
    }

    fn parse_c_str(s: String) -> String {
        if s.len() < 2 {
            return s; // Not possible?
        }
        let mut s = s[1..s.len()-1].chars();
        let mut out = String::new();
        while let Some(ch) = s.next() {
            out.push(match ch {
                '\\' => {
                    match s.next().unwrap_or('?') {
                        'n' => '\n',
                        'r' => '\r',
                        'v' => '\x0B',
                        't' => '\t',
                        'f' => '\x0C',
                        'b' => '\x08',
                        'a' => '\x07',
                        'e' => '\x1B',
                        // FIXME: hex and octal, U, u
                        'x' | 'u' | 'U' | '0'...'7' => {
                            eprintln!("warning: char string escape not supported yet");
                            '\\'
                        },
                        other => other,
                    }
                },
                other => other,
            });
        }
        out
    }

    fn for_from_cur(&self, cur: Cursor) -> Res<Kind> {
        Ok(Kind::For(For {
            init: if let Some(ch) = cur.for_init() {Some(Box::new(self.expr_from_cur(ch)?))} else {None},
            cond: if let Some(ch) = cur.for_cond() {
                let cond = self.expr_from_cur(ch)?;
                Some(self.to_boolean_context(ch, cond)?)
            } else {None},
            inc: if let Some(ch) = cur.for_inc() {Some(Box::new(self.expr_from_cur(ch)?))} else {None},
            body: Box::new(self.expr_from_cur(cur.for_body())?),
        }))
    }

    fn if_from_cur(&self, cur: Cursor) -> Res<Kind> {
        let cond_cur = cur.if_cond();
        let cond_expr = self.expr_from_cur(cond_cur).map_err(|err|{
            err.context(format!("while parsing cond {:?}", cond_cur))
        })?;
        Ok(Kind::If(If {
            cond: self.to_boolean_context(cond_cur, cond_expr)?,
            body: Box::new(self.expr_from_cur(cur.if_then()).map_err(|err|{
                        err.context(format!("while parsing then {:?}", cur))
                    })?),
            alt: if let Some(child) = cur.if_else() {
                Some(Box::new(self.expr_from_cur(child)?))
            } else {
                None
            },
            returns_value: false,
        }))
    }

    fn var_from_cur(&self, cur: Cursor) -> Res<Kind> {
        let mut child_iter = cur.collect_children().into_iter();
        let ty = self.ty_from_ty(cur.cur_type(), &mut child_iter)?;

        let init = if let Some(child) = cur.var_init() {
            Some(Box::new(self.expr_from_cur(child)?))
        } else {
            None
        };

        Ok(Kind::VarDecl(VarDecl {
            name: cur.spelling(),
            init,
            ty: Some(ty),
            storage: self.storage_from_cur(cur),
        }))
    }

    fn ty_from_cur(&self, cur: Cursor) -> Res<Ty> {
        self.ty_from_cur_opts(cur, TyOptions::typerefs(true))
    }

    fn ty_from_cur_opts(&self, cur: Cursor, opts: TyOptions) -> Res<Ty> {
        self.ty_from_ty_opts(cur.cur_type(), cur.collect_children(), opts)
            .map_err(|err| {
                self.dump("err in ty", cur);
                err.context(format!("error in type {:?}", cur))
            })
    }

    fn ty_from_ty<I: IntoIterator<Item=Cursor>>(&self, ty: clang::Type, iter: I) -> Res<Ty> {
        self.ty_from_ty_opts(ty, iter, TyOptions::typerefs(true))
    }

    fn ty_from_ty_opts<I: IntoIterator<Item=Cursor>>(&self, ty: clang::Type, iter: I, opts: TyOptions) -> Res<Ty> {
        let ref mut iter = iter.into_iter();
        self.ty_from_ty_iter(ty, iter, opts)
    }

    fn ty_from_ty_iter<I: Iterator<Item=Cursor>>(&self, ty: clang::Type, iter: &mut I, opts: TyOptions) -> Res<Ty> {
        Ok(Ty {
            is_const: ty.is_const(),
            debug_name: self.type_name(ty)?,
            kind: match ty.kind() {
            CXType_Bool => TyKind::Bool,
            CXType_Int => TyKind::Int,
            CXType_UShort => TyKind::UShort,
            CXType_Short => TyKind::Short,
            CXType_UInt => TyKind::UInt,
            CXType_Long => TyKind::Long,
            CXType_ULong => TyKind::ULong,
            CXType_LongLong => TyKind::LongLong,
            CXType_ULongLong => TyKind::ULongLong,
            CXType_Float => TyKind::Float,
            CXType_Double => TyKind::Double,
            CXType_LongDouble => TyKind::LongDouble,
            CXType_UChar | CXType_Char_U => TyKind::UChar,
            CXType_SChar | CXType_Char_S => TyKind::SChar,
            CXType_WChar => TyKind::WChar,
            CXType_Char32 => TyKind::Char32,
            CXType_Char16 => TyKind::Char16,
            CXType_Void => TyKind::Void,
            CXType_IncompleteArray => TyKind::IncompleteArray(Box::new(self.ty_from_ty_iter(ty.elem_type().ok_or("array type")?, iter, opts)?)),
            CXType_ConstantArray => {
                let elem_ty = self.ty_from_ty_iter(ty.elem_type().ok_or("array type")?, iter, opts)?;
                TyKind::ConstantArray(ty.num_elements().ok_or("array without size")?, Box::new(elem_ty))
            },
            CXType_VariableArray => {
                let elem_ty = self.ty_from_ty_iter(ty.elem_type().ok_or("array type")?, iter, opts)?;
                if let Some(size) = iter.next() {
                    let size = self.expr_from_cur(size)?;
                    TyKind::VariableArray(Box::new(size), Box::new(elem_ty))
                } else {
                    // Only declaration location has the size. Other places have only type, no size.
                    TyKind::IncompleteArray(Box::new(elem_ty))
                }
            },
            CXType_Pointer => TyKind::Pointer(Box::new(self.ty_from_ty_iter(ty.pointee_type().ok_or("pointee type")?, iter, opts)?)),
            CXType_Typedef => {
                // Seems necessary to consume TypeRef, otherwise VarDecl goes out of sync
                if opts.typerefs {
                    if let Some(ch) = iter.next() {
                        match ch.kind() {
                            CXCursor_TypeRef => {},
                            CXCursor_DeclRefExpr => {},
                            _ => {
                                eprintln!("warning: foung unexpected element in clang AST. Expected TypeRef, found {:?}", ch);
                            }
                        }
                    }
                }

                TyKind::Typedef(self.type_name(ty)?)
            },
            CXType_Enum => {
                TyKind::Enum(
                    self.type_name_cur(ty.declaration().ok_or("type decl")?)?,
                    iter.map(|ch| {
                        assert_eq!(CXCursor_EnumConstantDecl, ch.kind());
                        Ok(EnumConstant {
                            name: ch.spelling(),
                            value: if let Some(ch) = ch.first_child() {Some(self.expr_from_cur(ch)?)} else {None},
                        })
                    }).collect::<Res<_>>()?,
                )
            },
            CXType_Record => {
                let decl = ty.declaration().ok_or("record decl")?;
                let name = self.type_name_cur(decl)?;
                let items = iter
                    // both adhoc struct decls and anon fields add extra nodes?
                    .filter(|ch| ch.kind() == CXCursor_FieldDecl)
                    .map(|ch| {
                        Ok(Field {
                            ty: self.ty_from_cur(ch)?,
                            name: ch.spelling(),
                        })
                    }).collect::<Res<_>>()?;
                match decl.kind() {
                    CXCursor_StructDecl => TyKind::Struct(name, items),
                    CXCursor_UnionDecl => TyKind::Union(name, items),
                    _ => Err(format!("unknown kind {:?}", decl))?
                }
            },
            CXType_Elaborated => {
                // FIXME: *this* may be the only inline declaration for (struct Foo{} var;)
                // However, is_definition() is inaccurate, so it needs a workaround
                // potentially check loc of parent cur with cur of declaration()
                TyKind::Elaborated(
                    self.type_name_cur(ty.declaration().ok_or("eladecl")?)?,
                    // I don't know why it exists.
                    if let Some(cur) = iter.next() {
                        Some(Box::new(self.ty_from_cur(cur)?))
                    } else {
                        None
                    },
                )
            },
            CXType_FunctionProto => {
                TyKind::FunctionProto
            },
            CXType_FunctionNoProto => {
                TyKind::FunctionNoProto
            },
            CXType_Unexposed => {
                match ty.extended_type() {
                    ClangTypeExt::ParenType => {
                        // This is a fragile workaround
                        let canon = ty.canonical_type();
                        if canon != ty {
                            return self.ty_from_ty_iter(canon, iter, TyOptions::typerefs(false));
                        } else {
                            Err("paren type without canonical type")?
                        }
                    },
                    k => Err(format!("ERROR: Extended type kind is unsupported '{}', {:?}; {:?}\n ", ty.spelling(), k, ty))?
                }
            },
            k => {
                Err(format!("ERROR: Standard type kind is unsupported '{}', {:?}; {:?}\n ", ty.spelling(), k, ty))?
            },
        }})
    }

    fn storage_from_cur(&self, cur: Cursor) -> Storage {
        match cur.storage() {
            CX_SC_Extern => Storage::Extern,
            CX_SC_Static | CX_SC_PrivateExtern => Storage::Static,
            _ => Storage::None,
        }
    }

    fn fn_from_cur(&self, cur: Cursor) -> Res<Kind> {
        let mut body = None;
        let mut args = vec![];
        let mut storage = self.storage_from_cur(cur);
        let fn_ty = cur.cur_type();
        let variadic = fn_ty.is_variadic();
        let abi = match fn_ty.call_conv() {
            CXCallingConv_AAPCS => Abi::Aapcs,
            CXCallingConv_AAPCS_VFP => Abi::Aapcs,
            CXCallingConv_C => Abi::C,
            CXCallingConv_Default => Abi::C,
            CXCallingConv_X86FastCall => Abi::Fastcall,
            CXCallingConv_X86StdCall => Abi::Stdcall,
            CXCallingConv_X86ThisCall => Abi::Thiscall,
            CXCallingConv_X86VectorCall => Abi::Vectorcall,
            CXCallingConv_X86_64SysV => Abi::SysV64,
            CXCallingConv_X86_64Win64 => Abi::Win64,
            _ => Err("unsupported calling convention")?,
        };
        let children = cur.collect_children();
        for child in &children {
            match child.kind() {
                CXCursor_VisibilityAttr => {
                    if child.spelling() == "hidden" {
                        // Citrus interprets all of it as private
                        storage = Storage::Static;
                    }
                },
                _ => {},
            }
        }
        let mut children = children.into_iter()
            // FIXME: support attrs
            .filter(|c| c.kind() != CXCursor_UnexposedAttr) // fn attrs come before ret ty
            .filter(|c| c.kind() != CXCursor_VisibilityAttr);

        let ty = self.ty_from_ty(cur.ret_type().ok_or("no return type for fn?")?, &mut children)
                .map_err(|err| {
                    self.dump_ty(0, "err in ty", cur.ret_type().unwrap());
                    self.dump("err in ty", cur);

                    err.context(format!("ret type of function is borked {}", cur.spelling()))
                })?;
        for child in children {
            match child.kind() {
                CXCursor_CompoundStmt => {
                    body = Some(Box::new(self.expr_from_cur(child)
                    .map_err(|err| {
                        self.dump("failed function body", child);
                        err.context(format!("error when parsing body of {}() {:?}", cur.spelling(), cur.loc()))
                    })?));
                },
                CXCursor_ParmDecl => {
                    args.push(Arg {
                        ty: self.ty_from_cur(child)?,
                        name: child.spelling(),
                        loc: child.loc(),
                    })
                },
                _ => {
                    Err(format!("ERROR: unknown kind of function child? {:?}", child))?;
                }
            }
        }

        let f = FunctionDecl {
            name: cur.spelling(),
            args,
            ty,
            storage,
            abi,
            variadic,
        };
        if let Some(body) = body {
            Ok(Kind::FunctionDecl(f, body))
        } else {
            Ok(Kind::FunctionProtoDecl(f))
        }
    }
}

#[cfg(test)]
fn test_parse(source: &str) -> Vec<Expr> {
    let tu = C3::new().parse_source(source, Path::new("_test_.c"), &[])
        .unwrap();
    match tu.kind {
        Kind::TranslationUnit(tu) => tu.items,
        _ => panic!("{:?}", tu),
    }
}

#[test]
fn test_parse_enum() {
    let items = test_parse("enum Foo {BAR=1, BAZ};");
    assert_eq!(items.len(), 1);
}

#[test]
fn test_parse_typedef_pointer() {
    test_parse(r"typedef int rgba_pixel;
void test(const void* bitmap) {
    (rgba_pixel *const)bitmap;
}
");
}

#[test]
fn test_parse_typedef() {
    let items = test_parse("typedef int renamed; typedef renamed renamed_twice;");
    match items[0].kind {
        Kind::TyDecl(ref t) if t.name == "renamed" => {},
        _ => panic!(),
    }
    match items[1].kind {
        Kind::TyDecl(ref t) if t.name == "renamed_twice" => {
            match t.ty.kind {
                TyKind::Typedef(ref name) if name == "renamed" => {},
                _ => panic!(),
            }
        },
        _ => panic!(),
    }
}

#[test]
fn test_parse_ptr() {
    let items = test_parse(r"int main(int argc, char *argv[]) {
        argv + 1;
        argv += 1;
        argv++;
        argc + 1;
        argc++;
    }");

    match items[0].kind {
        Kind::FunctionDecl(_, ref body) => {
            match body.kind {
                Kind::Block(Block{ref items,..}) => {
                    match items[0].kind {
                        Kind::BinaryOperator(BinaryOperator{kind:BinOpKind::AddPtr,..}) => {},
                        _ => panic!(),
                    };
                    match items[1].kind {
                        Kind::BinaryOperator(BinaryOperator{kind:BinOpKind::AddPtrAssign,..}) => {},
                        _ => panic!(),
                    };
                    match items[2].kind {
                        Kind::UnaryOperator(UnaryOperator{kind:UnOpKind::PostIncPtr,..}) => {},
                        _ => panic!(),
                    };
                    match items[3].kind {
                        Kind::BinaryOperator(BinaryOperator{kind:BinOpKind::Add,..}) => {},
                        _ => panic!(),
                    };
                    match items[4].kind {
                        Kind::UnaryOperator(UnaryOperator{kind:UnOpKind::PostInc,..}) => {},
                        _ => panic!(),
                    };
                },
                _ => panic!("bl {:?}", body),
            }
        },
        _ => panic!("fu {:?}", items),
    }
}
