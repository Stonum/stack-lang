//! Extensions for things which are not easily generated in ast expr nodes
use crate::static_value::StaticValue;
use crate::{
    AnyMExpression, AnyMLiteralExpression, AnyMObjectMemberName, MArrayExpression, MArrayHole,
    MBinaryExpression, MComputedMemberAssignment, MComputedMemberExpression, MLogicalExpression,
    MLongStringLiteralExpression, MObjectExpression, MPostUpdateExpression, MPreUpdateExpression,
    MReferenceIdentifier, MStringLiteralExpression, MSyntaxToken, MUnaryExpression,
    OperatorPrecedence, T, inner_string_text,
};
use biome_rowan::{AstSeparatedList, SyntaxResult, TokenText, declare_node_union};
use core::iter;

/// A binary operation applied to two expressions
///
/// The variants are ordered based on their precedence
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MBinaryOperator {
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `<=`
    LessThanOrEqual,
    /// `>=`
    GreaterThanOrEqual,
    /// `==`
    Equality,
    /// `!=`
    Inequality,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Times,
    /// `/`
    Divide,
    /// `%`
    Remainder,
    /// `&`
    BitwiseAnd,
    /// `|`
    BitwiseOr,
}

impl MBinaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            MBinaryOperator::LessThan
            | MBinaryOperator::GreaterThan
            | MBinaryOperator::LessThanOrEqual
            | MBinaryOperator::GreaterThanOrEqual => OperatorPrecedence::Relational,

            MBinaryOperator::Equality | MBinaryOperator::Inequality => OperatorPrecedence::Equality,

            MBinaryOperator::Plus | MBinaryOperator::Minus => OperatorPrecedence::Additive,

            MBinaryOperator::Times | MBinaryOperator::Divide | MBinaryOperator::Remainder => {
                OperatorPrecedence::Multiplicative
            }

            MBinaryOperator::BitwiseAnd => OperatorPrecedence::BitwiseAnd,
            MBinaryOperator::BitwiseOr => OperatorPrecedence::BitwiseOr,
        }
    }

    /// Determines whether a binary operator is commutative, meaning that the order of its operands
    /// does not affect the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use mlang_syntax::MBinaryOperator;
    ///
    /// let times = MBinaryOperator::Times;
    ///
    /// assert!(times.is_commutative());
    ///
    ///  let plus = MBinaryOperator::Plus; // Non-commutative operator
    /// assert!(!plus.is_commutative());
    /// ```
    pub const fn is_commutative(&self) -> bool {
        matches!(
            self,
            MBinaryOperator::Times | MBinaryOperator::BitwiseAnd | MBinaryOperator::BitwiseOr
        )
    }
}

impl MBinaryExpression {
    pub fn operator(&self) -> SyntaxResult<MBinaryOperator> {
        let kind = match self.operator_token()?.kind() {
            T![<] => MBinaryOperator::LessThan,
            T![>] => MBinaryOperator::GreaterThan,
            T![<=] => MBinaryOperator::LessThanOrEqual,
            T![>=] => MBinaryOperator::GreaterThanOrEqual,
            T![==] => MBinaryOperator::Equality,
            T![!=] => MBinaryOperator::Inequality,
            T![+] => MBinaryOperator::Plus,
            T![-] => MBinaryOperator::Minus,
            T![*] => MBinaryOperator::Times,
            T![/] => MBinaryOperator::Divide,
            T![%] => MBinaryOperator::Remainder,
            T![&] => MBinaryOperator::BitwiseAnd,
            T![|] => MBinaryOperator::BitwiseOr,
            _ => unreachable!(),
        };

        Ok(kind)
    }

