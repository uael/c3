
use std::iter;
use std::vec;
use std::ptr;
use std::cmp::{min,max};
use clang_sys::*;
use bindgen::clang::*;
use expr::{Loc, LocPos};
use super::Res;

pub(crate) trait MapResult<T> {
    fn map_res<U, E, F>(self, f: F) -> Result<Option<U>, E> where F: FnOnce(T) -> Result<U, E>;
}

impl<T> MapResult<T> for Option<T> {
    fn map_res<U, E, F>(self, f: F) -> Result<Option<U>, E> where F: FnOnce(T) -> Result<U, E> {
        if let Some(val) = self {
            match f(val) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }
}

pub(crate) trait CursorExt where Self: Sized {
    fn map<T, F: FnMut(Cursor) -> T>(&self, cb: F) -> iter::Map<vec::IntoIter<Self>, F>;
    fn map_children<T, F: FnMut(Cursor) -> T>(&self, cb: F) -> Vec<T>;
    fn map_children_err<T,F: FnMut(Cursor) -> Res<T>>(&self, cb: F) -> Res<Vec<T>>;
    fn first_child(&self) -> Option<Cursor>;

    fn loc(&self) -> Loc;
    fn file(&self) -> File;
    fn limit_range(range: &mut CXSourceRange, cur: &Cursor);

    fn new_cur(x: CXCursor) -> Option<Cursor>;
    fn for_body(&self) -> Cursor;
    fn var_init(&self) -> Option<Cursor>;
    fn for_cond(&self) -> Option<Cursor>;
    fn for_inc(&self) -> Option<Cursor>;
    fn for_init(&self) -> Option<Cursor>;
    fn if_else(&self) -> Option<Cursor>;
    fn if_then(&self) -> Cursor;
    fn if_cond(&self) -> Cursor;
    fn if_condition_variable(&self) -> Option<Cursor>;
    fn sub_expr(&self) -> Option<Cursor>;
    fn is_anon(&self) -> bool;
    fn is_function_macro(&self, tu: &TranslationUnit) -> bool;

    fn binary_opcode(&self) -> BinaryOperatorKind;
    fn unary_opcode(&self) -> UnaryOperatorKind;
    fn get_int_value(&self) -> u64;
    fn get_float_value(&self) -> f64;

    fn storage(&self) -> CX_StorageClass ;

    fn sizeof_arg_type(&self) -> Option<Type>;
}

impl CursorExt for Cursor {
    fn map<T, F: FnMut(Cursor) -> T>(&self, cb: F) -> iter::Map<vec::IntoIter<Self>, F> {
        self.collect_children().into_iter().map(cb)
    }

    /// Collect all of this cursor's children into a vec and return them.
    fn map_children<T, F: FnMut(Cursor) -> T>(&self, mut cb: F) -> Vec<T> {
        let mut children = vec![];
        self.visit(|c| {
            children.push(cb(c));
            CXChildVisit_Continue
        });
        children
    }

    fn map_children_err<T,F: FnMut(Cursor) -> Res<T>>(&self, mut cb: F) -> Res<Vec<T>> {
        let mut children = vec![];
        let mut err = None;
        self.visit(|c| {
            match cb(c) {
                Ok(c) => {
                    children.push(c);
                    CXChildVisit_Continue
                }
                Err(e) => {
                    err = Some(e);
                    CXChildVisit_Break
                }
            }
        });
        if let Some(e) = err {
            Err(e)
        } else {
            Ok(children)
        }
    }

    fn first_child(&self) -> Option<Cursor> {
        let mut r = None;
        self.visit(|c| {
            r = Some(c);
            CXChildVisit_Break
        });
        r
    }

    fn file(&self) -> File {
        let (file, ..) = self.location().location();
        file
    }

    fn loc(&self) -> Loc {
        Loc::new(self.extent())
    }

    fn limit_range(range: &mut CXSourceRange, cur: &Cursor) {
        cur.visit(|child_cur|{
            let sub = child_cur.extent();
            // println!(" + subrange! {:?} = {}-{} ({:?})", self.kind(), sub.begin_int_data, sub.end_int_data, sub.locations());
            if sub.begin_int_data > range.begin_int_data {
                range.end_int_data = min(range.end_int_data, sub.begin_int_data);
            }
            if sub.end_int_data < range.end_int_data {
                range.begin_int_data = max(range.begin_int_data, sub.end_int_data);
            }
            Self::limit_range(range, &child_cur);
            CXChildVisit_Continue
        });
    }

