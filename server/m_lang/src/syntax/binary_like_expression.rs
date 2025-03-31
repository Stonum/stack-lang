//! Binary like nodes are nodes with `left` and `right` expressions. They include:
//! - [MBinaryExpression]
//! - [MLogicalExpression]
//! - [MInExpression]
//! - [MInstanceofExpression]

use super::{
    AnyMExpression, MBinaryExpression, MBinaryOperator, MIfStatement, MInExpression, MInProperty,
    MLogicalExpression, MLogicalOperator, MSwitchStatement, MSyntaxKind, MSyntaxNode, MSyntaxToken,
    MWhileStatement, OperatorPrecedence,
};

use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, SyntaxResult};
use std::fmt::Debug;
use std::hash::Hash;

declare_node_union! {
    pub AnyMBinaryLikeExpression = MLogicalExpression | MBinaryExpression | MInExpression
}

impl AnyMBinaryLikeExpression {
    pub fn left(&self) -> SyntaxResult<AnyMBinaryLikeLeftExpression> {
        match self {
            Self::MLogicalExpression(logical) => logical
                .left()
                .map(AnyMBinaryLikeLeftExpression::AnyMExpression),
            Self::MBinaryExpression(binary) => binary
                .left()
                .map(AnyMBinaryLikeLeftExpression::AnyMExpression),
            Self::MInExpression(in_expression) => in_expression
                .property()
                .map(AnyMBinaryLikeLeftExpression::from),
        }
    }

    pub fn operator_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            Self::MLogicalExpression(logical) => logical.operator_token(),
            Self::MBinaryExpression(binary) => binary.operator_token(),
            Self::MInExpression(in_expression) => in_expression.in_token(),
        }
    }

    pub fn operator(&self) -> SyntaxResult<BinaryLikeOperator> {
        match self {
            Self::MLogicalExpression(logical) => {
                logical.operator().map(BinaryLikeOperator::Logical)
            }
            Self::MBinaryExpression(binary) => binary.operator().map(BinaryLikeOperator::Binary),
            Self::MInExpression(_) => Ok(BinaryLikeOperator::In),
        }
    }

    /// Returns the right hand side of the binary like expression.
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        match self {
            Self::MLogicalExpression(logical) => logical.right(),
            Self::MBinaryExpression(binary) => binary.right(),
            Self::MInExpression(in_expression) => in_expression.object(),
        }
    }

    /// Returns `true` if the expression is inside of a test condition of `parent`.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// if (a + b) {} // true
    /// if (true) { a + b } // false
    /// switch (a + b) {} // true
    /// ```
    pub fn is_inside_condition(&self, parent: Option<&MSyntaxNode>) -> bool {
        parent.is_some_and(|parent| {
            let test = match parent.kind() {
                MSyntaxKind::M_IF_STATEMENT => MIfStatement::unwrap_cast(parent.clone()).test(),
                MSyntaxKind::M_WHILE_STATEMENT => {
                    MWhileStatement::unwrap_cast(parent.clone()).test()
                }
                MSyntaxKind::M_SWITCH_STATEMENT => {
                    MSwitchStatement::unwrap_cast(parent.clone()).discriminant()
                }
                _ => return false,
            };
            test.is_ok_and(|test| test.syntax() == self.syntax())
        })
    }

    /// Determines if a binary like expression should be flattened or not. As a rule of thumb, an expression
    /// can be flattened if its left hand side has the same operator-precedence
    pub fn can_flatten(&self) -> SyntaxResult<bool> {
        let left = self.left()?.into_expression();
        let left_expression = left.map(|expression| expression.into_syntax());
        if let Some(left_binary_like) = left_expression.and_then(AnyMBinaryLikeExpression::cast) {
            Ok(should_flatten(
                self.operator()?,
                left_binary_like.operator()?,
            ))
        } else {
            Ok(false)
        }
    }

    pub fn should_inline_logical_expression(&self) -> bool {
        match self {
            AnyMBinaryLikeExpression::MLogicalExpression(logical) => {
                logical.right().is_ok_and(|right| match right {
                    AnyMExpression::MObjectExpression(object) => !object.members().is_empty(),
                    AnyMExpression::MArrayExpression(array) => !array.elements().is_empty(),
                    _ => false,
                })
            }
            _ => false,
        }
    }

    /// This function checks whether the chain of logical/binary expressions **should not** be indented
    ///
    /// There are some cases where the indentation is done by the parent, so if the parent is already doing
    /// the indentation, then there's no need to do a second indentation.
    pub fn should_not_indent_if_parent_indents(
        self: &AnyMBinaryLikeExpression,
        parent: Option<MSyntaxNode>,
    ) -> bool {
        parent.is_some_and(|parent| match parent.kind() {
            MSyntaxKind::M_RETURN_STATEMENT | MSyntaxKind::M_THROW_STATEMENT => true,
            MSyntaxKind::M_TEMPLATE_ELEMENT => true,
            MSyntaxKind::M_FOR_STATEMENT => true,
            MSyntaxKind::M_CONDITIONAL_EXPRESSION => parent.parent().is_some_and(|grand_parent| {
                !matches!(
                    grand_parent.kind(),
                    MSyntaxKind::M_RETURN_STATEMENT
                        | MSyntaxKind::M_THROW_STATEMENT
                        | MSyntaxKind::M_CALL_EXPRESSION
                        | MSyntaxKind::M_CALL_ARGUMENT_LIST
                )
            }),
            _ => false,
        })
    }
}

