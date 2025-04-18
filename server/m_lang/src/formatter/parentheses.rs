// #[cfg(test)]
// pub(crate) mod tests {
//     use crate::syntax::parentheses::NeedsParentheses;
//     use crate::syntax::{MFileSource, MLanguage};
//     use crate::transform;
//     use biome_rowan::AstNode;

//     pub(crate) fn assert_needs_parentheses_impl<
//         T: AstNode<Language = MLanguage> + std::fmt::Debug + NeedsParentheses,
//     >(
//         input: &'static str,
//         index: Option<usize>,
//         source_type: MFileSource,
//     ) {
//         let parse = crate::parser::parse(input, source_type);

//         let diagnostics = parse.diagnostics();
//         assert!(
//             diagnostics.is_empty(),
//             "Expected input program to not have syntax errors but had {diagnostics:?}"
//         );

//         let root = parse.syntax();
//         let (transformed, _) = transform(root);
//         let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

//         let node = if let Some(index) = index {
//             matching_nodes.get(index).unwrap_or_else(|| {
//                 panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
//             })
//         } else {
//             match matching_nodes.len() {
//                 0 => {
//                     panic!(
//                         "Expected to find a '{}' node in '{input}' but found none.",
//                         core::any::type_name::<T>(),
//                     )
//                 }
//                 1 => matching_nodes.first().unwrap(),
//                 _ => {
//                     panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
//                 }
//             }
//         };

//         assert!(node.needs_parentheses());
//     }

//     pub(crate) fn assert_not_needs_parentheses_impl<
//         T: AstNode<Language = MLanguage> + std::fmt::Debug + NeedsParentheses,
//     >(
//         input: &'static str,
//         index: Option<usize>,
//         source_type: MFileSource,
//     ) {
//         let parse = biome_M_parser::parse(input, source_type, MParserOptions::default());

//         let diagnostics = parse.diagnostics();
//         assert!(
//             diagnostics.is_empty(),
//             "Expected input program to not have syntax errors but had {diagnostics:?}"
//         );

//         let root = parse.syntax();
//         let (transformed, _) = transform(root);
//         let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

//         let node = if let Some(index) = index {
//             matching_nodes.get(index).unwrap_or_else(|| {
//                 panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
//             })
//         } else {
//             match matching_nodes.len() {
//                 0 => {
//                     panic!(
//                         "Expected to find a '{}' node in '{input}' but found none.",
//                         core::any::type_name::<T>(),
//                     )
//                 }
//                 1 => matching_nodes.first().unwrap(),
//                 _ => {
//                     panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
//                 }
//             }
//         };

//         assert!(!node.needs_parentheses());
//     }

//     /// Helper macro to test the [NeedsParentheses] implementation of a node.
//     ///
//     /// # Example
//     ///
//     ///
//     /// ```
//     /// # use biome_M_formatter::assert_needs_parentheses;
//     /// use biome_M_syntax::MStaticMemberExpression;
//     ///
//     /// assert_needs_parentheses!("new (test().a)()", MStaticMemberExpression);
//     /// ```
//     ///
//     /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [MStaticMemberExpression] in the program.
//     ///
//     /// ```
//     /// # use biome_M_syntax::MStaticMemberExpression;
//     /// use biome_M_formatter::assert_needs_parentheses;
//     ///
//     /// assert_needs_parentheses!("new (test().a).b)()", MStaticMemberExpression[1]);
//     /// ```
//     ///
//     /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the second (in pre-order) [MStaticMemberExpression] in the program.
//     #[macro_export]
//     macro_rules! assert_needs_parentheses {
//         ($input:expr, $Node:ident) => {{
//             $crate::assert_needs_parentheses!($input, $Node, biome_M_syntax::MFileSource::ts())
//         }};

//         ($input:expr, $Node:ident[$index:expr]) => {{
//             $crate::assert_needs_parentheses!(
//                 $input,
//                 $Node[$index],
//                 biome_M_syntax::MFileSource::ts()
//             )
//         }};

//         ($input:expr, $Node:ident, $source_type: expr) => {{
//             $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
//                 $input,
//                 None,
//                 $source_type,
//             )
//         }};

//         ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
//             $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
//                 $input,
//                 Some($index),
//                 $source_type,
//             )
//         }};
//     }

//     /// Helper macro to test the [NeedsParentheses] implementation of a node.
//     ///
//     /// # Example
//     ///
//     ///
//     /// ```
//     /// # use biome_M_syntax::MStaticMemberExpression;
//     /// use biome_M_formatter::assert_not_needs_parentheses;
//     ///
//     /// assert_not_needs_parentheses!("a.b", MStaticMemberExpression);
//     /// ```
//     ///
//     /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [MStaticMemberExpression] in the program.
//     ///
//     /// ```
//     /// # use biome_M_syntax::MStaticMemberExpression;
//     /// use biome_M_formatter::assert_not_needs_parentheses;
//     ///
//     /// assert_not_needs_parentheses!("a.b.c", MStaticMemberExpression[0]);
//     /// ```
//     ///
//     /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the first (in pre-order) [MStaticMemberExpression] in the program.
//     #[macro_export]
//     macro_rules! assert_not_needs_parentheses {
//         ($input:expr, $Node:ident) => {{
//             $crate::assert_not_needs_parentheses!($input, $Node, biome_M_syntax::MFileSource::ts())
//         }};

//         ($input:expr, $Node:ident[$index:expr]) => {{
//             $crate::assert_not_needs_parentheses!(
//                 $input,
//                 $Node[$index],
//                 biome_M_syntax::MFileSource::ts()
//             )
//         }};

//         ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
//             $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
//                 $input,
//                 Some($index),
//                 $source_type,
//             )
//         }};

//         ($input:expr, $Node:ident, $source_type: expr) => {{
//             $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
//                 $input,
//                 None,
//                 $source_type,
//             )
//         }};
//     }
// }
