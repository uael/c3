#![allow(non_camel_case_types)]

extern crate clang_sys;
use clang_sys::{CXCursor, CXType};

include!("cursorkind.rs.c");
include!("cursortype.rs.c");

extern "C" {
    pub fn c3_Cursor_getBinaryOpcode(cur: CXCursor) -> BinaryOperatorKind;
    pub fn c3_Cursor_getUnaryOpcode(cur: CXCursor) -> UnaryOperatorKind;
    pub fn c3_Cursor_getIntValue(cur: CXCursor) -> u64;
    pub fn c3_Cursor_getFloatValue(cur: CXCursor) -> f64;
    pub fn c3_Cursor_getKindExt(cur: CXCursor) -> CusorKindExt;
    pub fn c3_CursorType_getKindExt(ty: CXType) -> ClangTypeExt;
    pub fn c3_Cursor_getSubExpr(cur: CXCursor) -> CXCursor;

    pub fn c3_ForStmt_getBody(cur: CXCursor) -> CXCursor;
    pub fn c3_ForStmt_getCond(cur: CXCursor) -> CXCursor;
    pub fn c3_ForStmt_getInc(cur: CXCursor) -> CXCursor;
    pub fn c3_ForStmt_getInit(cur: CXCursor) -> CXCursor;

    pub fn c3_IfStmt_getElse(cur: CXCursor) -> CXCursor;
    pub fn c3_IfStmt_getThen(cur: CXCursor) -> CXCursor;
    pub fn c3_IfStmt_getCond(cur: CXCursor) -> CXCursor;
    pub fn c3_IfStmt_getConditionVariable(cur: CXCursor) -> CXCursor;

    pub fn c3_UnaryExpr_getArgType(cur: CXCursor) -> CXType;

    pub fn c3_VarDecl_getInit(cur: CXCursor) -> CXCursor;
}

#[repr(u32)]
/// CastKind - The kind of operation required for a conversion.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CastKind {
    CK_Dependent = 0,
    CK_BitCast = 1,
    CK_LValueBitCast = 2,
    CK_LValueToRValue = 3,
    CK_NoOp = 4,
    CK_BaseToDerived = 5,
    CK_DerivedToBase = 6,
    CK_UncheckedDerivedToBase = 7,
    CK_Dynamic = 8,
    CK_ToUnion = 9,
    CK_ArrayToPointerDecay = 10,
    CK_FunctionToPointerDecay = 11,
    CK_NullToPointer = 12,
    CK_NullToMemberPointer = 13,
    CK_BaseToDerivedMemberPointer = 14,
    CK_DerivedToBaseMemberPointer = 15,
    CK_MemberPointerToBoolean = 16,
    CK_ReinterpretMemberPointer = 17,
    CK_UserDefinedConversion = 18,
    CK_ConstructorConversion = 19,
    CK_IntegralToPointer = 20,
    CK_PointerToIntegral = 21,
    CK_PointerToBoolean = 22,
    CK_ToVoid = 23,
    CK_VectorSplat = 24,
    CK_IntegralCast = 25,
    CK_IntegralToBoolean = 26,
    CK_IntegralToFloating = 27,
    CK_FloatingToIntegral = 28,
    CK_FloatingToBoolean = 29,
    CK_BooleanToSignedIntegral = 30,
    CK_FloatingCast = 31,
    CK_CPointerToObjCPointerCast = 32,
    CK_BlockPointerToObjCPointerCast = 33,
    CK_AnyPointerToBlockPointerCast = 34,
    CK_ObjCObjectLValueCast = 35,
    CK_FloatingRealToComplex = 36,
    CK_FloatingComplexToReal = 37,
    CK_FloatingComplexToBoolean = 38,
    CK_FloatingComplexCast = 39,
    CK_FloatingComplexToIntegralComplex = 40,
    CK_IntegralRealToComplex = 41,
    CK_IntegralComplexToReal = 42,
    CK_IntegralComplexToBoolean = 43,
    CK_IntegralComplexCast = 44,
    CK_IntegralComplexToFloatingComplex = 45,
    CK_ARCProduceObject = 46,
    CK_ARCConsumeObject = 47,
    CK_ARCReclaimReturnedObject = 48,
    CK_ARCExtendBlockObject = 49,
    CK_AtomicToNonAtomic = 50,
    CK_NonAtomicToAtomic = 51,
    CK_CopyAndAutoreleaseBlockObject = 52,
    CK_BuiltinFnToFnPtr = 53,
    CK_ZeroToOCLEvent = 54,
    CK_ZeroToOCLQueue = 55,
    CK_AddressSpaceConversion = 56,
    CK_IntToOCLSampler = 57,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperatorKind {
    BO_PtrMemD = 0,
    BO_PtrMemI = 1,
    BO_Mul = 2,
    BO_Div = 3,
    BO_Rem = 4,
    BO_Add = 5,
    BO_Sub = 6,
    BO_Shl = 7,
    BO_Shr = 8,
    BO_LT = 9,
    BO_GT = 10,
    BO_LE = 11,
    BO_GE = 12,
    BO_EQ = 13,
    BO_NE = 14,
    BO_And = 15,
    BO_Xor = 16,
    BO_Or = 17,
    BO_LAnd = 18,
    BO_LOr = 19,
    BO_Assign = 20,
    BO_MulAssign = 21,
    BO_DivAssign = 22,
    BO_RemAssign = 23,
    BO_AddAssign = 24,
    BO_SubAssign = 25,
    BO_ShlAssign = 26,
    BO_ShrAssign = 27,
    BO_AndAssign = 28,
    BO_XorAssign = 29,
    BO_OrAssign = 30,
    BO_Comma = 31,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperatorKind {
    UO_PostInc = 0,
    UO_PostDec = 1,
    UO_PreInc = 2,
    UO_PreDec = 3,
    UO_AddrOf = 4,
    UO_Deref = 5,
    UO_Plus = 6,
    UO_Minus = 7,
    UO_Not = 8,
    UO_LNot = 9,
    UO_Real = 10,
    UO_Imag = 11,
    UO_Extension = 12,
    UO_Coawait = 13,
}

#[repr(u32)]
/// \brief The kind of bridging performed by the Objective-C bridge cast.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ObjCBridgeCastKind {
    OBC_Bridge = 0,
    OBC_BridgeTransfer = 1,
    OBC_BridgeRetained = 2,
}
