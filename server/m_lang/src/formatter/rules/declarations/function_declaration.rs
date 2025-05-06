use crate::formatter::prelude::*;

use crate::formatter::rules::expressions::call_arguments::GroupedCallArgumentLayout;
use crate::formatter::utils::function_body::{
    FormatMaybeCachedFunctionBody, FunctionBodyCacheMode,
};
use crate::syntax::{
    AnyMFunctionBinding, AnyMStringLiteralExpression, MAnnotationGroupList, MFunctionBody,
    MFunctionDeclaration, MFunctionExpression, MParameters, MSyntaxToken,
};
use biome_formatter::{write, RemoveSoftLinesBuffer};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMFunctionDeclaration;
impl_format_with_rule!(MFunctionDeclaration, FormatMFunctionDeclaration);

impl FormatNodeRule<MFunctionDeclaration> for FormatMFunctionDeclaration {
    fn fmt_fields(&self, node: &MFunctionDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [FormatFunction::from(node.clone())]]
    }
}

declare_node_union! {
    pub(crate) FormatFunction =
        MFunctionDeclaration |
        MFunctionExpression
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct FormatFunctionOptions {
    pub call_argument_layout: Option<GroupedCallArgumentLayout>,
    pub body_cache_mode: FunctionBodyCacheMode,
}

impl FormatFunction {
    fn annotation(&self) -> Option<MAnnotationGroupList> {
        match self {
            FormatFunction::MFunctionDeclaration(declaration) => Some(declaration.annotation()),
            FormatFunction::MFunctionExpression(_) => None,
        }
    }

    fn function_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            FormatFunction::MFunctionDeclaration(declaration) => declaration.function_token(),
            FormatFunction::MFunctionExpression(expression) => expression.function_token(),
        }
    }

    fn id(&self) -> SyntaxResult<Option<AnyMFunctionBinding>> {
        match self {
            FormatFunction::MFunctionDeclaration(declaration) => declaration.id().map(Some),
            FormatFunction::MFunctionExpression(_) => Ok(None),
        }
    }

    fn parameters(&self) -> SyntaxResult<MParameters> {
        match self {
            FormatFunction::MFunctionDeclaration(declaration) => declaration.parameters(),
            FormatFunction::MFunctionExpression(expression) => expression.parameters(),
        }
    }

    fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        match self {
            FormatFunction::MFunctionDeclaration(declaration) => declaration.doc_string(),
            FormatFunction::MFunctionExpression(_) => None,
        }
    }

    fn body(&self) -> SyntaxResult<Option<MFunctionBody>> {
        Ok(match self {
            FormatFunction::MFunctionDeclaration(declaration) => Some(declaration.body()?),
            FormatFunction::MFunctionExpression(expression) => Some(expression.body()?),
        })
    }

    /// Formats the function with the specified `options`.
    ///
    /// # Errors
    ///
    /// Returns [`FormatError::PoorLayout`] if [`call_argument_layout`](FormatFunctionOptions::call_argument_layout] is `Some`
    /// and the function parameters contain some content that [*force a group to break*](FormatElements::will_break).
    ///
    /// This error is handled by [FormatMCallArguments].
    pub(crate) fn fmt_with_options(
        &self,
        f: &mut MFormatter,
        options: &FormatFunctionOptions,
    ) -> FormatResult<()> {
        if let Some(annotation) = self.annotation() {
            write!(f, [annotation.format()])?;
        }

        write!(f, [self.function_token().format()])?;

        match self.id()? {
            Some(id) => {
                write!(f, [space(), id.format()])?;
            }
            None => {
                write!(f, [space()])?;
            }
        }

        let parameters = self.parameters()?;

        let format_parameters = format_with(|f: &mut MFormatter| {
            if options.call_argument_layout.is_some() {
                let mut buffer = RemoveSoftLinesBuffer::new(f);

                let mut recording = buffer.start_recording();
                write!(recording, [parameters.format()])?;
                let recorded = recording.stop();

                if recorded.will_break() {
                    return Err(FormatError::PoorLayout);
                }
            } else {
                parameters.format().fmt(f)?;
            }

            Ok(())
        });

        write!(
            f,
            [group(&format_with(|f| { write!(f, [format_parameters]) }))]
        )?;

        if let Some(doc_string) = self.doc_string() {
            write!(
                f,
                [hard_line_break(), doc_string.format(), hard_line_break()]
            )?;
        }

        if let Some(body) = self.body()? {
            write!(
                f,
                [FormatMaybeCachedFunctionBody {
                    body: &body.into(),
                    mode: options.body_cache_mode
                }]
            )?;
        }

        Ok(())
    }
}

impl Format<MFormatContext> for FormatFunction {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        self.fmt_with_options(f, &FormatFunctionOptions::default())?;
        Ok(())
    }
}
