//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{Format, FormatElement, FormatNode, Formatter};
use biome_formatter::FormatResult;
impl Format for crate::syntax::MScript {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MModule {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExpressionSnipped {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MDirective {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBlockStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBreakStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MClassDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MContinueStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MDebuggerStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MDoWhileStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MEmptyStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExpressionStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MForInStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MForOfStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MForStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MIfStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MLabeledStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MReturnStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSwitchStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MThrowStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MTryFinallyStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MTryStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MVariableStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MWhileStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MWithStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFunctionDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsEnumDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeAliasDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsInterfaceDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDeclareFunctionDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDeclareStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsModuleDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExternalModuleDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsGlobalDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsImportEqualsDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MElseClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MVariableDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MForVariableDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MVariableDeclarator {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MCaseClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MDefaultClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MCatchClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFinallyClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MCatchDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::ImportMeta {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrowFunctionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MAssignmentExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MAwaitExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBinaryExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MCallExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MClassExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MComputedMemberExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MConditionalExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFunctionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MIdentifierExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportCallExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MInExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MInstanceofExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MLogicalExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNewExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MParenthesizedExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPostUpdateExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPreUpdateExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSequenceExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MStaticMemberExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSuperExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MThisExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnaryExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MYieldExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::NewTarget {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MTemplate {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeAssertionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAsExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNonNullAssertionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxTagExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeArguments {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MTemplateChunkElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MCallArguments {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MYieldArgument {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsReturnTypeAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFunctionBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSpread {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayHole {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MReferenceIdentifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPrivateName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MLiteralMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MComputedMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPropertyObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MMethodObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MGetterObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSetterObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MShorthandPropertyObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExtendsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsImplementsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MClassExportDefaultDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPrivateClassMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MConstructorClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MStaticInitializationBlockClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MPropertyClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MMethodClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MGetterClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MSetterClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsConstructorSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsPropertySignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMethodSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsGetterSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsSetterSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIndexSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MStaticModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDeclareModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsReadonlyModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAbstractModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsOverrideModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAccessibilityModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MConstructorParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MRestParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsPropertyParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MInitializerClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsOptionalPropertyAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDefinitePropertyAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIndexSignatureParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MIdentifierAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MStaticMemberAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MComputedMemberAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MParenthesizedAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNonNullAssertionAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAsAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeAssertionAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayAssignmentPatternElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayAssignmentPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectAssignmentPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayAssignmentPatternRestElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectAssignmentPatternShorthandProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectAssignmentPatternProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectAssignmentPatternRest {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MIdentifierBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBindingPatternWithDefault {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayBindingPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectBindingPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MArrayBindingPatternRestElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectBindingPatternProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectBindingPatternRest {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MObjectBindingPatternShorthandProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MStringLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNumberLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBigIntLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MBooleanLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNullLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MRegexLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MVariableDeclarationClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDefiniteVariableAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExport {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImport {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportBareClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportNamedClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportDefaultClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportNamespaceClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MModuleSource {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportAssertion {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MDefaultImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNamedImportSpecifiers {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNamespaceImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MShorthandNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MLiteralExportName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MImportAssertionEntry {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportDefaultDeclarationClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportDefaultExpressionClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportNamedClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportFromClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportNamedFromClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExportAsNamespaceClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExportAssignmentClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExportDeclareClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFunctionExportDefaultDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportNamedShorthandSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportNamedSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportAsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MExportNamedFromSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MFormalParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsThisParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAnyType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsUnknownType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNumberType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsBooleanType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsBigintType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsStringType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsSymbolType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsVoidType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsUndefinedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNeverType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsParenthesizedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsReferenceType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsArrayType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTupleType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeofType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsImportType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeOperatorType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIndexedAccessType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMappedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsObjectType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNonPrimitiveType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsThisType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNumberLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsBigIntLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsStringLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNullLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsBooleanLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTemplateLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsInferType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIntersectionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsUnionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsFunctionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsConstructorType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsConditionalType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIdentifierBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsEnumMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExternalModuleReference {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsModuleBlock {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsQualifiedModuleName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsEmptyExternalModuleDeclarationBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeParameterName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeParameterModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsPredicateReturnType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAssertsReturnType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsAssertsCondition {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTypeConstraintClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsDefaultTypeClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsExtendsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsCallSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsPropertySignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsConstructSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMethodSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsGetterSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsSetterSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsIndexSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMappedTypeReadonlyModifierClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMappedTypeAsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsMappedTypeOptionalModifierClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsImportTypeQualifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsNamedTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsRestTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsOptionalTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTemplateChunkElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::TsQualifiedName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxSelfClosingElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxOpeningElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxClosingElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxOpeningFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxClosingFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxReferenceIdentifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxNamespaceName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxAttribute {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxSpreadAttribute {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxAttributeInitializerClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxString {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxExpressionAttributeValue {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxText {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxExpressionChild {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MxSpreadChild {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknown {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownImportAssertionEntry {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for crate::syntax::MUnknownNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