    /// Whether this is a numeric operation, such as `+`, `-`, `*`, '/', `%`.
    pub fn is_numeric_operation(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![+] | T![-] | T![*] | T![/] | T![%])
        )
    }

    /// Whether this is a binary operation, such as `&`, `|`, `^`.
    pub fn is_binary_operation(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![&] | T![|] | T![^])
        )
    }

    /// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, etc.
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![>] | T![<] | T![>=] | T![<=] | T![==] | T![!=])
        )
    }

    /// Whether this is a comparison operation similar to the optional chain
    /// ```JavaScript
    /// foo != null;
    ///```
    pub fn is_optional_chain_like(&self) -> SyntaxResult<bool> {
        if matches!(self.operator(), Ok(MBinaryOperator::Inequality)) {
            Ok(self
                .right()?
                .as_static_value()
                .is_some_and(|x| x.is_null_or_undefined()))
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum MLogicalOperator {
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,
}

impl MLogicalOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            MLogicalOperator::LogicalOr => OperatorPrecedence::LogicalOr,
            MLogicalOperator::LogicalAnd => OperatorPrecedence::LogicalAnd,
        }
    }
}

impl MLogicalExpression {
    pub fn operator(&self) -> SyntaxResult<MLogicalOperator> {
        let kind = match self.operator_token()?.kind() {
            T![&&] | T![and] => MLogicalOperator::LogicalAnd,
            T![||] | T![or] => MLogicalOperator::LogicalOr,
            _ => unreachable!(),
        };

        Ok(kind)
    }
}

impl MArrayHole {
    pub fn hole_token(&self) -> Option<MSyntaxToken> {
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MUnaryOperator {
    /// `delete`
    Delete,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `!`
    LogicalNot,
    /// 'classof'
    Classof,
}

impl MUnaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl MUnaryExpression {
    pub fn operator(&self) -> SyntaxResult<MUnaryOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![+] => MUnaryOperator::Plus,
            T![-] => MUnaryOperator::Minus,
            T![!] => MUnaryOperator::LogicalNot,
            T![delete] => MUnaryOperator::Delete,
            T![classof] => MUnaryOperator::Classof,
            _ => unreachable!(),
        })
    }

    /// This function checks that `MUnaryExpression` is a signed numeric literal:
    /// ```JavaScript
    ///     +123
    ///     -321
    /// ```
    pub fn is_signed_numeric_literal(&self) -> SyntaxResult<bool> {
        let argument = self.argument()?;

        let is_signed = matches!(
            self.operator()?,
            MUnaryOperator::Plus | MUnaryOperator::Minus
        );

        let is_numeric_literal = matches!(
            argument,
            AnyMExpression::AnyMLiteralExpression(AnyMLiteralExpression::MNumberLiteralExpression(
                _
            ))
        );

        Ok(is_signed && is_numeric_literal)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MPreUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl MPreUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl MPreUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<MPreUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => MPreUpdateOperator::Increment,
            T![--] => MPreUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MPostUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl MPostUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl MPostUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<MPostUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => MPostUpdateOperator::Increment,
            T![--] => MPostUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

impl MArrayExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.elements().trailing_separator().is_some()
    }
}

impl MObjectExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.members().trailing_separator().is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.members().is_empty()
    }
}