pub(crate) fn should_flatten(
    parent_operator: BinaryLikeOperator,
    operator: BinaryLikeOperator,
) -> bool {
    if operator.precedence() != parent_operator.precedence() {
        return false;
    }
    match (parent_operator.precedence(), operator.precedence()) {
        // `**` is right associative
        (OperatorPrecedence::Exponential, _) => false,

        // `a == b == c` => `(a == b) == c`
        (OperatorPrecedence::Equality, OperatorPrecedence::Equality) => false,

        (OperatorPrecedence::Multiplicative, OperatorPrecedence::Multiplicative) => {
            // `a * 3 % 5` -> `(a * 3) % 5`
            if parent_operator == BinaryLikeOperator::Binary(MBinaryOperator::Remainder)
                || operator == BinaryLikeOperator::Binary(MBinaryOperator::Remainder)
            {
                false
            }
            // `a * 3 / 5` -> `(a * 3) / 5
            else {
                parent_operator == operator
            }
        }
        // `a << 3 << 4` -> `(a << 3) << 4`
        (OperatorPrecedence::Shift, OperatorPrecedence::Shift) => false,
        _ => true,
    }
}

impl From<AnyMBinaryLikeExpression> for AnyMExpression {
    fn from(binary: AnyMBinaryLikeExpression) -> Self {
        match binary {
            AnyMBinaryLikeExpression::MLogicalExpression(expr) => expr.into(),
            AnyMBinaryLikeExpression::MBinaryExpression(expr) => expr.into(),
            AnyMBinaryLikeExpression::MInExpression(expr) => expr.into(),
        }
    }
}

declare_node_union! {
    /// Union type for any valid left hand side of a [AnyMBinaryLikeExpression].
    pub AnyMBinaryLikeLeftExpression = AnyMExpression
}

impl AnyMBinaryLikeLeftExpression {
    pub fn into_expression(self) -> Option<AnyMExpression> {
        match self {
            Self::AnyMExpression(expression) => Some(expression),
        }
    }
}

impl From<MInProperty> for AnyMBinaryLikeLeftExpression {
    fn from(property: MInProperty) -> Self {
        Self::AnyMExpression(property.any_m_expression().unwrap())
    }
}

/// Union over all binary like operators.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryLikeOperator {
    Logical(MLogicalOperator),
    Binary(MBinaryOperator),
    In,
}

impl BinaryLikeOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            Self::Logical(logical) => logical.precedence(),
            Self::Binary(binary) => binary.precedence(),
            Self::In => OperatorPrecedence::Relational,
        }
    }

    pub const fn is_remainder(&self) -> bool {
        matches!(self, Self::Binary(MBinaryOperator::Remainder))
    }
}

impl From<MLogicalOperator> for BinaryLikeOperator {
    fn from(operator: MLogicalOperator) -> Self {
        Self::Logical(operator)
    }
}

impl From<MBinaryOperator> for BinaryLikeOperator {
    fn from(binary: MBinaryOperator) -> Self {
        Self::Binary(binary)
    }
}
