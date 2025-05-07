use super::prelude::*;
use crate::syntax::{
    AnyMRoot, AnyMStatement, MBlockStatement, MCallArguments, MCatchClause, MClassDeclaration,
    MConditionalExpression, MEmptyStatement, MFinallyClause, MFormalParameter, MFunctionBody,
    MIdentifierExpression, MIfStatement, MLanguage, MName, MSyntaxKind, MSyntaxNode,
    MVariableDeclarator, MWhileStatement,
};

use biome_formatter::{
    comments::{
        CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments,
        DecoratedComment, SourceComment,
    },
    write,
};
use biome_rowan::{AstNode, SyntaxNodeOptionExt, SyntaxTriviaPieceComments};

pub type MComments = Comments<MLanguage>;

#[derive(Default)]
pub(crate) struct FormatMLeadingComment;

impl FormatRule<SourceComment<MLanguage>> for FormatMLeadingComment {
    type Context = MFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<MLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct MCommentStyle;

impl CommentStyle for MCommentStyle {
    type Language = MLanguage;

    fn is_suppression(_text: &str) -> bool {
        false
    }

    fn get_comment_kind(_omment: &SyntaxTriviaPieceComments<MLanguage>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        match comment.text_position() {
            CommentTextPosition::EndOfLine => handle_function_comment(comment)
                .or_else(handle_conditional_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_class_comment)
                .or_else(handle_method_comment)
                .or_else(handle_for_comment)
                .or_else(handle_root_comments)
                .or_else(handle_variable_declarator_comment)
                .or_else(handle_parameter_comment)
                .or_else(handle_call_expression_comment)
                .or_else(handle_continue_break_comment)
                .or_else(handle_switch_default_case_comment),
            CommentTextPosition::OwnLine => handle_member_expression_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_while_comment)
                .or_else(handle_try_comment)
                .or_else(handle_class_comment)
                .or_else(handle_method_comment)
                .or_else(handle_for_comment)
                .or_else(handle_root_comments)
                .or_else(handle_parameter_comment)
                .or_else(handle_call_expression_comment)
                .or_else(handle_continue_break_comment)
                .or_else(handle_class_method_comment),
            CommentTextPosition::SameLine => {
                unreachable!("There can be no tokens after the comment.")
            }
        }
    }
}

fn handle_call_expression_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    // Make comments between the callee and the arguments leading comments of the first argument.
    // ```javascript
    // a /* comment */ (call)
    // ```
    if let Some(arguments) = comment.following_node().and_then(MCallArguments::cast_ref) {
        return if let Some(Ok(first)) = arguments.args().first() {
            CommentPlacement::leading(first.into_syntax(), comment)
        } else {
            CommentPlacement::dangling(arguments.into_syntax(), comment)
        };
    }

    CommentPlacement::Default(comment)
}

fn handle_continue_break_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    let enclosing = comment.enclosing_node();

    // Make comments between the `continue` and label token trailing comments
    // ```javascript
    // continue /* comment */ a;
    // ```
    // This differs from Prettier because other ASTs use an identifier for the label whereas Biome uses
    // a token.
    match enclosing.kind() {
        MSyntaxKind::M_CONTINUE_STATEMENT | MSyntaxKind::M_BREAK_STATEMENT => {
            match enclosing.parent() {
                // Make it the trailing of the parent if this is a single-statement body
                // to prevent that the comment becomes a trailing comment of the parent when re-formatting
                // ```javascript
                // for (;;) continue /* comment */;
                // ```
                Some(parent)
                    if matches!(
                        parent.kind(),
                        MSyntaxKind::M_FOR_STATEMENT
                            | MSyntaxKind::M_FOR_ALL_STATEMENT
                            | MSyntaxKind::M_FOR_ALL_IN_STATEMENT
                            | MSyntaxKind::M_WHILE_STATEMENT
                            | MSyntaxKind::M_IF_STATEMENT
                    ) =>
                {
                    CommentPlacement::trailing(parent, comment)
                }
                _ => CommentPlacement::trailing(enclosing.clone(), comment),
            }
        }
        _ => CommentPlacement::Default(comment),
    }
}

