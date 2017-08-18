/* These are functions copied from Clang/LLVM 4
  Obviously, dependent on unstable internals.

==============================================================================
LLVM Release License
==============================================================================
University of Illinois/NCSA
Open Source License

Copyright (c) 2007-2016 University of Illinois at Urbana-Champaign.
All rights reserved.

Developed by:

    LLVM Team

    University of Illinois at Urbana-Champaign

    http://llvm.org

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal with
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

    * Redistributions of source code must retain the above copyright notice,
      this list of conditions and the following disclaimers.

    * Redistributions in binary form must reproduce the above copyright notice,
      this list of conditions and the following disclaimers in the
      documentation and/or other materials provided with the distribution.

    * Neither the names of the LLVM Team, University of Illinois at
      Urbana-Champaign, nor the names of its contributors may be used to
      endorse or promote products derived from this Software without specific
      prior written permission.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
CONTRIBUTORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS WITH THE
SOFTWARE.

 */

using namespace clang;

static const clang::Stmt *getCursorStmt(CXCursor cursor) {
    return static_cast<const clang::Stmt *>(cursor.data[1]);
}

static const clang::Expr *getCursorExpr(CXCursor cursor) {
  return clang::dyn_cast_or_null<clang::Expr>(getCursorStmt(cursor));
}

static const clang::UnaryExprOrTypeTraitExpr *getCursorUnaryExprOrTypeTrait(CXCursor cursor) {
  return cast<clang::UnaryExprOrTypeTraitExpr>(getCursorExpr(cursor));
}

static const clang::Attr *getCursorAttr(CXCursor cursor) {
  return static_cast<const clang::Attr *>(cursor.data[1]);;
}

static const clang::ForStmt *getForStmt(CXCursor cursor) {
    return static_cast<const clang::ForStmt *>(getCursorStmt(cursor));
}

static const clang::IfStmt *getIfStmt(CXCursor cursor) {
    return static_cast<const clang::IfStmt *>(getCursorStmt(cursor));
}

static CXTranslationUnit getCursorTU(CXCursor Cursor) {
  return static_cast<CXTranslationUnit>(const_cast<void*>(Cursor.data[2]));
}

static const clang::Decl *getCursorDecl(CXCursor Cursor) {
  return static_cast<const clang::Decl *>(Cursor.data[0]);
}

static CXTranslationUnit GetTU(CXType CT) {
  return static_cast<CXTranslationUnit>(CT.data[1]);
}

static QualType GetQualType(CXType CT) {
  return QualType::getFromOpaquePtr(CT.data[0]);
}

static CXTypeKind GetBuiltinTypeKind(const BuiltinType *BT) {
#define BTCASE(K) case BuiltinType::K: return CXType_##K
  switch (BT->getKind()) {
    BTCASE(Void);
    BTCASE(Bool);
    BTCASE(Char_U);
    BTCASE(UChar);
    BTCASE(Char16);
    BTCASE(Char32);
    BTCASE(UShort);
    BTCASE(UInt);
    BTCASE(ULong);
    BTCASE(ULongLong);
    BTCASE(UInt128);
    BTCASE(Char_S);
    BTCASE(SChar);
    case BuiltinType::WChar_S: return CXType_WChar;
    case BuiltinType::WChar_U: return CXType_WChar;
    BTCASE(Short);
    BTCASE(Int);
    BTCASE(Long);
    BTCASE(LongLong);
    BTCASE(Int128);
    BTCASE(Float);
    BTCASE(Double);
    BTCASE(LongDouble);
    BTCASE(Float128);
    BTCASE(NullPtr);
    BTCASE(Overload);
    BTCASE(Dependent);
    BTCASE(ObjCId);
    BTCASE(ObjCClass);
    BTCASE(ObjCSel);
  default:
    return CXType_Unexposed;
  }
#undef BTCASE
}

