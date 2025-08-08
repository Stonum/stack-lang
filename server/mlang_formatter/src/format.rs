//! Generated file, do not edit by hand, see `xtask/codegen`

use super::{Format, FormatElement, FormatNode, Formatter};
use biome_formatter::FormatResult;
impl Format for mlang_syntax::MScript {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MModule {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExpressionSnipped {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MDirective {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBlockStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBreakStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MClassDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MContinueStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MDebuggerStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MDoWhileStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MEmptyStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExpressionStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MForInStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MForOfStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MForStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MIfStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MLabeledStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MReturnStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSwitchStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MThrowStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MTryFinallyStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MTryStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MVariableStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MWhileStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MWithStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFunctionDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsEnumDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeAliasDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsInterfaceDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDeclareFunctionDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDeclareStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsModuleDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExternalModuleDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsGlobalDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsImportEqualsDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MElseClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MVariableDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MForVariableDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MVariableDeclarator {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MCaseClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MDefaultClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MCatchClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFinallyClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MCatchDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::ImportMeta {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrowFunctionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MAssignmentExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MAwaitExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBinaryExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MCallExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MClassExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MComputedMemberExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MConditionalExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFunctionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MIdentifierExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportCallExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MInExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MInstanceofExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MLogicalExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNewExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MParenthesizedExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPostUpdateExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPreUpdateExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSequenceExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MStaticMemberExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSuperExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MThisExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnaryExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MYieldExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::NewTarget {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MTemplate {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeAssertionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAsExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNonNullAssertionExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxTagExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeArguments {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MTemplateChunkElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MCallArguments {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MYieldArgument {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsReturnTypeAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFunctionBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSpread {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayHole {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MReferenceIdentifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPrivateName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MLiteralMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MComputedMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPropertyObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MMethodObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MGetterObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSetterObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MShorthandPropertyObjectMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExtendsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsImplementsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MClassExportDefaultDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPrivateClassMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MConstructorClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MStaticInitializationBlockClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MPropertyClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MMethodClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MGetterClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MSetterClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsConstructorSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsPropertySignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMethodSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsGetterSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsSetterSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIndexSignatureClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MStaticModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDeclareModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsReadonlyModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAbstractModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsOverrideModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAccessibilityModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MConstructorParameters {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MRestParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsPropertyParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MInitializerClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsOptionalPropertyAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDefinitePropertyAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIndexSignatureParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MIdentifierAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MStaticMemberAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MComputedMemberAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MParenthesizedAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNonNullAssertionAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAsAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeAssertionAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayAssignmentPatternElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayAssignmentPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectAssignmentPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayAssignmentPatternRestElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectAssignmentPatternShorthandProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectAssignmentPatternProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectAssignmentPatternRest {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MIdentifierBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBindingPatternWithDefault {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayBindingPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectBindingPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MArrayBindingPatternRestElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectBindingPatternProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectBindingPatternRest {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MObjectBindingPatternShorthandProperty {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MStringLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNumberLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBigIntLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MBooleanLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNullLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MRegexLiteralExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MVariableDeclarationClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDefiniteVariableAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExport {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImport {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportBareClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportNamedClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportDefaultClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportNamespaceClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MModuleSource {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportAssertion {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MDefaultImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNamedImportSpecifiers {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNamespaceImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MShorthandNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MLiteralExportName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MImportAssertionEntry {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportDefaultDeclarationClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportDefaultExpressionClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportNamedClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportFromClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportNamedFromClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExportAsNamespaceClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExportAssignmentClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExportDeclareClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFunctionExportDefaultDeclaration {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportNamedShorthandSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportNamedSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportAsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MExportNamedFromSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MFormalParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsThisParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAnyType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsUnknownType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNumberType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsBooleanType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsBigintType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsStringType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsSymbolType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsVoidType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsUndefinedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNeverType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsParenthesizedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsReferenceType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsArrayType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTupleType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeofType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsImportType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeOperatorType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIndexedAccessType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMappedType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsObjectType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNonPrimitiveType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsThisType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNumberLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsBigIntLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsStringLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNullLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsBooleanLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTemplateLiteralType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsInferType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIntersectionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsUnionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsFunctionType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsConstructorType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsConditionalType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIdentifierBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsEnumMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExternalModuleReference {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsModuleBlock {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsQualifiedModuleName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsEmptyExternalModuleDeclarationBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeParameterName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeParameterModifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsPredicateReturnType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAssertsReturnType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsAssertsCondition {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTypeConstraintClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsDefaultTypeClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsExtendsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsCallSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsPropertySignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsConstructSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMethodSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsGetterSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsSetterSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsIndexSignatureTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMappedTypeReadonlyModifierClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMappedTypeAsClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsMappedTypeOptionalModifierClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsImportTypeQualifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsNamedTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsRestTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsOptionalTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTemplateChunkElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsTemplateElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::TsQualifiedName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxSelfClosingElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxOpeningElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxClosingElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxOpeningFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxClosingFragment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxReferenceIdentifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxNamespaceName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxMemberName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxAttribute {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxSpreadAttribute {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxAttributeInitializerClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxString {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxExpressionAttributeValue {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxText {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxExpressionChild {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MxSpreadChild {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknown {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownStatement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownBinding {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownAssignment {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownParameter {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownImportAssertionEntry {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
impl Format for mlang_syntax::MUnknownNamedImportSpecifier {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_node(formatter)
    }
}