/// Moves line comments after the `default` keyword into the block statement:
///
/// ```javascript
/// switch (x) {
///     default: // comment
///     {
///         break;
///     }
/// ```
///
/// All other same line comments will use `Default` placement if they have a preceding node.
/// ```javascript
/// switch(x) {
///     default:
///         a(); // asd
///         break;
/// }
/// ```
///
/// All other comments become `Dangling` comments that are handled inside of the default case
/// formatting.
fn handle_switch_default_case_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    if comment.enclosing_node().kind() != MSyntaxKind::M_DEFAULT_CLAUSE {
        return CommentPlacement::Default(comment);
    }

    if !comment.kind().is_line() {
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    }

    let Some(block) = comment.following_node().and_then(MBlockStatement::cast_ref) else {
        if comment.preceding_node().is_some() {
            return CommentPlacement::Default(comment);
        }
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    };

    place_block_statement_comment(block, comment)
}

fn handle_class_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    // Make comments following after the `extends` or `implements` keyword trailing comments
    // of the preceding extends, type parameters, or id.
    // ```javascript
    // class a9 extends
    //   /* comment */
    // b {
    //   constructor() {}
    // }
    // ```
    if matches!(
        comment.enclosing_node().kind(),
        MSyntaxKind::M_EXTENDS_CLAUSE
    ) {
        if comment.preceding_node().is_none() && !comment.text_position().is_same_line() {
            if let Some(sibling) = comment.enclosing_node().prev_sibling() {
                return CommentPlacement::trailing(sibling, comment);
            }
        }

        return CommentPlacement::Default(comment);
    }

    let first_member = if let Some(class) = MClassDeclaration::cast_ref(comment.enclosing_node()) {
        class.members().first().map(AstNode::into_syntax)
    } else {
        return CommentPlacement::Default(comment);
    };

    if comment.text_position().is_same_line() {
        // Handle same line comments in empty class bodies
        // ```javascript
        // class Test { /* comment */ }
        // ```
        if comment
            .following_token()
            .is_some_and(|token| token.kind() == MSyntaxKind::R_CURLY)
            && first_member.is_none()
        {
            return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
        } else {
            return CommentPlacement::Default(comment);
        }
    }

    if let Some(following) = comment.following_node() {
        // Make comments preceding the first member leading comments of the member
        // ```javascript
        // class Test { /* comment */
        //      prop;
        // }
        // ```
        if let Some(member) = first_member {
            if following == &member {
                return CommentPlacement::leading(member, comment);
            }
        }
    } else if first_member.is_none() {
        // Handle the case where there are no members, attach the comments as dangling comments.
        // ```javascript
        // class Test // comment
        // {
        // }
        // ```
        return CommentPlacement::dangling(comment.enclosing_node().clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_method_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    let enclosing = comment.enclosing_node();

    let is_method = matches!(
        enclosing.kind(),
        MSyntaxKind::M_METHOD_CLASS_MEMBER
            | MSyntaxKind::M_SETTER_CLASS_MEMBER
            | MSyntaxKind::M_GETTER_CLASS_MEMBER
            | MSyntaxKind::M_CONSTRUCTOR_CLASS_MEMBER
    );

    if !is_method {
        return CommentPlacement::Default(comment);
    }

    // Move end of line and own line comments before the method body into the function
    // ```javascript
    // class Test {
    //   method() /* test */
    //  {}
    // }
    // ```
    if let Some(following) = comment.following_node() {
        if let Some(body) = MFunctionBody::cast_ref(following) {
            if let Some(directive) = body.directives().first() {
                return CommentPlacement::leading(directive.into_syntax(), comment);
            }

            let first_non_empty = body
                .statements()
                .iter()
                .find(|statement| !matches!(statement, AnyMStatement::MEmptyStatement(_)));

            return match first_non_empty {
                None => CommentPlacement::dangling(body.into_syntax(), comment),
                Some(statement) => CommentPlacement::leading(statement.into_syntax(), comment),
            };
        }
    }

    CommentPlacement::Default(comment)
}

/// Handle a all comments document.
/// See `blank.M`
fn handle_root_comments(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    if let Some(root) = AnyMRoot::cast_ref(comment.enclosing_node()) {
        let is_blank = match &root {
            AnyMRoot::MExpressionSnipped(_) => false,
            AnyMRoot::MModule(module) => {
                module.directives().is_empty() && module.items().is_empty()
            }
            AnyMRoot::MScript(script) => script.statements().is_empty(),
        };

        if is_blank {
            return CommentPlacement::leading(root.into_syntax(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_member_expression_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                MSyntaxKind::M_STATIC_MEMBER_EXPRESSION | MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // ```javascript
    // a
    // /* comment */.b
    // a
    // /* comment */[b]
    // ```
    if MName::can_cast(following.kind()) || MIdentifierExpression::can_cast(following.kind()) {
        CommentPlacement::leading(comment.enclosing_node().clone(), comment)
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_function_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    if !matches!(
        comment.enclosing_node().kind(),
        MSyntaxKind::M_FUNCTION_DECLARATION | MSyntaxKind::M_FUNCTION_EXPRESSION
    ) {
        return CommentPlacement::Default(comment);
    };

    let Some(body) = comment.following_node().and_then(MFunctionBody::cast_ref) else {
        return CommentPlacement::Default(comment);
    };

    // Make line comments between the `)` token and the function empty body leading
    // ```javascript
    // function test() // comment
    // {
    //  console.log("Hy");
    // }
    // ```
    match body.statements().first() {
        Some(_) => CommentPlacement::leading(comment.enclosing_node().clone(), comment),
        None => CommentPlacement::dangling(body.into_syntax(), comment),
    }
}

fn handle_conditional_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    let enclosing = comment.enclosing_node();

    let (conditional, following) = match (
        MConditionalExpression::cast_ref(enclosing),
        comment.following_node(),
    ) {
        (Some(conditional), Some(following)) => (conditional, following),
        _ => {
            return CommentPlacement::Default(comment);
        }
    };

    // Make end of line comments that come after the operator leading comments of the consequent / alternate.
    // ```javascript
    // a
    //   // becomes leading of consequent
    //   ? { x: 5 } :
    //   {};
    //
    // a
    //   ? { x: 5 }
    //   : // becomes leading of alternate
    // 	{};
    //
    // a // remains trailing, because it directly follows the node
    //   ? { x: 5 }
    //   : {};
    // ```
    let token = comment.piece().as_piece().token();
    let is_after_operator = conditional.colon_token().as_ref() == Ok(&token)
        || conditional.question_mark_token().as_ref() == Ok(&token);

    if is_after_operator {
        return CommentPlacement::leading(following.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_if_statement_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    fn handle_else_clause(
        comment: DecoratedComment<MLanguage>,
        consequent: MSyntaxNode,
        if_statement: MSyntaxNode,
    ) -> CommentPlacement<MLanguage> {
        // Handle end of line comments that aren't stretching over multiple lines.
        // Make them dangling comments of the consequent expression
        //
        // ```javascript
        // if (cond1) expr1; // comment A
        // else if (cond2) expr2; // comment A
        // else expr3;
        //
        // if (cond1) expr1; /* comment */ else  expr2;
        //
        // if (cond1) expr1; /* b */
        // else if (cond2) expr2; /* b */
        // else expr3; /* b*/
        // ```
        if !comment.kind().is_block()
            && !comment.text_position().is_own_line()
            && comment.preceding_node().is_some()
        {
            return CommentPlacement::trailing(consequent, comment);
        }

        // ```javascript
        // if (cond1) expr1;
        //
        // /* comment */ else  expr2;
        //
        // if (cond) expr; /*
        // a multiline comment */
        // else b;
        //
        // if (6) // comment
        // true
        // else // comment
        // {true}
        // ```
        CommentPlacement::dangling(if_statement, comment)
    }

    match (comment.enclosing_node().kind(), comment.following_node()) {
        (MSyntaxKind::M_IF_STATEMENT, Some(following)) => {
            let if_statement = MIfStatement::unwrap_cast(comment.enclosing_node().clone());

            if let Some(preceding) = comment.preceding_node() {
                // Test if this is a comment right before the condition's `)`
                if comment
                    .following_token()
                    .is_some_and(|token| token.kind() == MSyntaxKind::R_PAREN)
                {
                    return CommentPlacement::trailing(preceding.clone(), comment);
                }

                // Handle comments before `else`
                if following.kind() == MSyntaxKind::M_ELSE_CLAUSE {
                    let consequent = preceding.clone();
                    let if_statement = comment.enclosing_node().clone();
                    return handle_else_clause(comment, consequent, if_statement);
                }
            }

            // Move comments coming before the `{` leading
            //
            // ```javascript
            // if (cond) // test
            // {
            // }
            // ```
            if let Some(_block_statement) = MBlockStatement::cast_ref(following) {
                return CommentPlacement::leading(if_statement.syntax().clone(), comment);
            }

            // Don't attach comments to empty statements
            // ```javascript
            // if (cond) /* test */ ;
            // ```
            if let Some(preceding) = comment.preceding_node() {
                if MEmptyStatement::can_cast(following.kind()) {
                    return CommentPlacement::trailing(preceding.clone(), comment);
                }
            }

            // Move comments coming before an if chain inside the body of the first non chain if.
            //
            // ```javascript
            // if (cond1)  /* test */ if (other) { a }
            // ```
            if let Some(if_statement) = MIfStatement::cast_ref(following) {
                if let Ok(nested_consequent) = if_statement.consequent() {
                    return place_leading_statement_comment(nested_consequent, comment);
                }
            }

            // Make all comments after the condition's `)` leading comments
            // ```javascript
            // if (5) // comment
            // true
            //
            // ```
            if let Ok(consequent) = if_statement.consequent() {
                if consequent.syntax() == following {
                    return CommentPlacement::leading(if_statement.syntax().clone(), comment);
                }
            }
        }
        (MSyntaxKind::M_ELSE_CLAUSE, _) => {
            if let Some(if_statement) = comment
                .enclosing_node()
                .parent()
                .and_then(MIfStatement::cast)
            {
                if let Ok(consequent) = if_statement.consequent() {
                    return handle_else_clause(
                        comment,
                        consequent.into_syntax(),
                        if_statement.into_syntax(),
                    );
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_while_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    let (while_statement, following) = match (
        MWhileStatement::cast_ref(comment.enclosing_node()),
        comment.following_node(),
    ) {
        (Some(while_statement), Some(following)) => (while_statement, following),
        _ => return CommentPlacement::Default(comment),
    };

    if let Some(preceding) = comment.preceding_node() {
        // Test if this is a comment right before the condition's `)`
        if comment
            .following_token()
            .is_some_and(|token| token.kind() == MSyntaxKind::R_PAREN)
        {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Move comments coming before the `{` to leading
    //
    // ```javascript
    // while (cond) // test
    // {
    // }
    // ```
    if MBlockStatement::cast_ref(following).is_some() {
        return CommentPlacement::leading(while_statement.syntax().clone(), comment);
    }

    // Don't attach comments to empty statements
    // ```javascript
    // if (cond) // test  ;
    // ```
    if let Some(preceding) = comment.preceding_node() {
        if MEmptyStatement::can_cast(following.kind()) {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Make all comments after the condition's `)` leading comments
    // ```javascript
    // while (5) // comment
    // true
    //
    // ```
    if let Ok(body) = while_statement.body() {
        if body.syntax() == following {
            return CommentPlacement::leading(while_statement.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_try_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    let following = match comment.following_node() {
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                MSyntaxKind::M_TRY_STATEMENT | MSyntaxKind::M_TRY_FINALLY_STATEMENT
            ) =>
        {
            // Move comments before the `catch` or `finally` inside of the body
            // ```javascript
            // try {
            // }
            //  catch(e) {
            // }
            // // Comment 7
            // finally {}
            // ```
            let body = if let Some(catch) = MCatchClause::cast_ref(following) {
                catch.body()
            } else if let Some(finally) = MFinallyClause::cast_ref(following) {
                finally.body()
            } else {
                // Use an err, so that the following code skips over it
                Err(biome_rowan::SyntaxError::MissingRequiredChild)
            };

            //
            // ```javascript
            // try {
            // } /* comment catch {
            // }
            // ```
            if let Ok(body) = body {
                return place_block_statement_comment(body, comment);
            }

            following
        }
        Some(following)
            if matches!(
                comment.enclosing_node().kind(),
                MSyntaxKind::M_CATCH_CLAUSE | MSyntaxKind::M_FINALLY_CLAUSE
            ) =>
        {
            following
        }
        _ => return CommentPlacement::Default(comment),
    };

    // Move comments coming before the `{` inside of the block
    //
    // ```javascript
    // try /* test */ {
    // }
    // ```
    if let Some(block) = MBlockStatement::cast_ref(following) {
        return place_block_statement_comment(block, comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_for_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    let enclosing = comment.enclosing_node();

    let is_for_all = matches!(
        enclosing.kind(),
        MSyntaxKind::M_FOR_ALL_STATEMENT | MSyntaxKind::M_FOR_ALL_IN_STATEMENT
    );

    if !is_for_all && !matches!(enclosing.kind(), MSyntaxKind::M_FOR_STATEMENT) {
        return CommentPlacement::Default(comment);
    }

    if comment.text_position().is_own_line() && is_for_all {
        CommentPlacement::leading(enclosing.clone(), comment)
    }
    // Don't attach comments to empty statement
    // ```javascript
    // for /* comment */ (;;);
    // for (;;a++) /* comment */;
    // ```
    else if comment
        .following_node()
        .is_some_and(|following| MEmptyStatement::can_cast(following.kind()))
    {
        if let Some(preceding) = comment.preceding_node() {
            CommentPlacement::trailing(preceding.clone(), comment)
        } else {
            CommentPlacement::dangling(comment.enclosing_node().clone(), comment)
        }
    } else {
        CommentPlacement::Default(comment)
    }
}

fn handle_variable_declarator_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    let following = match comment.following_node() {
        Some(following) => following,
        None => return CommentPlacement::Default(comment),
    };

    fn is_complex_value(value: &MSyntaxNode) -> bool {
        matches!(
            value.kind(),
            MSyntaxKind::M_OBJECT_EXPRESSION | MSyntaxKind::M_ARRAY_EXPRESSION
        )
    }

    let enclosing = comment.enclosing_node();
    match enclosing.kind() {
        MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
            // Makes all comments preceding objects/arrays/templates or block comments leading comments of these nodes.
            // ```javascript
            // let a = // comment
            // { };
            // ```
            if is_complex_value(following) || !comment.kind().is_line() {
                return CommentPlacement::leading(following.clone(), comment);
            }
        }
        MSyntaxKind::M_VARIABLE_DECLARATOR => {
            let variable_declarator = MVariableDeclarator::unwrap_cast(enclosing.clone());

            match variable_declarator.initializer() {
                // ```javascript
                // let obj2 // Comment
                // = {
                //   key: 'val'
                // }
                // ```
                Some(initializer)
                    if initializer.syntax() == following
                        && initializer
                            .expression()
                            .is_ok_and(|expression| is_complex_value(expression.syntax())) =>
                {
                    return CommentPlacement::leading(initializer.into_syntax(), comment);
                }
                _ => {
                    // fall through
                }
            }
        }
        MSyntaxKind::M_INITIALIZER_CLAUSE => {
            let parent_kind = enclosing.parent().kind();

            if matches!(
                parent_kind,
                Some(MSyntaxKind::M_VARIABLE_DECLARATOR | MSyntaxKind::M_PROPERTY_CLASS_MEMBER)
            ) {
                let not_complex = matches!(parent_kind, Some(MSyntaxKind::M_PROPERTY_CLASS_MEMBER))
                    || !is_complex_value(following);

                // Keep trailing comments with the id for variable declarators. Necessary because the value is wrapped
                // inside of an initializer clause.
                // ```javascript
                // let a = // comment
                //      b;
                // ```
                if not_complex
                    && !MCommentStyle::is_suppression(comment.piece().text())
                    && comment.kind().is_line()
                    && comment.preceding_node().is_none()
                {
                    if let Some(prev_node) = enclosing.prev_sibling() {
                        return CommentPlacement::trailing(prev_node, comment);
                    }
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_parameter_comment(comment: DecoratedComment<MLanguage>) -> CommentPlacement<MLanguage> {
    // Make all own line comments leading comments of the parameter
    // ```javascript
    // function a(
    //   b
    //   // comment
    //   = c
    // )
    // ```
    match comment.enclosing_node().kind() {
        MSyntaxKind::M_FORMAL_PARAMETER => {
            // Keep decorator comments near the decorator
            // Attach leading parameter comments to the last decorator
            // ```javascript
            // class Foo {
            // 	method(
            // 	//leading own line
            // 	/*leading same line*/ @Decorator /*trailing*/
            // 	//leading own line between
            // 	/*leading same line between*/ @dec //trailing
            // 	/*leading parameter*/
            // 	parameter
            // 	) {}
            // }
            // ```
            if comment.text_position().is_own_line() {
                return CommentPlacement::leading(comment.enclosing_node().clone(), comment);
            }
        }
        MSyntaxKind::M_INITIALIZER_CLAUSE => {
            if let Some(parameter) = comment
                .enclosing_node()
                .parent()
                .and_then(MFormalParameter::cast)
            {
                // Keep end of line comments after the `=` trailing comments of the id
                // ```javascript
                // function a(
                //   b = // test
                //     c
                // )
                // ```
                if comment.text_position().is_end_of_line() && comment.preceding_node().is_none() {
                    if let Ok(binding) = parameter.binding() {
                        return CommentPlacement::trailing(binding.into_syntax(), comment);
                    }
                } else if comment.text_position().is_own_line() {
                    return CommentPlacement::leading(parameter.into_syntax(), comment);
                }
            }
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

/// Attach comments before the `async` keyword or `*` token to the preceding node if it exists
/// to ensure these comments are placed before the `async` keyword or `*` token.
/// ```javascript
/// class Foo {
///    @decorator()
///    // comment
///    async method() {}
/// }
fn handle_class_method_comment(
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    let enclosing_node = comment.enclosing_node();
    match enclosing_node.kind() {
        MSyntaxKind::M_METHOD_CLASS_MEMBER => CommentPlacement::Default(comment),
        _ => CommentPlacement::Default(comment),
    }
}

fn place_leading_statement_comment(
    statement: AnyMStatement,
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    match statement {
        AnyMStatement::MBlockStatement(block) => place_block_statement_comment(block, comment),
        statement => CommentPlacement::leading(statement.into_syntax(), comment),
    }
}

fn place_block_statement_comment(
    block_statement: MBlockStatement,
    comment: DecoratedComment<MLanguage>,
) -> CommentPlacement<MLanguage> {
    let first_non_empty = block_statement
        .statements()
        .iter()
        .find(|statement| !matches!(statement, AnyMStatement::MEmptyStatement(_)));

    match first_non_empty {
        None => CommentPlacement::dangling(block_statement.into_syntax(), comment),
        Some(statement) => CommentPlacement::leading(statement.into_syntax(), comment),
    }
}