static CXTypeKind GetTypeKind(QualType T) {
  const Type *TP = T.getTypePtrOrNull();
  if (!TP)
    return CXType_Invalid;

#define TKCASE(K) case Type::K: return CXType_##K
  switch (TP->getTypeClass()) {
    case Type::Builtin:
      return GetBuiltinTypeKind(cast<BuiltinType>(TP));
    TKCASE(Complex);
    TKCASE(Pointer);
    TKCASE(BlockPointer);
    TKCASE(LValueReference);
    TKCASE(RValueReference);
    TKCASE(Record);
    TKCASE(Enum);
    TKCASE(Typedef);
    TKCASE(ObjCInterface);
    TKCASE(ObjCObjectPointer);
    TKCASE(FunctionNoProto);
    TKCASE(FunctionProto);
    TKCASE(ConstantArray);
    TKCASE(IncompleteArray);
    TKCASE(VariableArray);
    TKCASE(DependentSizedArray);
    TKCASE(Vector);
    TKCASE(MemberPointer);
    TKCASE(Auto);
    TKCASE(Elaborated);
    default:
      return CXType_Unexposed;
  }
#undef TKCASE
}

static CXType MakeCXType(QualType T, CXTranslationUnit TU) {
  CXTypeKind TK = CXType_Invalid;

  if (TU && !T.isNull()) {
    // Handle attributed types as the original type
    if (auto *ATT = T->getAs<AttributedType>()) {
      return MakeCXType(ATT->getModifiedType(), TU);
    }

    /* Handle decayed types as the original type */
    if (const DecayedType *DT = T->getAs<DecayedType>()) {
      return MakeCXType(DT->getOriginalType(), TU);
    }
  }
  if (TK == CXType_Invalid)
    TK = GetTypeKind(T);

  CXType CT = { TK, { TK == CXType_Invalid ? nullptr
                                           : T.getAsOpaquePtr(), TU } };
  return CT;
}

static CXType MakeNullType() {
    CXType CT = { CXType_Invalid, { nullptr, nullptr } };
    return CT;
}