    fn is_function_macro(&self, tu: &TranslationUnit) -> bool {
        if let Some(t) = tu.tokens(self) {
            if let Some(t) = t.get(1) {
                return t.spelling == "("; // FIXME: that's bad approximation, read C++ API
            }
        }
        false
    }

    fn is_anon(&self) -> bool {
        unsafe {
            clang_Cursor_isAnonymous(self.x) != 0
        }
    }

    fn new_cur(x: CXCursor) -> Option<Cursor> {
        let cur = Cursor {x};
        if cur.is_valid() {
            Some(cur)
        } else {
            None
        }
    }

    fn for_body(&self) -> Cursor {
        unsafe { Cursor {
            x: c3_ForStmt_getBody(self.x)
        } }
    }

    fn var_init(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_VarDecl_getInit(self.x)) }
    }

    fn for_cond(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_ForStmt_getCond(self.x)) }
    }

    fn for_inc(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_ForStmt_getInc(self.x)) }
    }

    fn for_init(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_ForStmt_getInit(self.x)) }
    }

    fn if_else(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_IfStmt_getElse(self.x)) }
    }

    fn if_then(&self) -> Cursor {
        unsafe { Cursor { x: c3_IfStmt_getThen(self.x) } }
    }

    fn if_cond(&self) -> Cursor {
        unsafe { Cursor { x: c3_IfStmt_getCond(self.x) } }
    }

    fn if_condition_variable(&self) -> Option<Cursor> {
        unsafe { Self::new_cur(c3_IfStmt_getConditionVariable(self.x)) }
    }

    fn sub_expr(&self) -> Option<Cursor> {
        unsafe {
            Self::new_cur(c3_Cursor_getSubExpr(self.x))
                .or_else(||self.first_child())
        }
    }

    fn binary_opcode(&self) -> BinaryOperatorKind {
        unsafe {
            c3_Cursor_getBinaryOpcode(self.x)
        }
    }

    fn unary_opcode(&self) -> UnaryOperatorKind {
        unsafe {
            c3_Cursor_getUnaryOpcode(self.x)
        }
    }

    fn get_int_value(&self) -> u64 {
        unsafe { c3_Cursor_getIntValue(self.x) }
    }

    fn get_float_value(&self) -> f64 {
        unsafe { c3_Cursor_getFloatValue(self.x) }
    }

    fn sizeof_arg_type(&self) -> Option<Type> {
        let ty = unsafe { Type { x: c3_UnaryExpr_getArgType(self.x) } };
        if ty.is_valid() {
            Some(ty)
        } else {
            None
        }
    }

    fn storage(&self) -> CX_StorageClass {
        unsafe {
            clang_Cursor_getStorageClass(self.x)
        }
    }
}

pub trait RangeLocations {
    fn locations(self) -> (SourceLocation, SourceLocation);
}

impl RangeLocations for CXSourceRange {
    fn locations(self) -> (SourceLocation, SourceLocation) {
        unsafe {
            (SourceLocation {x:clang_getRangeStart(self)}, SourceLocation{x:clang_getRangeEnd(self)})
        }
    }
}

impl Loc {
    pub fn new(ext: CXSourceRange) -> Self {
        fn pos(loc: CXSourceLocation) -> LocPos {
            unsafe {
                let mut line = 0;
                let mut col = 0;
                clang_getFileLocation(loc, ptr::null_mut(), &mut line, &mut col, ptr::null_mut());
                LocPos {
                    line: line as u32, col: col as u32,
                }
            }
        }
        unsafe {
            Loc {
                start: pos(clang_getRangeStart(ext)),
                end: pos(clang_getRangeEnd(ext)),
            }
        }
    }
}

use std::slice;
use std::os::raw::c_uint;

pub fn comment_tokens(tu: &TranslationUnit) -> Option<Vec<::expr::Comment>> {
    let range = tu.cursor().extent();
    let mut tokens = vec![];
    unsafe {
        let mut token_ptr = ptr::null_mut();
        let mut num_tokens: c_uint = 0;
        clang_tokenize(tu.x, range, &mut token_ptr, &mut num_tokens);
        if token_ptr.is_null() {
            return None;
        }

        let token_array = slice::from_raw_parts(token_ptr,
                                                num_tokens as usize);
        for &token in token_array.iter() {
            let kind = clang_getTokenKind(token);
            if kind != CXToken_Comment {
                continue;
            }
            let extent = clang_getTokenExtent(tu.x, token);
            let spelling =
                cxstring_into_string(clang_getTokenSpelling(tu.x, token));
            tokens.push(::expr::Comment {
                syntax: spelling,
                loc: Loc::new(extent),
            });
        }
        clang_disposeTokens(tu.x, token_ptr, num_tokens);
    }
    Some(tokens)
}