impl MStringLiteralExpression {
    /// Get the inner text of a string not including the quotes
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl MLongStringLiteralExpression {
    /// Get the inner text of a string not including the quotes
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl AnyMExpression {
    /// Try to extract non `MParenthesizedExpression` from `MAnyExpression`
    pub fn omit_parentheses(self) -> AnyMExpression {
        let first = self
            .as_m_parenthesized_expression()
            .and_then(|expression| expression.expression().ok());

        iter::successors(first, |expression| {
            let parenthesized = expression.as_m_parenthesized_expression()?;
            parenthesized.expression().ok()
        })
        .last()
        .unwrap_or(self)
    }

    pub fn precedence(&self) -> SyntaxResult<OperatorPrecedence> {
        let precedence = match self {
            AnyMExpression::MSequenceExpression(_) => OperatorPrecedence::Comma,
            AnyMExpression::MConditionalExpression(_) => OperatorPrecedence::Conditional,
            AnyMExpression::MAssignmentExpression(_) => OperatorPrecedence::Assignment,
            AnyMExpression::MInExpression(_) | AnyMExpression::MInstanceofExpression(_) => {
                OperatorPrecedence::Relational
            }
            AnyMExpression::MLogicalExpression(expression) => expression.operator()?.precedence(),
            AnyMExpression::MBinaryExpression(expression) => expression.operator()?.precedence(),
            AnyMExpression::MUnaryExpression(_) => OperatorPrecedence::Unary,
            AnyMExpression::MPostUpdateExpression(_) | AnyMExpression::MPreUpdateExpression(_) => {
                OperatorPrecedence::Update
            }
            AnyMExpression::MCallExpression(_) | AnyMExpression::MSuperExpression(_) => {
                OperatorPrecedence::LeftHandSide
            }

            AnyMExpression::MNewExpression(expression) => {
                if expression.arguments().is_none() {
                    OperatorPrecedence::NewWithoutArguments
                } else {
                    OperatorPrecedence::LeftHandSide
                }
            }
            AnyMExpression::MComputedMemberExpression(_)
            | AnyMExpression::MStaticMemberExpression(_) => OperatorPrecedence::Member,

            AnyMExpression::MThisExpression(_)
            | AnyMExpression::AnyMLiteralExpression(_)
            | AnyMExpression::MArrayExpression(_)
            | AnyMExpression::MFunctionExpression(_)
            | AnyMExpression::MIdentifierExpression(_)
            | AnyMExpression::MConstantExpression(_)
            | AnyMExpression::MObjectExpression(_)
            | AnyMExpression::MHashMapExpression(_)
            | AnyMExpression::MHashSetExpression(_) => OperatorPrecedence::Primary,

            AnyMExpression::MBogusExpression(_) => OperatorPrecedence::lowest(),
            AnyMExpression::MParenthesizedExpression(_) => OperatorPrecedence::highest(),
        };

        Ok(precedence)
    }

    /// Return identifier if the expression is an identifier expression.
    pub fn as_m_reference_identifier(&self) -> Option<MReferenceIdentifier> {
        self.as_m_identifier_expression()?.name().ok()
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            AnyMExpression::AnyMLiteralExpression(literal) => literal.as_static_value(),
            _ => None,
        }
    }

    pub fn get_callee_object_name(&self) -> Option<MSyntaxToken> {
        let identifier = self.get_callee_object_identifier()?;
        identifier.value_token().ok()
    }

    pub fn get_callee_object_identifier(&self) -> Option<MReferenceIdentifier> {
        match self {
            AnyMExpression::MStaticMemberExpression(node) => {
                let member = node.object().ok()?;
                member.as_m_identifier_expression()?.name().ok()
            }
            AnyMExpression::MIdentifierExpression(node) => node.name().ok(),
            _ => None,
        }
    }

    pub fn get_callee_member_name(&self) -> Option<MSyntaxToken> {
        match self {
            AnyMExpression::MStaticMemberExpression(node) => {
                let member = node.member().ok()?;
                member.value_token().ok()
            }
            AnyMExpression::MIdentifierExpression(node) => node.name().ok()?.value_token().ok(),
            _ => None,
        }
    }

    /// Determining if an expression is literal
    /// - Any literal: 1, true, null, etc
    /// - Negative numeric literal: -1
    /// - Parenthesized expression: (1)
    pub fn is_literal_expression(&self) -> bool {
        match self {
            // Any literal: 1, true, null, etc
            AnyMExpression::AnyMLiteralExpression(_) => true,

            // Negative numeric literal: -1
            AnyMExpression::MUnaryExpression(unary_expression) => {
                let is_minus_operator =
                    matches!(unary_expression.operator(), Ok(MUnaryOperator::Minus));
                let is_number_expression = matches!(
                    unary_expression.argument(),
                    Ok(AnyMExpression::AnyMLiteralExpression(
                        AnyMLiteralExpression::MNumberLiteralExpression(_)
                    ))
                );

                is_minus_operator && is_number_expression
            }

            // Parenthesized expression: (1)
            AnyMExpression::MParenthesizedExpression(parenthesized_expression) => {
                parenthesized_expression
                    .expression()
                    .is_ok_and(|expression| expression.is_literal_expression())
            }

            _ => false,
        }
    }
}

impl AnyMLiteralExpression {
    pub fn value_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMLiteralExpression::MBooleanLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyMLiteralExpression::MNullLiteralExpression(expression) => expression.value_token(),
            AnyMLiteralExpression::MNumberLiteralExpression(expression) => expression.value_token(),
            AnyMLiteralExpression::MStringLiteralExpression(expression) => expression.value_token(),
            AnyMLiteralExpression::MLongStringLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyMLiteralExpression::MDateLiteralExpression(expression) => expression.value_token(),
            AnyMLiteralExpression::MTimeLiteralExpression(expression) => expression.value_token(),
        }
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            AnyMLiteralExpression::MBooleanLiteralExpression(boolean) => {
                Some(StaticValue::Boolean(boolean.value_token().ok()?))
            }
            AnyMLiteralExpression::MNullLiteralExpression(null) => {
                Some(StaticValue::Null(null.value_token().ok()?))
            }
            AnyMLiteralExpression::MNumberLiteralExpression(number) => {
                Some(StaticValue::Number(number.value_token().ok()?))
            }
            AnyMLiteralExpression::MStringLiteralExpression(string) => {
                Some(StaticValue::String(string.value_token().ok()?))
            }
            AnyMLiteralExpression::MLongStringLiteralExpression(string) => {
                Some(StaticValue::String(string.value_token().ok()?))
            }
            AnyMLiteralExpression::MDateLiteralExpression(date) => {
                Some(StaticValue::Date(date.value_token().ok()?))
            }
            AnyMLiteralExpression::MTimeLiteralExpression(time) => {
                Some(StaticValue::Time(time.value_token().ok()?))
            }
        }
    }
}