static CXCursor MakeCXCursor(const Stmt *S, const Decl *Parent, CXTranslationUnit TU, SourceRange RegionOfInterest) {
  if (!S) return clang_getNullCursor();
  CXCursorKind K = CXCursor_NotImplemented;

  switch (S->getStmtClass()) {
  case Stmt::NoStmtClass:
    break;

  case Stmt::CaseStmtClass:
    K = CXCursor_CaseStmt;
    break;

  case Stmt::DefaultStmtClass:
    K = CXCursor_DefaultStmt;
    break;

  case Stmt::IfStmtClass:
    K = CXCursor_IfStmt;
    break;

  case Stmt::SwitchStmtClass:
    K = CXCursor_SwitchStmt;
    break;

  case Stmt::WhileStmtClass:
    K = CXCursor_WhileStmt;
    break;

  case Stmt::DoStmtClass:
    K = CXCursor_DoStmt;
    break;

  case Stmt::ForStmtClass:
    K = CXCursor_ForStmt;
    break;

  case Stmt::GotoStmtClass:
    K = CXCursor_GotoStmt;
    break;

  case Stmt::IndirectGotoStmtClass:
    K = CXCursor_IndirectGotoStmt;
    break;

  case Stmt::ContinueStmtClass:
    K = CXCursor_ContinueStmt;
    break;

  case Stmt::BreakStmtClass:
    K = CXCursor_BreakStmt;
    break;

  case Stmt::ReturnStmtClass:
    K = CXCursor_ReturnStmt;
    break;

  case Stmt::GCCAsmStmtClass:
    K = CXCursor_GCCAsmStmt;
    break;

  case Stmt::MSAsmStmtClass:
    K = CXCursor_MSAsmStmt;
    break;

  case Stmt::ObjCAtTryStmtClass:
    K = CXCursor_ObjCAtTryStmt;
    break;

  case Stmt::ObjCAtCatchStmtClass:
    K = CXCursor_ObjCAtCatchStmt;
    break;

  case Stmt::ObjCAtFinallyStmtClass:
    K = CXCursor_ObjCAtFinallyStmt;
    break;

  case Stmt::ObjCAtThrowStmtClass:
    K = CXCursor_ObjCAtThrowStmt;
    break;

  case Stmt::ObjCAtSynchronizedStmtClass:
    K = CXCursor_ObjCAtSynchronizedStmt;
    break;

  case Stmt::ObjCAutoreleasePoolStmtClass:
    K = CXCursor_ObjCAutoreleasePoolStmt;
    break;

  case Stmt::ObjCForCollectionStmtClass:
    K = CXCursor_ObjCForCollectionStmt;
    break;

  case Stmt::CXXCatchStmtClass:
    K = CXCursor_CXXCatchStmt;
    break;

  case Stmt::CXXTryStmtClass:
    K = CXCursor_CXXTryStmt;
    break;

  case Stmt::CXXForRangeStmtClass:
    K = CXCursor_CXXForRangeStmt;
    break;

  case Stmt::SEHTryStmtClass:
    K = CXCursor_SEHTryStmt;
    break;

  case Stmt::SEHExceptStmtClass:
    K = CXCursor_SEHExceptStmt;
    break;

  case Stmt::SEHFinallyStmtClass:
    K = CXCursor_SEHFinallyStmt;
    break;

  case Stmt::SEHLeaveStmtClass:
    K = CXCursor_SEHLeaveStmt;
    break;

  case Stmt::ArrayTypeTraitExprClass:
  case Stmt::AsTypeExprClass:
  case Stmt::AtomicExprClass:
  case Stmt::BinaryConditionalOperatorClass:
  case Stmt::TypeTraitExprClass:
  case Stmt::CoroutineBodyStmtClass:
  case Stmt::CoawaitExprClass:
  case Stmt::CoreturnStmtClass:
  case Stmt::CoyieldExprClass:
  case Stmt::CXXBindTemporaryExprClass:
  case Stmt::CXXDefaultArgExprClass:
  case Stmt::CXXDefaultInitExprClass:
  case Stmt::CXXFoldExprClass:
  case Stmt::CXXStdInitializerListExprClass:
  case Stmt::CXXScalarValueInitExprClass:
  case Stmt::CXXUuidofExprClass:
  case Stmt::ChooseExprClass:
  case Stmt::DesignatedInitExprClass:
  case Stmt::DesignatedInitUpdateExprClass:
  case Stmt::ArrayInitLoopExprClass:
  case Stmt::ArrayInitIndexExprClass:
  case Stmt::ExprWithCleanupsClass:
  case Stmt::ExpressionTraitExprClass:
  case Stmt::ExtVectorElementExprClass:
  case Stmt::ImplicitCastExprClass:
  case Stmt::ImplicitValueInitExprClass:
  case Stmt::NoInitExprClass:
  case Stmt::MaterializeTemporaryExprClass:
  case Stmt::ObjCIndirectCopyRestoreExprClass:
  case Stmt::OffsetOfExprClass:
  case Stmt::ParenListExprClass:
  case Stmt::PredefinedExprClass:
  case Stmt::ShuffleVectorExprClass:
  case Stmt::ConvertVectorExprClass:
  case Stmt::VAArgExprClass:
  case Stmt::ObjCArrayLiteralClass:
  case Stmt::ObjCDictionaryLiteralClass:
  case Stmt::ObjCBoxedExprClass:
  case Stmt::ObjCSubscriptRefExprClass:
    K = CXCursor_UnexposedExpr;
    break;

  case Stmt::OpaqueValueExprClass:
    if (Expr *Src = cast<OpaqueValueExpr>(S)->getSourceExpr())
      return MakeCXCursor(Src, Parent, TU, RegionOfInterest);
    K = CXCursor_UnexposedExpr;
    break;

  case Stmt::PseudoObjectExprClass:
    return MakeCXCursor(cast<PseudoObjectExpr>(S)->getSyntacticForm(),
                        Parent, TU, RegionOfInterest);

  case Stmt::CompoundStmtClass:
    K = CXCursor_CompoundStmt;
    break;

  case Stmt::NullStmtClass:
    K = CXCursor_NullStmt;
    break;

  case Stmt::LabelStmtClass:
    K = CXCursor_LabelStmt;
    break;

  case Stmt::AttributedStmtClass:
    K = CXCursor_UnexposedStmt;
    break;

  case Stmt::DeclStmtClass:
    K = CXCursor_DeclStmt;
    break;

  case Stmt::CapturedStmtClass:
    K = CXCursor_UnexposedStmt;
    break;

  case Stmt::IntegerLiteralClass:
    K = CXCursor_IntegerLiteral;
    break;

  case Stmt::FloatingLiteralClass:
    K = CXCursor_FloatingLiteral;
    break;

  case Stmt::ImaginaryLiteralClass:
    K = CXCursor_ImaginaryLiteral;
    break;

  case Stmt::StringLiteralClass:
    K = CXCursor_StringLiteral;
    break;

  case Stmt::CharacterLiteralClass:
    K = CXCursor_CharacterLiteral;
    break;

  case Stmt::ParenExprClass:
    K = CXCursor_ParenExpr;
    break;

  case Stmt::UnaryOperatorClass:
    K = CXCursor_UnaryOperator;
    break;

  case Stmt::UnaryExprOrTypeTraitExprClass:
  case Stmt::CXXNoexceptExprClass:
    K = CXCursor_UnaryExpr;
    break;

  case Stmt::MSPropertySubscriptExprClass:
  case Stmt::ArraySubscriptExprClass:
    K = CXCursor_ArraySubscriptExpr;
    break;

  case Stmt::OMPArraySectionExprClass:
    K = CXCursor_OMPArraySectionExpr;
    break;

  case Stmt::BinaryOperatorClass:
    K = CXCursor_BinaryOperator;
    break;

  case Stmt::CompoundAssignOperatorClass:
    K = CXCursor_CompoundAssignOperator;
    break;

  case Stmt::ConditionalOperatorClass:
    K = CXCursor_ConditionalOperator;
    break;

  case Stmt::CStyleCastExprClass:
    K = CXCursor_CStyleCastExpr;
    break;

  case Stmt::CompoundLiteralExprClass:
    K = CXCursor_CompoundLiteralExpr;
    break;

  case Stmt::InitListExprClass:
    K = CXCursor_InitListExpr;
    break;

  case Stmt::AddrLabelExprClass:
    K = CXCursor_AddrLabelExpr;
    break;

  case Stmt::StmtExprClass:
    K = CXCursor_StmtExpr;
    break;

  case Stmt::GenericSelectionExprClass:
    K = CXCursor_GenericSelectionExpr;
    break;

  case Stmt::GNUNullExprClass:
    K = CXCursor_GNUNullExpr;
    break;

  case Stmt::CXXStaticCastExprClass:
    K = CXCursor_CXXStaticCastExpr;
    break;

  case Stmt::CXXDynamicCastExprClass:
    K = CXCursor_CXXDynamicCastExpr;
    break;

  case Stmt::CXXReinterpretCastExprClass:
    K = CXCursor_CXXReinterpretCastExpr;
    break;

  case Stmt::CXXConstCastExprClass:
    K = CXCursor_CXXConstCastExpr;
    break;

  case Stmt::CXXFunctionalCastExprClass:
    K = CXCursor_CXXFunctionalCastExpr;
    break;

  case Stmt::CXXTypeidExprClass:
    K = CXCursor_CXXTypeidExpr;
    break;

  case Stmt::CXXBoolLiteralExprClass:
    K = CXCursor_CXXBoolLiteralExpr;
    break;

  case Stmt::CXXNullPtrLiteralExprClass:
    K = CXCursor_CXXNullPtrLiteralExpr;
    break;

  case Stmt::CXXThisExprClass:
    K = CXCursor_CXXThisExpr;
    break;

  case Stmt::CXXThrowExprClass:
    K = CXCursor_CXXThrowExpr;
    break;

  case Stmt::CXXNewExprClass:
    K = CXCursor_CXXNewExpr;
    break;

  case Stmt::CXXDeleteExprClass:
    K = CXCursor_CXXDeleteExpr;
    break;

  case Stmt::ObjCStringLiteralClass:
    K = CXCursor_ObjCStringLiteral;
    break;

  case Stmt::ObjCEncodeExprClass:
    K = CXCursor_ObjCEncodeExpr;
    break;

  case Stmt::ObjCSelectorExprClass:
    K = CXCursor_ObjCSelectorExpr;
    break;

  case Stmt::ObjCProtocolExprClass:
    K = CXCursor_ObjCProtocolExpr;
    break;

  case Stmt::ObjCBoolLiteralExprClass:
    K = CXCursor_ObjCBoolLiteralExpr;
    break;

  case Stmt::ObjCAvailabilityCheckExprClass:
    K = CXCursor_ObjCAvailabilityCheckExpr;
    break;

  case Stmt::ObjCBridgedCastExprClass:
    K = CXCursor_ObjCBridgedCastExpr;
    break;

  case Stmt::BlockExprClass:
    K = CXCursor_BlockExpr;
    break;

  case Stmt::PackExpansionExprClass:
    K = CXCursor_PackExpansionExpr;
    break;

  case Stmt::SizeOfPackExprClass:
    K = CXCursor_SizeOfPackExpr;
    break;


  case Stmt::DependentScopeDeclRefExprClass:
  case Stmt::SubstNonTypeTemplateParmExprClass:
  case Stmt::SubstNonTypeTemplateParmPackExprClass:
  case Stmt::FunctionParmPackExprClass:
  case Stmt::UnresolvedLookupExprClass:
  case Stmt::TypoExprClass: // A typo could actually be a DeclRef or a MemberRef
    K = CXCursor_DeclRefExpr;
    break;

  case Stmt::CXXDependentScopeMemberExprClass:
  case Stmt::CXXPseudoDestructorExprClass:
  case Stmt::MemberExprClass:
  case Stmt::MSPropertyRefExprClass:
  case Stmt::ObjCIsaExprClass:
  case Stmt::ObjCIvarRefExprClass:
  case Stmt::ObjCPropertyRefExprClass:
  case Stmt::UnresolvedMemberExprClass:
    K = CXCursor_MemberRefExpr;
    break;

  case Stmt::CallExprClass:
  case Stmt::CXXOperatorCallExprClass:
  case Stmt::CXXMemberCallExprClass:
  case Stmt::CUDAKernelCallExprClass:
  case Stmt::CXXConstructExprClass:
  case Stmt::CXXInheritedCtorInitExprClass:
  case Stmt::CXXTemporaryObjectExprClass:
  case Stmt::CXXUnresolvedConstructExprClass:
  case Stmt::UserDefinedLiteralClass:
    K = CXCursor_CallExpr;
    break;

  case Stmt::LambdaExprClass:
    K = CXCursor_LambdaExpr;
    break;

  case Stmt::MSDependentExistsStmtClass:
    K = CXCursor_UnexposedStmt;
    break;
  case Stmt::OMPParallelDirectiveClass:
    K = CXCursor_OMPParallelDirective;
    break;
  case Stmt::OMPSimdDirectiveClass:
    K = CXCursor_OMPSimdDirective;
    break;
  case Stmt::OMPForDirectiveClass:
    K = CXCursor_OMPForDirective;
    break;
  case Stmt::OMPForSimdDirectiveClass:
    K = CXCursor_OMPForSimdDirective;
    break;
  case Stmt::OMPSectionsDirectiveClass:
    K = CXCursor_OMPSectionsDirective;
    break;
  case Stmt::OMPSectionDirectiveClass:
    K = CXCursor_OMPSectionDirective;
    break;
  case Stmt::OMPSingleDirectiveClass:
    K = CXCursor_OMPSingleDirective;
    break;
  case Stmt::OMPMasterDirectiveClass:
    K = CXCursor_OMPMasterDirective;
    break;
  case Stmt::OMPCriticalDirectiveClass:
    K = CXCursor_OMPCriticalDirective;
    break;
  case Stmt::OMPParallelForDirectiveClass:
    K = CXCursor_OMPParallelForDirective;
    break;
  case Stmt::OMPParallelForSimdDirectiveClass:
    K = CXCursor_OMPParallelForSimdDirective;
    break;
  case Stmt::OMPParallelSectionsDirectiveClass:
    K = CXCursor_OMPParallelSectionsDirective;
    break;
  case Stmt::OMPTaskDirectiveClass:
    K = CXCursor_OMPTaskDirective;
    break;
  case Stmt::OMPTaskyieldDirectiveClass:
    K = CXCursor_OMPTaskyieldDirective;
    break;
  case Stmt::OMPBarrierDirectiveClass:
    K = CXCursor_OMPBarrierDirective;
    break;
  case Stmt::OMPTaskwaitDirectiveClass:
    K = CXCursor_OMPTaskwaitDirective;
    break;
  case Stmt::OMPTaskgroupDirectiveClass:
    K = CXCursor_OMPTaskgroupDirective;
    break;
  case Stmt::OMPFlushDirectiveClass:
    K = CXCursor_OMPFlushDirective;
    break;
  case Stmt::OMPOrderedDirectiveClass:
    K = CXCursor_OMPOrderedDirective;
    break;
  case Stmt::OMPAtomicDirectiveClass:
    K = CXCursor_OMPAtomicDirective;
    break;
  case Stmt::OMPTargetDirectiveClass:
    K = CXCursor_OMPTargetDirective;
    break;
  case Stmt::OMPTargetDataDirectiveClass:
    K = CXCursor_OMPTargetDataDirective;
    break;
  case Stmt::OMPTargetEnterDataDirectiveClass:
    K = CXCursor_OMPTargetEnterDataDirective;
    break;
  case Stmt::OMPTargetExitDataDirectiveClass:
    K = CXCursor_OMPTargetExitDataDirective;
    break;
  case Stmt::OMPTargetParallelDirectiveClass:
    K = CXCursor_OMPTargetParallelDirective;
    break;
  case Stmt::OMPTargetParallelForDirectiveClass:
    K = CXCursor_OMPTargetParallelForDirective;
    break;
  case Stmt::OMPTargetUpdateDirectiveClass:
    K = CXCursor_OMPTargetUpdateDirective;
    break;
  case Stmt::OMPTeamsDirectiveClass:
    K = CXCursor_OMPTeamsDirective;
    break;
  case Stmt::OMPCancellationPointDirectiveClass:
    K = CXCursor_OMPCancellationPointDirective;
    break;
  case Stmt::OMPCancelDirectiveClass:
    K = CXCursor_OMPCancelDirective;
    break;
  case Stmt::OMPTaskLoopDirectiveClass:
    K = CXCursor_OMPTaskLoopDirective;
    break;
  case Stmt::OMPTaskLoopSimdDirectiveClass:
    K = CXCursor_OMPTaskLoopSimdDirective;
    break;
  case Stmt::OMPDistributeDirectiveClass:
    K = CXCursor_OMPDistributeDirective;
    break;
  case Stmt::OMPDistributeParallelForDirectiveClass:
    K = CXCursor_OMPDistributeParallelForDirective;
    break;
  case Stmt::OMPDistributeParallelForSimdDirectiveClass:
    K = CXCursor_OMPDistributeParallelForSimdDirective;
    break;
  case Stmt::OMPDistributeSimdDirectiveClass:
    K = CXCursor_OMPDistributeSimdDirective;
    break;
  case Stmt::OMPTargetParallelForSimdDirectiveClass:
    K = CXCursor_OMPTargetParallelForSimdDirective;
    break;
  case Stmt::OMPTargetSimdDirectiveClass:
    K = CXCursor_OMPTargetSimdDirective;
    break;
  case Stmt::OMPTeamsDistributeDirectiveClass:
    K = CXCursor_OMPTeamsDistributeDirective;
    break;
  case Stmt::OMPTeamsDistributeSimdDirectiveClass:
    K = CXCursor_OMPTeamsDistributeSimdDirective;
    break;
  case Stmt::OMPTeamsDistributeParallelForSimdDirectiveClass:
    K = CXCursor_OMPTeamsDistributeParallelForSimdDirective;
    break;
  case Stmt::OMPTeamsDistributeParallelForDirectiveClass:
    K = CXCursor_OMPTeamsDistributeParallelForDirective;
    break;
  case Stmt::OMPTargetTeamsDirectiveClass:
    K = CXCursor_OMPTargetTeamsDirective;
    break;
  case Stmt::OMPTargetTeamsDistributeDirectiveClass:
    K = CXCursor_OMPTargetTeamsDistributeDirective;
    break;
  case Stmt::OMPTargetTeamsDistributeParallelForDirectiveClass:
    K = CXCursor_OMPTargetTeamsDistributeParallelForDirective;
    break;
  case Stmt::OMPTargetTeamsDistributeParallelForSimdDirectiveClass:
    K = CXCursor_OMPTargetTeamsDistributeParallelForSimdDirective;
    break;
  case Stmt::OMPTargetTeamsDistributeSimdDirectiveClass:
    K = CXCursor_OMPTargetTeamsDistributeSimdDirective;
    break;
    default:
    K = CXCursor_NotImplemented;
  }

  CXCursor C = { K, 0, { Parent, S, TU } };
  return C;
}

