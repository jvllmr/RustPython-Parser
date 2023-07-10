// File automatically generated by ast/asdl_rs.py.

#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::ModModule<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::ModInteractive<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::ModExpression<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::ModFunctionType<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::Mod {
    fn range(&self) -> TextRange {
        match self {
            Self::Module(node) => node.range(),
            Self::Interactive(node) => node.range(),
            Self::Expression(node) => node.range(),
            Self::FunctionType(node) => node.range(),
        }
    }
}

impl Ranged for crate::generic::StmtFunctionDef<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAsyncFunctionDef<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtClassDef<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtReturn<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtDelete<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAssign<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtTypeAlias<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAugAssign<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAnnAssign<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtFor<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAsyncFor<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtWhile<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtIf<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtWith<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAsyncWith<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtMatch<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtRaise<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtTry<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtTryStar<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtAssert<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtImport<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtImportFrom<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtGlobal<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtNonlocal<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtExpr<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtPass<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtBreak<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::StmtContinue<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::Stmt {
    fn range(&self) -> TextRange {
        match self {
            Self::FunctionDef(node) => node.range(),
            Self::AsyncFunctionDef(node) => node.range(),
            Self::ClassDef(node) => node.range(),
            Self::Return(node) => node.range(),
            Self::Delete(node) => node.range(),
            Self::Assign(node) => node.range(),
            Self::TypeAlias(node) => node.range(),
            Self::AugAssign(node) => node.range(),
            Self::AnnAssign(node) => node.range(),
            Self::For(node) => node.range(),
            Self::AsyncFor(node) => node.range(),
            Self::While(node) => node.range(),
            Self::If(node) => node.range(),
            Self::With(node) => node.range(),
            Self::AsyncWith(node) => node.range(),
            Self::Match(node) => node.range(),
            Self::Raise(node) => node.range(),
            Self::Try(node) => node.range(),
            Self::TryStar(node) => node.range(),
            Self::Assert(node) => node.range(),
            Self::Import(node) => node.range(),
            Self::ImportFrom(node) => node.range(),
            Self::Global(node) => node.range(),
            Self::Nonlocal(node) => node.range(),
            Self::Expr(node) => node.range(),
            Self::Pass(node) => node.range(),
            Self::Break(node) => node.range(),
            Self::Continue(node) => node.range(),
        }
    }
}

impl Ranged for crate::generic::ExprBoolOp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprNamedExpr<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprBinOp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprUnaryOp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprLambda<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprIfExp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprDict<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprSet<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprListComp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprSetComp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprDictComp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprGeneratorExp<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprAwait<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprYield<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprYieldFrom<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprCompare<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprCall<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprFormattedValue<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprJoinedStr<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprConstant<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprAttribute<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprSubscript<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprStarred<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprName<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprList<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprTuple<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExprSlice<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::Expr {
    fn range(&self) -> TextRange {
        match self {
            Self::BoolOp(node) => node.range(),
            Self::NamedExpr(node) => node.range(),
            Self::BinOp(node) => node.range(),
            Self::UnaryOp(node) => node.range(),
            Self::Lambda(node) => node.range(),
            Self::IfExp(node) => node.range(),
            Self::Dict(node) => node.range(),
            Self::Set(node) => node.range(),
            Self::ListComp(node) => node.range(),
            Self::SetComp(node) => node.range(),
            Self::DictComp(node) => node.range(),
            Self::GeneratorExp(node) => node.range(),
            Self::Await(node) => node.range(),
            Self::Yield(node) => node.range(),
            Self::YieldFrom(node) => node.range(),
            Self::Compare(node) => node.range(),
            Self::Call(node) => node.range(),
            Self::FormattedValue(node) => node.range(),
            Self::JoinedStr(node) => node.range(),
            Self::Constant(node) => node.range(),
            Self::Attribute(node) => node.range(),
            Self::Subscript(node) => node.range(),
            Self::Starred(node) => node.range(),
            Self::Name(node) => node.range(),
            Self::List(node) => node.range(),
            Self::Tuple(node) => node.range(),
            Self::Slice(node) => node.range(),
        }
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::Comprehension<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::ExceptHandlerExceptHandler<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::ExceptHandler {
    fn range(&self) -> TextRange {
        match self {
            Self::ExceptHandler(node) => node.range(),
        }
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::PythonArguments<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::Arg<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::Keyword<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::Alias<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::WithItem<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::MatchCase<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchValue<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchSingleton<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchSequence<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchMapping<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchClass<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchStar<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchAs<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::PatternMatchOr<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::Pattern {
    fn range(&self) -> TextRange {
        match self {
            Self::MatchValue(node) => node.range(),
            Self::MatchSingleton(node) => node.range(),
            Self::MatchSequence(node) => node.range(),
            Self::MatchMapping(node) => node.range(),
            Self::MatchClass(node) => node.range(),
            Self::MatchStar(node) => node.range(),
            Self::MatchAs(node) => node.range(),
            Self::MatchOr(node) => node.range(),
        }
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::TypeIgnoreTypeIgnore<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::TypeIgnore {
    fn range(&self) -> TextRange {
        match self {
            Self::TypeIgnore(node) => node.range(),
        }
    }
}

impl Ranged for crate::generic::TypeParamTypeVar<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::TypeParamParamSpec<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::generic::TypeParamTypeVarTuple<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
impl Ranged for crate::TypeParam {
    fn range(&self) -> TextRange {
        match self {
            Self::TypeVar(node) => node.range(),
            Self::ParamSpec(node) => node.range(),
            Self::TypeVarTuple(node) => node.range(),
        }
    }
}

#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::Arguments<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
#[cfg(feature = "all-nodes-with-ranges")]
impl Ranged for crate::generic::ArgWithDefault<TextRange> {
    fn range(&self) -> TextRange {
        self.range
    }
}