declare_node_union! {
    pub AnyMComputedMember = MComputedMemberExpression | MComputedMemberAssignment
}

impl AnyMComputedMember {
    pub fn object(&self) -> Option<AnyMExpression> {
        match self {
            AnyMComputedMember::MComputedMemberExpression(expression) => expression.object(),
            AnyMComputedMember::MComputedMemberAssignment(assignment) => assignment.object(),
        }
    }

    pub fn l_brack_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMComputedMember::MComputedMemberExpression(expression) => expression.l_brack_token(),
            AnyMComputedMember::MComputedMemberAssignment(assignment) => assignment.l_brack_token(),
        }
    }

    pub fn member(&self) -> SyntaxResult<AnyMExpression> {
        match self {
            AnyMComputedMember::MComputedMemberExpression(expression) => expression.member(),
            AnyMComputedMember::MComputedMemberAssignment(assignment) => assignment.member(),
        }
    }

    pub fn r_brack_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMComputedMember::MComputedMemberExpression(expression) => expression.r_brack_token(),
            AnyMComputedMember::MComputedMemberAssignment(assignment) => assignment.r_brack_token(),
        }
    }
}

impl AnyMObjectMemberName {
    /// Returns the member name of the current node
    /// if it is a literal member name or a computed member with a literal value.
    pub fn name(&self) -> Option<TokenText> {
        let token = match self {
            AnyMObjectMemberName::MComputedMemberName(expr) => {
                let expr = expr.expression().ok()?;
                match expr.omit_parentheses() {
                    AnyMExpression::AnyMLiteralExpression(expr) => expr.value_token().ok()?,
                    _ => return None,
                }
            }
            AnyMObjectMemberName::MLiteralMemberName(expr) => expr.value().ok()?,
        };
        Some(inner_string_text(&token))
    }
}