static CXCursorKind getCursorKindForDecl(const Decl *D) {
  if (!D)
    return CXCursor_UnexposedDecl;

  switch (D->getKind()) {
    case Decl::Enum:               return CXCursor_EnumDecl;
    case Decl::EnumConstant:       return CXCursor_EnumConstantDecl;
    case Decl::Field:              return CXCursor_FieldDecl;
    case Decl::Function:
      return CXCursor_FunctionDecl;
    case Decl::ObjCCategory:       return CXCursor_ObjCCategoryDecl;
    case Decl::ObjCCategoryImpl:   return CXCursor_ObjCCategoryImplDecl;
    case Decl::ObjCImplementation: return CXCursor_ObjCImplementationDecl;

    case Decl::ObjCInterface:      return CXCursor_ObjCInterfaceDecl;
    case Decl::ObjCIvar:           return CXCursor_ObjCIvarDecl;
    case Decl::CXXMethod:          return CXCursor_CXXMethod;
    case Decl::CXXConstructor:     return CXCursor_Constructor;
    case Decl::CXXDestructor:      return CXCursor_Destructor;
    case Decl::CXXConversion:      return CXCursor_ConversionFunction;
    case Decl::ObjCProperty:       return CXCursor_ObjCPropertyDecl;
    case Decl::ObjCProtocol:       return CXCursor_ObjCProtocolDecl;
    case Decl::ParmVar:            return CXCursor_ParmDecl;
    case Decl::Typedef:            return CXCursor_TypedefDecl;
    case Decl::TypeAlias:          return CXCursor_TypeAliasDecl;
    case Decl::TypeAliasTemplate:  return CXCursor_TypeAliasTemplateDecl;
    case Decl::Var:                return CXCursor_VarDecl;
    case Decl::Namespace:          return CXCursor_Namespace;
    case Decl::NamespaceAlias:     return CXCursor_NamespaceAlias;
    case Decl::TemplateTypeParm:   return CXCursor_TemplateTypeParameter;
    case Decl::NonTypeTemplateParm:return CXCursor_NonTypeTemplateParameter;
    case Decl::TemplateTemplateParm:return CXCursor_TemplateTemplateParameter;
    case Decl::FunctionTemplate:   return CXCursor_FunctionTemplate;
    case Decl::ClassTemplate:      return CXCursor_ClassTemplate;
    case Decl::AccessSpec:         return CXCursor_CXXAccessSpecifier;
    case Decl::ClassTemplatePartialSpecialization:
      return CXCursor_ClassTemplatePartialSpecialization;
    case Decl::UsingDirective:     return CXCursor_UsingDirective;
    case Decl::StaticAssert:       return CXCursor_StaticAssert;
    case Decl::Friend:             return CXCursor_FriendDecl;
    case Decl::TranslationUnit:    return CXCursor_TranslationUnit;

    case Decl::Using:
    case Decl::UnresolvedUsingValue:
    case Decl::UnresolvedUsingTypename:
      return CXCursor_UsingDeclaration;

      case Decl::Import:
        return CXCursor_ModuleImportDecl;

    case Decl::ObjCTypeParam:   return CXCursor_TemplateTypeParameter;

    default:
      if (const TagDecl *TD = dyn_cast<TagDecl>(D)) {
        switch (TD->getTagKind()) {
          case TTK_Interface:  // fall through
          case TTK_Struct: return CXCursor_StructDecl;
          case TTK_Class:  return CXCursor_ClassDecl;
          case TTK_Union:  return CXCursor_UnionDecl;
          case TTK_Enum:   return CXCursor_EnumDecl;
        }
      }
  }

  return CXCursor_UnexposedDecl;
}

static CXCursor MakeCXCursor(const Decl *D, CXTranslationUnit TU, SourceRange RegionOfInterest, bool FirstInDeclGroup) {
  if (!D) {
    return clang_getNullCursor();
  }

  CXCursorKind K = getCursorKindForDecl(D);

  CXCursor C = { K, 0, { D, (void*)(intptr_t) (FirstInDeclGroup ? 1 : 0), TU }};
  return C;
}
