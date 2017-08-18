/** These are hastily-written hacks to work around limitations of libclang */

#include <math.h>
#include "clang/AST/OperationKinds.h"
#include "clang-c/Index.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang_copy.cpp"

#include "cursorkind.rs.c"
;

#include "cursortype.rs.c"
;

/** get cursor to an initializer of cursor pointing to VarDecl */
extern "C" CXCursor c3_VarDecl_getInit(CXCursor cursor) {
    auto D = static_cast<const clang::VarDecl *>(getCursorDecl(cursor));
    if (!D) {
        return clang_getNullCursor();
    }
    auto e = D->getInit();
    if (!e) {
        return clang_getNullCursor();
    }
    return MakeCXCursor(e, getCursorDecl(cursor), getCursorTU(cursor), e->getSourceRange());
}

/** get cursor to else statement of cursor pointing to IfStmt */
extern "C" CXCursor c3_IfStmt_getElse(CXCursor cursor) {
    const clang::IfStmt *f = getIfStmt(cursor);
    return MakeCXCursor(f->getElse(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

extern "C" CXCursor c3_IfStmt_getThen(CXCursor cursor) {
    const clang::IfStmt *f = getIfStmt(cursor);
    return MakeCXCursor(f->getThen(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

extern "C" CXCursor c3_IfStmt_getCond(CXCursor cursor) {
    const clang::IfStmt *f = getIfStmt(cursor);
    return MakeCXCursor(f->getCond(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

/**
 * For C++ if (int x = )
 */
extern "C" CXCursor c3_IfStmt_getConditionVariable(CXCursor cursor) {
    const clang::IfStmt *f = getIfStmt(cursor);
    return MakeCXCursor(f->getConditionVariable(), getCursorTU(cursor), f->getSourceRange(), true);
}

extern "C" CXCursor c3_ForStmt_getBody(CXCursor cursor) {
    const clang::ForStmt *f = getForStmt(cursor);
    return MakeCXCursor(f->getBody(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

extern "C" CXCursor c3_ForStmt_getCond(CXCursor cursor) {
    const clang::ForStmt *f = getForStmt(cursor);
    return MakeCXCursor(f->getCond(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

extern "C" CXCursor c3_ForStmt_getInc(CXCursor cursor) {
    const clang::ForStmt *f = getForStmt(cursor);
    return MakeCXCursor(f->getInc(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

extern "C" CXCursor c3_ForStmt_getInit(CXCursor cursor) {
    const clang::ForStmt *f = getForStmt(cursor);
    return MakeCXCursor(f->getInit(), getCursorDecl(cursor), getCursorTU(cursor), f->getSourceRange());
}

/** Get type of argument of an unary expression (intended for sizeof) */
extern "C" CXType c3_UnaryExpr_getArgType(CXCursor cursor) {
    if (cursor.kind == CXCursor_UnaryExpr) {
        auto e = getCursorUnaryExprOrTypeTrait(cursor);
        if (e) {
            return MakeCXType(e->getTypeOfArgument(), getCursorTU(cursor));
        }
    }
    return MakeNullType();
}

/**
 * Get actual kind of "Unexposed" cursor kind
 */
extern "C" uint32_t c3_Cursor_getKindExt(CXCursor cursor) {
    if (cursor.kind == CXCursor_UnexposedAttr) {
        switch(getCursorAttr(cursor)->getKind()) {
            case clang::attr::AbiTag: return CusorKindExt::AbiTagAttr;
            case clang::attr::AcquireCapability: return CusorKindExt::AcquireCapabilityAttr;
            case clang::attr::AcquiredAfter: return CusorKindExt::AcquiredAfterAttr;
            case clang::attr::AcquiredBefore: return CusorKindExt::AcquiredBeforeAttr;
            case clang::attr::Alias: return CusorKindExt::AliasAttr;
            case clang::attr::Aligned: return CusorKindExt::AlignedAttr;
            case clang::attr::AlignMac68k: return CusorKindExt::AlignMac68kAttr;
            case clang::attr::AlignValue: return CusorKindExt::AlignValueAttr;
            case clang::attr::AllocSize: return CusorKindExt::AllocSizeAttr;
            case clang::attr::AlwaysInline: return CusorKindExt::AlwaysInlineAttr;
            case clang::attr::AMDGPUFlatWorkGroupSize: return CusorKindExt::AMDGPUFlatWorkGroupSizeAttr;
            case clang::attr::AMDGPUNumSGPR: return CusorKindExt::AMDGPUNumSGPRAttr;
            case clang::attr::AMDGPUNumVGPR: return CusorKindExt::AMDGPUNumVGPRAttr;
            case clang::attr::AMDGPUWavesPerEU: return CusorKindExt::AMDGPUWavesPerEUAttr;
            case clang::attr::AnalyzerNoReturn: return CusorKindExt::AnalyzerNoReturnAttr;
            case clang::attr::Annotate: return CusorKindExt::AnnotateAttr;
            case clang::attr::AnyX86Interrupt: return CusorKindExt::AnyX86InterruptAttr;
            case clang::attr::ArcWeakrefUnavailable: return CusorKindExt::ArcWeakrefUnavailableAttr;
            case clang::attr::ArgumentWithTypeTag: return CusorKindExt::ArgumentWithTypeTagAttr;
            case clang::attr::ARMInterrupt: return CusorKindExt::ARMInterruptAttr;
            case clang::attr::AsmLabel: return CusorKindExt::AsmLabelAttr;
            case clang::attr::AssertCapability: return CusorKindExt::AssertCapabilityAttr;
            case clang::attr::AssertExclusiveLock: return CusorKindExt::AssertExclusiveLockAttr;
            case clang::attr::AssertSharedLock: return CusorKindExt::AssertSharedLockAttr;
            case clang::attr::AssumeAligned: return CusorKindExt::AssumeAlignedAttr;
            case clang::attr::Availability: return CusorKindExt::AvailabilityAttr;
            case clang::attr::Blocks: return CusorKindExt::BlocksAttr;
            case clang::attr::C11NoReturn: return CusorKindExt::C11NoReturnAttr;
            case clang::attr::CallableWhen: return CusorKindExt::CallableWhenAttr;
            case clang::attr::Capability: return CusorKindExt::CapabilityAttr;
            case clang::attr::CapturedRecord: return CusorKindExt::CapturedRecordAttr;
            case clang::attr::CarriesDependency: return CusorKindExt::CarriesDependencyAttr;
            case clang::attr::CDecl: return CusorKindExt::CDeclAttr;
            case clang::attr::CFAuditedTransfer: return CusorKindExt::CFAuditedTransferAttr;
            case clang::attr::CFConsumed: return CusorKindExt::CFConsumedAttr;
            case clang::attr::CFReturnsNotRetained: return CusorKindExt::CFReturnsNotRetainedAttr;
            case clang::attr::CFReturnsRetained: return CusorKindExt::CFReturnsRetainedAttr;
            case clang::attr::CFUnknownTransfer: return CusorKindExt::CFUnknownTransferAttr;
            case clang::attr::Cleanup: return CusorKindExt::CleanupAttr;
            case clang::attr::Cold: return CusorKindExt::ColdAttr;
            case clang::attr::Common: return CusorKindExt::CommonAttr;
            case clang::attr::Const: return CusorKindExt::ConstAttr;
            case clang::attr::Constructor: return CusorKindExt::ConstructorAttr;
            case clang::attr::Consumable: return CusorKindExt::ConsumableAttr;
            case clang::attr::ConsumableAutoCast: return CusorKindExt::ConsumableAutoCastAttr;
            case clang::attr::ConsumableSetOnRead: return CusorKindExt::ConsumableSetOnReadAttr;
            case clang::attr::Convergent: return CusorKindExt::ConvergentAttr;
            case clang::attr::CUDAConstant: return CusorKindExt::CUDAConstantAttr;
            case clang::attr::CUDADevice: return CusorKindExt::CUDADeviceAttr;
            case clang::attr::CUDAGlobal: return CusorKindExt::CUDAGlobalAttr;
            case clang::attr::CUDAHost: return CusorKindExt::CUDAHostAttr;
            case clang::attr::CUDAInvalidTarget: return CusorKindExt::CUDAInvalidTargetAttr;
            case clang::attr::CUDALaunchBounds: return CusorKindExt::CUDALaunchBoundsAttr;
            case clang::attr::CUDAShared: return CusorKindExt::CUDASharedAttr;
            case clang::attr::CXX11NoReturn: return CusorKindExt::CXX11NoReturnAttr;
            case clang::attr::Deprecated: return CusorKindExt::DeprecatedAttr;
            case clang::attr::Destructor: return CusorKindExt::DestructorAttr;
            case clang::attr::DiagnoseIf: return CusorKindExt::DiagnoseIfAttr;
            case clang::attr::DisableTailCalls: return CusorKindExt::DisableTailCallsAttr;
            case clang::attr::DLLExport: return CusorKindExt::DLLExportAttr;
            case clang::attr::DLLImport: return CusorKindExt::DLLImportAttr;
            case clang::attr::EmptyBases: return CusorKindExt::EmptyBasesAttr;
            case clang::attr::EnableIf: return CusorKindExt::EnableIfAttr;
            case clang::attr::ExclusiveTrylockFunction: return CusorKindExt::ExclusiveTrylockFunctionAttr;
            case clang::attr::FallThrough: return CusorKindExt::FallThroughAttr;
            case clang::attr::FastCall: return CusorKindExt::FastCallAttr;
            case clang::attr::Final: return CusorKindExt::FinalAttr;
            case clang::attr::FlagEnum: return CusorKindExt::FlagEnumAttr;
            case clang::attr::Flatten: return CusorKindExt::FlattenAttr;
            case clang::attr::Format: return CusorKindExt::FormatAttr;
            case clang::attr::FormatArg: return CusorKindExt::FormatArgAttr;
            case clang::attr::GNUInline: return CusorKindExt::GNUInlineAttr;
            case clang::attr::GuardedBy: return CusorKindExt::GuardedByAttr;
            case clang::attr::GuardedVar: return CusorKindExt::GuardedVarAttr;
            case clang::attr::Hot: return CusorKindExt::HotAttr;
            case clang::attr::IBAction: return CusorKindExt::IBActionAttr;
            case clang::attr::IBOutlet: return CusorKindExt::IBOutletAttr;
            case clang::attr::IBOutletCollection: return CusorKindExt::IBOutletCollectionAttr;
            case clang::attr::IFunc: return CusorKindExt::IFuncAttr;
            case clang::attr::InitPriority: return CusorKindExt::InitPriorityAttr;
            case clang::attr::InitSeg: return CusorKindExt::InitSegAttr;
            case clang::attr::IntelOclBicc: return CusorKindExt::IntelOclBiccAttr;
            case clang::attr::InternalLinkage: return CusorKindExt::InternalLinkageAttr;
            case clang::attr::LayoutVersion: return CusorKindExt::LayoutVersionAttr;
            case clang::attr::LockReturned: return CusorKindExt::LockReturnedAttr;
            case clang::attr::LocksExcluded: return CusorKindExt::LocksExcludedAttr;
            case clang::attr::LoopHint: return CusorKindExt::LoopHintAttr;
            case clang::attr::LTOVisibilityPublic: return CusorKindExt::LTOVisibilityPublicAttr;
            case clang::attr::MaxFieldAlignment: return CusorKindExt::MaxFieldAlignmentAttr;
            case clang::attr::MayAlias: return CusorKindExt::MayAliasAttr;
            case clang::attr::MinSize: return CusorKindExt::MinSizeAttr;
            case clang::attr::Mips16: return CusorKindExt::Mips16Attr;
            case clang::attr::MipsInterrupt: return CusorKindExt::MipsInterruptAttr;
            case clang::attr::Mode: return CusorKindExt::ModeAttr;
            case clang::attr::MSABI: return CusorKindExt::MSABIAttr;
            case clang::attr::MSInheritance: return CusorKindExt::MSInheritanceAttr;
            case clang::attr::MSNoVTable: return CusorKindExt::MSNoVTableAttr;
            case clang::attr::MSP430Interrupt: return CusorKindExt::MSP430InterruptAttr;
            case clang::attr::MSStruct: return CusorKindExt::MSStructAttr;
            case clang::attr::MSVtorDisp: return CusorKindExt::MSVtorDispAttr;
            case clang::attr::Naked: return CusorKindExt::NakedAttr;
            case clang::attr::NoAlias: return CusorKindExt::NoAliasAttr;
            case clang::attr::NoCommon: return CusorKindExt::NoCommonAttr;
            case clang::attr::NoDebug: return CusorKindExt::NoDebugAttr;
            case clang::attr::NoDuplicate: return CusorKindExt::NoDuplicateAttr;
            case clang::attr::NoInline: return CusorKindExt::NoInlineAttr;
            case clang::attr::NoInstrumentFunction: return CusorKindExt::NoInstrumentFunctionAttr;
            case clang::attr::NoMips16: return CusorKindExt::NoMips16Attr;
            case clang::attr::NonNull: return CusorKindExt::NonNullAttr;
            case clang::attr::NoReturn: return CusorKindExt::NoReturnAttr;
            case clang::attr::NoSanitize: return CusorKindExt::NoSanitizeAttr;
            case clang::attr::NoSplitStack: return CusorKindExt::NoSplitStackAttr;
            case clang::attr::NoThreadSafetyAnalysis: return CusorKindExt::NoThreadSafetyAnalysisAttr;
            case clang::attr::NoThrow: return CusorKindExt::NoThrowAttr;
            case clang::attr::NotTailCalled: return CusorKindExt::NotTailCalledAttr;
            case clang::attr::NSConsumed: return CusorKindExt::NSConsumedAttr;
            case clang::attr::NSConsumesSelf: return CusorKindExt::NSConsumesSelfAttr;
            case clang::attr::NSReturnsAutoreleased: return CusorKindExt::NSReturnsAutoreleasedAttr;
            case clang::attr::NSReturnsNotRetained: return CusorKindExt::NSReturnsNotRetainedAttr;
            case clang::attr::NSReturnsRetained: return CusorKindExt::NSReturnsRetainedAttr;
            case clang::attr::ObjCBoxable: return CusorKindExt::ObjCBoxableAttr;
            case clang::attr::ObjCBridge: return CusorKindExt::ObjCBridgeAttr;
            case clang::attr::ObjCBridgeMutable: return CusorKindExt::ObjCBridgeMutableAttr;
            case clang::attr::ObjCBridgeRelated: return CusorKindExt::ObjCBridgeRelatedAttr;
            case clang::attr::ObjCDesignatedInitializer: return CusorKindExt::ObjCDesignatedInitializerAttr;
            case clang::attr::ObjCException: return CusorKindExt::ObjCExceptionAttr;
            case clang::attr::ObjCExplicitProtocolImpl: return CusorKindExt::ObjCExplicitProtocolImplAttr;
            case clang::attr::ObjCIndependentClass: return CusorKindExt::ObjCIndependentClassAttr;
            case clang::attr::ObjCMethodFamily: return CusorKindExt::ObjCMethodFamilyAttr;
            case clang::attr::ObjCNSObject: return CusorKindExt::ObjCNSObjectAttr;
            case clang::attr::ObjCPreciseLifetime: return CusorKindExt::ObjCPreciseLifetimeAttr;
            case clang::attr::ObjCRequiresPropertyDefs: return CusorKindExt::ObjCRequiresPropertyDefsAttr;
            case clang::attr::ObjCRequiresSuper: return CusorKindExt::ObjCRequiresSuperAttr;
            case clang::attr::ObjCReturnsInnerPointer: return CusorKindExt::ObjCReturnsInnerPointerAttr;
            case clang::attr::ObjCRootClass: return CusorKindExt::ObjCRootClassAttr;
            case clang::attr::ObjCRuntimeName: return CusorKindExt::ObjCRuntimeNameAttr;
            case clang::attr::ObjCRuntimeVisible: return CusorKindExt::ObjCRuntimeVisibleAttr;
            case clang::attr::ObjCSubclassingRestricted: return CusorKindExt::ObjCSubclassingRestrictedAttr;
            case clang::attr::OMPCaptureNoInit: return CusorKindExt::OMPCaptureNoInitAttr;
            case clang::attr::OMPDeclareSimdDecl: return CusorKindExt::OMPDeclareSimdDeclAttr;
            case clang::attr::OMPDeclareTargetDecl: return CusorKindExt::OMPDeclareTargetDeclAttr;
            case clang::attr::OMPThreadPrivateDecl: return CusorKindExt::OMPThreadPrivateDeclAttr;
            case clang::attr::OpenCLAccess: return CusorKindExt::OpenCLAccessAttr;
            case clang::attr::OpenCLKernel: return CusorKindExt::OpenCLKernelAttr;
            case clang::attr::OpenCLUnrollHint: return CusorKindExt::OpenCLUnrollHintAttr;
            case clang::attr::OptimizeNone: return CusorKindExt::OptimizeNoneAttr;
            case clang::attr::Overloadable: return CusorKindExt::OverloadableAttr;
            case clang::attr::Override: return CusorKindExt::OverrideAttr;
            case clang::attr::Ownership: return CusorKindExt::OwnershipAttr;
            case clang::attr::Packed: return CusorKindExt::PackedAttr;
            case clang::attr::ParamTypestate: return CusorKindExt::ParamTypestateAttr;
            case clang::attr::Pascal: return CusorKindExt::PascalAttr;
            case clang::attr::PassObjectSize: return CusorKindExt::PassObjectSizeAttr;
            case clang::attr::Pcs: return CusorKindExt::PcsAttr;
            case clang::attr::PreserveAll: return CusorKindExt::PreserveAllAttr;
            case clang::attr::PreserveMost: return CusorKindExt::PreserveMostAttr;
            case clang::attr::PtGuardedBy: return CusorKindExt::PtGuardedByAttr;
            case clang::attr::PtGuardedVar: return CusorKindExt::PtGuardedVarAttr;
            case clang::attr::Pure: return CusorKindExt::PureAttr;
            case clang::attr::RegCall: return CusorKindExt::RegCallAttr;
            case clang::attr::ReleaseCapability: return CusorKindExt::ReleaseCapabilityAttr;
            case clang::attr::RenderScriptKernel: return CusorKindExt::RenderScriptKernelAttr;
            case clang::attr::ReqdWorkGroupSize: return CusorKindExt::ReqdWorkGroupSizeAttr;
            case clang::attr::RequireConstantInit: return CusorKindExt::RequireConstantInitAttr;
            case clang::attr::RequiresCapability: return CusorKindExt::RequiresCapabilityAttr;
            case clang::attr::Restrict: return CusorKindExt::RestrictAttr;
            case clang::attr::ReturnsNonNull: return CusorKindExt::ReturnsNonNullAttr;
            case clang::attr::ReturnsTwice: return CusorKindExt::ReturnsTwiceAttr;
            case clang::attr::ReturnTypestate: return CusorKindExt::ReturnTypestateAttr;
            case clang::attr::ScopedLockable: return CusorKindExt::ScopedLockableAttr;
            case clang::attr::Section: return CusorKindExt::SectionAttr;
            case clang::attr::SelectAny: return CusorKindExt::SelectAnyAttr;
            case clang::attr::Sentinel: return CusorKindExt::SentinelAttr;
            case clang::attr::SetTypestate: return CusorKindExt::SetTypestateAttr;
            case clang::attr::SharedTrylockFunction: return CusorKindExt::SharedTrylockFunctionAttr;
            case clang::attr::StdCall: return CusorKindExt::StdCallAttr;
            case clang::attr::SwiftCall: return CusorKindExt::SwiftCallAttr;
            case clang::attr::SwiftContext: return CusorKindExt::SwiftContextAttr;
            case clang::attr::SwiftErrorResult: return CusorKindExt::SwiftErrorResultAttr;
            case clang::attr::SwiftIndirectResult: return CusorKindExt::SwiftIndirectResultAttr;
            case clang::attr::SysVABI: return CusorKindExt::SysVABIAttr;
            case clang::attr::Target: return CusorKindExt::TargetAttr;
            case clang::attr::TestTypestate: return CusorKindExt::TestTypestateAttr;
            case clang::attr::ThisCall: return CusorKindExt::ThisCallAttr;
            case clang::attr::Thread: return CusorKindExt::ThreadAttr;
            case clang::attr::TLSModel: return CusorKindExt::TLSModelAttr;
            case clang::attr::TransparentUnion: return CusorKindExt::TransparentUnionAttr;
            case clang::attr::TryAcquireCapability: return CusorKindExt::TryAcquireCapabilityAttr;
            case clang::attr::TypeTagForDatatype: return CusorKindExt::TypeTagForDatatypeAttr;
            case clang::attr::TypeVisibility: return CusorKindExt::TypeVisibilityAttr;
            case clang::attr::Unavailable: return CusorKindExt::UnavailableAttr;
            case clang::attr::Unused: return CusorKindExt::UnusedAttr;
            case clang::attr::Used: return CusorKindExt::UsedAttr;
            case clang::attr::Uuid: return CusorKindExt::UuidAttr;
            case clang::attr::VecReturn: return CusorKindExt::VecReturnAttr;
            case clang::attr::VectorCall: return CusorKindExt::VectorCallAttr;
            case clang::attr::VecTypeHint: return CusorKindExt::VecTypeHintAttr;
            case clang::attr::Visibility: return CusorKindExt::VisibilityAttr;
            case clang::attr::WarnUnused: return CusorKindExt::WarnUnusedAttr;
            case clang::attr::WarnUnusedResult: return CusorKindExt::WarnUnusedResultAttr;
            case clang::attr::Weak: return CusorKindExt::WeakAttr;
            case clang::attr::WeakImport: return CusorKindExt::WeakImportAttr;
            case clang::attr::WeakRef: return CusorKindExt::WeakRefAttr;
            case clang::attr::WorkGroupSizeHint: return CusorKindExt::WorkGroupSizeHintAttr;
            case clang::attr::X86ForceAlignArgPointer: return CusorKindExt::X86ForceAlignArgPointerAttr;
            case clang::attr::XRayInstrument: return CusorKindExt::XRayInstrument;
        }
    }

    auto S = getCursorStmt(cursor);
    switch(S->getStmtClass()) {
        case clang::Stmt::OpaqueValueExprClass: return CusorKindExt::OpaqueValueExpr;
        case clang::Stmt::ArrayTypeTraitExprClass: return CusorKindExt::ArrayTypeTraitExpr;
        case clang::Stmt::AsTypeExprClass: return CusorKindExt::AsTypeExpr;
        case clang::Stmt::AtomicExprClass: return CusorKindExt::AtomicExpr;
        case clang::Stmt::BinaryConditionalOperatorClass: return CusorKindExt::BinaryConditionalOperator;
        case clang::Stmt::TypeTraitExprClass: return CusorKindExt::TypeTraitExpr;
        case clang::Stmt::CoroutineBodyStmtClass: return CusorKindExt::CoroutineBodyStmt;
        case clang::Stmt::CoawaitExprClass: return CusorKindExt::CoawaitExpr;
        case clang::Stmt::CoreturnStmtClass: return CusorKindExt::CoreturnStmt;
        case clang::Stmt::CoyieldExprClass: return CusorKindExt::CoyieldExpr;
        case clang::Stmt::CXXBindTemporaryExprClass: return CusorKindExt::CXXBindTemporaryExpr;
        case clang::Stmt::CXXDefaultArgExprClass: return CusorKindExt::CXXDefaultArgExpr;
        case clang::Stmt::CXXDefaultInitExprClass: return CusorKindExt::CXXDefaultInitExpr;
        case clang::Stmt::CXXFoldExprClass: return CusorKindExt::CXXFoldExpr;
        case clang::Stmt::CXXStdInitializerListExprClass: return CusorKindExt::CXXStdInitializerListExpr;
        case clang::Stmt::CXXScalarValueInitExprClass: return CusorKindExt::CXXScalarValueInitExpr;
        case clang::Stmt::CXXUuidofExprClass: return CusorKindExt::CXXUuidofExpr;
        case clang::Stmt::ChooseExprClass: return CusorKindExt::ChooseExpr;
        case clang::Stmt::DesignatedInitExprClass: return CusorKindExt::DesignatedInitExpr;
        case clang::Stmt::DesignatedInitUpdateExprClass: return CusorKindExt::DesignatedInitUpdateExpr;
        case clang::Stmt::ArrayInitLoopExprClass: return CusorKindExt::ArrayInitLoopExpr;
        case clang::Stmt::ArrayInitIndexExprClass: return CusorKindExt::ArrayInitIndexExpr;
        case clang::Stmt::ExprWithCleanupsClass: return CusorKindExt::ExprWithCleanups;
        case clang::Stmt::ExpressionTraitExprClass: return CusorKindExt::ExpressionTraitExpr;
        case clang::Stmt::ExtVectorElementExprClass: return CusorKindExt::ExtVectorElementExpr;
        case clang::Stmt::ImplicitCastExprClass: return CusorKindExt::ImplicitCastExpr;
        case clang::Stmt::ImplicitValueInitExprClass: return CusorKindExt::ImplicitValueInitExpr;
        case clang::Stmt::NoInitExprClass: return CusorKindExt::NoInitExpr;
        case clang::Stmt::MaterializeTemporaryExprClass: return CusorKindExt::MaterializeTemporaryExpr;
        case clang::Stmt::ObjCIndirectCopyRestoreExprClass: return CusorKindExt::ObjCIndirectCopyRestoreExpr;
        case clang::Stmt::OffsetOfExprClass: return CusorKindExt::OffsetOfExpr;
        case clang::Stmt::ParenListExprClass: return CusorKindExt::ParenListExpr;
        case clang::Stmt::PredefinedExprClass: return CusorKindExt::PredefinedExpr;
        case clang::Stmt::ShuffleVectorExprClass: return CusorKindExt::ShuffleVectorExpr;
        case clang::Stmt::ConvertVectorExprClass: return CusorKindExt::ConvertVectorExpr;
        case clang::Stmt::VAArgExprClass: return CusorKindExt::VAArgExpr;
        case clang::Stmt::ObjCArrayLiteralClass: return CusorKindExt::ObjCArrayLiteral;
        case clang::Stmt::ObjCDictionaryLiteralClass: return CusorKindExt::ObjCDictionaryLiteral;
        case clang::Stmt::ObjCBoxedExprClass: return CusorKindExt::ObjCBoxedExpr;
        case clang::Stmt::ObjCSubscriptRefExprClass: return CusorKindExt::ObjCSubscriptRefExpr;
        default: {}
    }
    return CusorKindExt::RanOutOfIdeas;
}

/** Value of integer or char literal */
extern "C" uint64_t c3_Cursor_getIntValue(CXCursor cursor)
{
    if (cursor.kind == CXCursor_IntegerLiteral) {
        auto lit = (clang::IntegerLiteral *) getCursorExpr(cursor);
        return lit->getValue().getZExtValue();
    }

    if (cursor.kind == CXCursor_CharacterLiteral) {
        auto lit = (clang::CharacterLiteral *) getCursorExpr(cursor);
        return lit->getValue();
    }

    return -1;
}

/** value of float literal */
extern "C" double c3_Cursor_getFloatValue(CXCursor cursor)
{
    if (cursor.kind == CXCursor_FloatingLiteral) {
        auto lit = (clang::FloatingLiteral *) getCursorExpr(cursor);
        return lit->getValueAsApproximateDouble();
    }

    return NAN;
}

extern "C" clang::BinaryOperatorKind c3_Cursor_getBinaryOpcode(CXCursor cursor)
{
    if (cursor.kind == CXCursor_BinaryOperator) {
        auto op = (clang::BinaryOperator *) getCursorExpr(cursor);
        return static_cast<clang::BinaryOperatorKind>(op->getOpcode());
    }

    if (cursor.kind == CXCursor_CompoundAssignOperator) {
        clang::CompoundAssignOperator *op = (clang::CompoundAssignOperator *) getCursorExpr(cursor);
        return static_cast<clang::BinaryOperatorKind>(op->getOpcode());
    }

    return (clang::BinaryOperatorKind) 99999;
}

extern "C" clang::UnaryOperatorKind c3_Cursor_getUnaryOpcode(CXCursor cursor)
{
    if (cursor.kind == CXCursor_UnaryOperator) {
        auto op = (clang::UnaryOperator*) getCursorExpr(cursor);
        return static_cast<clang::UnaryOperatorKind>(op->getOpcode());
    }

    return (clang::UnaryOperatorKind) 99999;
}

/** Actual type of "Unexposed" libclang Type */
extern "C" ClangTypeExt c3_CursorType_getKindExt(CXType CT) {
    auto T = clang::QualType::getFromOpaquePtr(CT.data[0]);
    if (T.isNull()) {
        return SomethingIsBrokenType;
    }
    auto TP = T.getTypePtrOrNull();
    if (!TP) {
        return SomethingIsBrokenType;
    }

    switch (TP->getTypeClass()) {
        case clang::Type::Adjusted: return ClangTypeExt::AdjustedType;
        case clang::Type::Atomic: return ClangTypeExt::AtomicType;
        case clang::Type::Attributed: return ClangTypeExt::AttributedType;
        case clang::Type::Auto: return ClangTypeExt::AutoType;
        case clang::Type::BlockPointer: return ClangTypeExt::BlockPointerType;
        case clang::Type::Builtin: return ClangTypeExt::BuiltinType;
        case clang::Type::Complex: return ClangTypeExt::ComplexType;
        case clang::Type::ConstantArray: return ClangTypeExt::ConstantArrayType;
        case clang::Type::Decayed: return ClangTypeExt::DecayedType;
        case clang::Type::Decltype: return ClangTypeExt::DecltypeType;
        case clang::Type::DependentName: return ClangTypeExt::DependentNameType;
        case clang::Type::DependentSizedArray: return ClangTypeExt::DependentSizedArrayType;
        case clang::Type::DependentSizedExtVector: return ClangTypeExt::DependentSizedExtVectorType;
        case clang::Type::DependentTemplateSpecialization: return ClangTypeExt::DependentTemplateSpecializationType;
        case clang::Type::Elaborated: return ClangTypeExt::ElaboratedType;
        case clang::Type::Enum: return ClangTypeExt::EnumType;
        case clang::Type::ExtVector: return ClangTypeExt::ExtVectorType;
        case clang::Type::FunctionNoProto: return ClangTypeExt::FunctionNoProtoType;
        case clang::Type::FunctionProto: return ClangTypeExt::FunctionProtoType;
        case clang::Type::IncompleteArray: return ClangTypeExt::IncompleteArrayType;
        case clang::Type::InjectedClassName: return ClangTypeExt::InjectedClassNameType;
        case clang::Type::LValueReference: return ClangTypeExt::LValueReferenceType;
        case clang::Type::MemberPointer: return ClangTypeExt::MemberPointerType;
        case clang::Type::ObjCInterface: return ClangTypeExt::ObjCInterfaceType;
        case clang::Type::ObjCObject: return ClangTypeExt::ObjCObjectType;
        case clang::Type::ObjCObjectPointer: return ClangTypeExt::ObjCObjectPointerType;
        case clang::Type::ObjCTypeParam: return ClangTypeExt::ObjCTypeParamType;
        case clang::Type::PackExpansion: return ClangTypeExt::PackExpansionType;
        case clang::Type::Paren: return ClangTypeExt::ParenType;
        case clang::Type::Pipe: return ClangTypeExt::PipeType;
        case clang::Type::Pointer: return ClangTypeExt::PointerType;
        case clang::Type::Record: return ClangTypeExt::RecordType;
        case clang::Type::RValueReference: return ClangTypeExt::RValueReferenceType;
        case clang::Type::SubstTemplateTypeParm: return ClangTypeExt::SubstTemplateTypeParmType;
        case clang::Type::SubstTemplateTypeParmPack: return ClangTypeExt::SubstTemplateTypeParmPackType;
        case clang::Type::TemplateSpecialization: return ClangTypeExt::TemplateSpecializationType;
        case clang::Type::TemplateTypeParm: return ClangTypeExt::TemplateTypeParmType;
        case clang::Type::Typedef: return ClangTypeExt::TypedefType;
        case clang::Type::TypeOf: return ClangTypeExt::TypeOfType;
        case clang::Type::TypeOfExpr: return ClangTypeExt::TypeOfExprType;
        case clang::Type::UnaryTransform: return ClangTypeExt::UnaryTransformType;
        case clang::Type::UnresolvedUsing: return ClangTypeExt::UnresolvedUsingType;
        case clang::Type::VariableArray: return ClangTypeExt::VariableArrayType;
        case clang::Type::Vector: return ClangTypeExt::VectorType;
    }
}
