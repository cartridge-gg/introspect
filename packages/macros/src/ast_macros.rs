#[macro_export]
macro_rules! terminal_to_string {

    {$($terminal:ident $(. $($methods:ident).+)?),* $(,)?} => {
        $(
            impl<'db> crate::AstToString<'db> for cairo_lang_syntax::node::ast::$terminal<'db> {

                fn to_string(&self, db: &'db dyn salsa::Database) -> String {
                    use cairo_lang_syntax::node::Terminal;
                    self$(.$($methods(db)).+)?.text(db).to_string(db)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! typed_syntax_node_to_string_without_trivia {
    {$typed_syntax_node:ident $(. $($methods:ident).+)?} => {
        impl<'db> crate::AstToString<'db> for cairo_lang_syntax::node::ast::$typed_syntax_node<'db> {
            fn to_string(&self, db: &'db dyn salsa::Database) -> String {
                use cairo_lang_syntax::node::TypedSyntaxNode;
                self$(.$($methods(db)).+)?.as_syntax_node().get_text_without_trivia(db).to_string(db)
            }
        }
    };
}

#[macro_export]
macro_rules! from_typed_syntax_node {
    {$typed_syntax_node:ident $(. $($methods:ident).+)?, $syntax_type:ident} => {
        impl<'db> crate::FromAst<'db, cairo_lang_syntax::node::ast::$typed_syntax_node<'db>> for $syntax_type {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$typed_syntax_node<'db>, db: &'db dyn salsa::Database) -> $syntax_type {
                use crate::AstInto;
                ast$(.$($methods(db)).+)?.ast_into(db)
            }
        }
    };
}

#[macro_export]
macro_rules! vec_from_element_list {
    {$list:ident $(. $($methods:ident).+)?, $element:ident} => {
        impl<'db> crate::FromAst<'db, cairo_lang_syntax::node::ast::$list<'db>> for Vec<$element> {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$list<'db>, db: &'db dyn salsa::Database) -> Vec<$element> {
                use crate::AstInto;
                ast$(.$($methods(db)).+)?.elements(db).into_iter().map(|e| e.ast_into(db)).collect()
            }
        }
    };
}

#[macro_export]
macro_rules! vec_try_from_element_list {
    {$list:ident $(. $($methods:ident).+)?, $element:ident} => {
        impl<'db> TryFromAst<'db, cairo_lang_syntax::node::ast::$list<'db>> for Vec<$element> {
            fn try_from_ast(ast: cairo_lang_syntax::node::ast::$list<'db>, db: &'db dyn Database) -> IntrospectResult<Vec<$element>> {
                ast$(.$($methods(db)).+)?.elements(db).into_iter().map(|e| e.ast_try_into(db)).collect()
            }
        }
    };
}

#[macro_export]
macro_rules! from_ast {
    // Handle mixed explicit and shorthand fields
    {
        $ast_type:ident,
        $target_type:ident {
            $($field:ident$(: $ast_method:ident.$conversion:ident)?),* $(,)?
        }
    } => {
        impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_type<'db>> for $target_type {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$ast_type<'db>, db: &'db dyn salsa::Database) -> Self {
                $(
                    let $field = $crate::from_ast!(@convert ast, db, $field $(, $ast_method, $conversion)?);
                )*
                $target_type { $($field),* }
            }
        }
    };

    // Internal: explicit conversion
    {@convert $ast:ident, $db:ident, $field:ident, $ast_method:ident, $conversion:ident} => {
        $ast.$ast_method($db).$conversion($db)
    };

    // Internal: shorthand - defaults to field.ast_into
    {@convert $ast:ident, $db:ident, $field:ident} => {
        $ast.$field($db).ast_into($db)
    };
}

#[macro_export]
macro_rules! syntax_type {
    // With AST type override in square brackets
    {
        $struct_name:ident[$ast_type:ident] { $($field:ident $([ $method:ident ])?: $field_type:ty),* $(,)? }
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $struct_name {
            $(pub $field: $field_type),*
        }

        impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_type<'db>> for $struct_name {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$ast_type<'db>, db: &'db dyn salsa::Database) -> Self {
                $struct_name {
                    $(
                        $field: $crate::syntax_type!(@get_value ast, db, $field $(, $method)?),
                    )*
                }
            }
        }
    };

    // Without AST type override - use struct name
    {
        $struct_name:ident { $($field:ident $([ $method:ident ])?: $field_type:ty),* $(,)? }
    } => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $struct_name {
            $(pub $field: $field_type),*
        }

        impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$struct_name<'db>> for $struct_name {
            fn from_ast(ast: cairo_lang_syntax::node::ast::$struct_name<'db>, db: &'db dyn salsa::Database) -> Self {
                use crate::AstInto;
                $struct_name {
                    $(
                        $field: $crate::syntax_type!(@get_value ast, db, $field $(, $method)?),
                    )*
                }
            }
        }
    };

    (@get_value $ast:ident, $db:ident, $field:ident, $method:ident) => {
        $ast.$method($db).ast_into($db)
    };

    (@get_value $ast:ident, $db:ident, $field:ident) => {
        $ast.$field($db).ast_into($db)
    };
}

#[macro_export]
macro_rules! syntax_terminal_enum {
    // With AST type override
    {
        $enum_name:ident[$ast_enum:ident] {
            $($variant:ident $([ $terminal:ident ])? ),* $(,)?
        }
    } => {
        syntax_terminal_enum!(@impl $enum_name, $ast_enum, { $($variant $([$terminal])?),* });
    };

    // Without AST type override - use enum name
    {
        $enum_name:ident {
            $($variant:ident $([ $terminal:ident ])? ),* $(,)?
        }
    } => {
        syntax_terminal_enum!(@impl $enum_name, $enum_name, { $($variant $([$terminal])?),* });
    };

    // Implementation
    (@impl $enum_name:ident, $ast_enum:ident, { $($variant:ident $([ $terminal:ident ])?),* }) => {
        paste::paste! {
            #[derive(Clone, Debug, PartialEq)]
            pub enum $enum_name {
                $($variant),*
            }

            impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_enum<'db>> for $enum_name {
                fn from_ast(ast: cairo_lang_syntax::node::ast::$ast_enum<'db>, _db: &'db dyn salsa::Database) -> Self {
                    From::from(ast)
                }

                fn from_syntax_node(db: &'db dyn salsa::Database, node: cairo_lang_syntax::node::SyntaxNode<'db>) -> Self {
                    use cairo_lang_syntax::node::kind::SyntaxKind;
                    let kind = node.kind(db);
                    match kind {
                        $(
                            $crate::syntax_terminal_enum!(@terminal_kind $variant $(, $terminal)?) => $enum_name::$variant,
                        )*
                        _ => panic!("Node {:?} Not a {}", kind, stringify!($ast_enum)),
                    }
                }
            }

            impl<'db> From<cairo_lang_syntax::node::ast::$ast_enum<'db>> for $enum_name {
                fn from(ast: cairo_lang_syntax::node::ast::$ast_enum<'db>) -> Self {
                    match ast {
                        $(
                            cairo_lang_syntax::node::ast::$ast_enum::$variant(_) => $enum_name::$variant,
                        )*
                    }
                }
            }
        }
    };

    // With override: use the specified terminal
    (@terminal_kind $variant:ident, $terminal:ident) => {
        paste::paste! { SyntaxKind::$terminal }
    };

    // Without override: use variant name as terminal
    (@terminal_kind $variant:ident) => {
        paste::paste! { SyntaxKind::[<Terminal $variant>] }
    };
}

#[macro_export]
macro_rules! syntax_enum {
    ( $enum_name:ident { $( $items:tt )* } ) => {
        syntax_enum!(@emit $enum_name $enum_name { $( $items )* });
    };

    ( $enum_name:ident [ $ast_enum:ident ] { $( $items:tt )* } ) => {
        syntax_enum!(@emit $enum_name $ast_enum { $( $items )* });
    };

    (@emit $enum_name:ident $ast_enum:ident {
        $(
            $variant:ident
            $( [ $ast_variant:ident ] )?
            $( ( $ty:ty ) )?
        ),* $(,)?
    }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $enum_name {
            $( $variant $( ( $ty ) )?, )*
        }

        impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_enum<'db>> for $enum_name {
            fn from_ast(
                ast: cairo_lang_syntax::node::ast::$ast_enum<'db>,
                db: &'db dyn salsa::Database,
            ) -> Self {
                match ast {
                    $(
                        syntax_enum!(@pat $ast_enum $variant $( [ $ast_variant ] )? $( ( $ty ) )? __e)
                            => syntax_enum!(@expr $variant $( ( $ty ) )? __e, db),
                    )*
                }
            }
        }
    };

    // ---- pattern side (returns a pattern) ----
    (@pat $ast_enum:ident $variant:ident [ $ast_variant:ident ] ( $ty:ty ) $e:ident) => {
        cairo_lang_syntax::node::ast::$ast_enum::$ast_variant($e)
    };
    (@pat $ast_enum:ident $variant:ident ( $ty:ty ) $e:ident) => {
        cairo_lang_syntax::node::ast::$ast_enum::$variant($e)
    };
    (@pat $ast_enum:ident $variant:ident [ $ast_variant:ident ] $e:ident) => {
        cairo_lang_syntax::node::ast::$ast_enum::$ast_variant(_)
    };
    (@pat $ast_enum:ident $variant:ident $e:ident) => {
        cairo_lang_syntax::node::ast::$ast_enum::$variant(_)
    };

    // ---- expression side (returns an expression) ----
    (@expr $variant:ident ( $ty:ty ) $e:ident, $db:ident) => {
        Self::$variant($crate::AstInto::ast_into($e, $db))
    };
    (@expr $variant:ident $e:ident, $db:ident) => {
        Self::$variant
    };
}

// #[macro_export]
// macro_rules! syntax_enum {
//     // With AST type override
//     (
//         $enum_name:ident[$ast_type:ident] {
//             $($variant:ident $([$ast_name:ident])? $(($type:ty))?),* $(,)?
//         }
//     ) => {
//         syntax_enum!(@impl $enum_name, $ast_type, { $($variant $([$ast_name])? $(($type))?),* });
//     };

//     // Without AST type override - use enum name
//     (
//         $enum_name:ident {
//             $($variant:ident $([$ast_name:ident])? $(($type:ty))?),* $(,)?
//         }
//     ) => {
//         syntax_enum!(@impl $enum_name, $enum_name, { $($variant $([$ast_name])? $(($type))?),* });
//     };

//     // Main implementation - initiates recursive processing wrapped in paste! once
//     (@impl $enum_name:ident, $ast_type:ident, { $($variants:tt)* }) => {
//         syntax_enum!(@build_enum $enum_name, [], [$($variants)*]);
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [], [$($variants)*]);
//         }
//     };

//     // Build enum definition recursively - process one variant at a time
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]($type:ty) , $($rest:tt)*]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant($type),], [$($rest)*]);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident($type:ty) , $($rest:tt)*]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant($type),], [$($rest)*]);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident] , $($rest:tt)*]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant,], [$($rest)*]);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident , $($rest:tt)*]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant,], [$($rest)*]);
//     };
//     // Handle last variant without trailing comma
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]($type:ty)]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant($type),], []);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident($type:ty)]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant($type),], []);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant,], []);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], [$variant:ident]) => {
//         syntax_enum!(@build_enum $enum_name, [$($accum)* $variant,], []);
//     };
//     (@build_enum $enum_name:ident, [$($accum:tt)*], []) => {
//         #[derive(Clone, Debug, PartialEq)]
//         pub enum $enum_name { $($accum)* }
//     };

//     // Build FromAst impl recursively - process one variant at a time
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]($type:ty) , $($rest:tt)*]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$ast_name(expr) =>
//                     $enum_name::$variant(expr.ast_into(__db)),],
//                 [$($rest)*]);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident($type:ty) , $($rest:tt)*]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$variant(expr) =>
//                     $enum_name::$variant(expr.ast_into(__db)),],
//                 [$($rest)*]);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident] , $($rest:tt)*]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$ast_name(_) =>
//                     $enum_name::$variant,],
//                 [$($rest)*]);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident , $($rest:tt)*]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$variant(_) =>
//                     $enum_name::$variant,],
//                 [$($rest)*]);
//         }
//     };
//     // Handle last variant without trailing comma
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]($type:ty)]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$ast_name(expr) =>
//                     $enum_name::$variant(expr.ast_into(__db)),],
//                 []);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident($type:ty)]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$variant(expr) =>
//                     $enum_name::$variant(expr.ast_into(__db)),],
//                 []);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident[$ast_name:ident]]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$ast_name(_) =>
//                     $enum_name::$variant,],
//                 []);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], [$variant:ident]) => {
//         paste::paste! {
//             syntax_enum!(@build_impl $enum_name, $ast_type, [$($accum)*
//                 cairo_lang_syntax::node::ast::$ast_type::$variant(_) =>
//                     $enum_name::$variant,],
//                 []);
//         }
//     };
//     (@build_impl $enum_name:ident, $ast_type:ident, [$($accum:tt)*], []) => {
//         impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_type<'db>> for $enum_name {
//             fn from_ast(
//                 ast: cairo_lang_syntax::node::ast::$ast_type<'db>,
//                 __db: &'db dyn salsa::Database
//             ) -> Self {
//                 use crate::AstInto;
//                 match ast { $($accum)* }
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! syntax_terminal_bool {
    ($ast_type:ident) => {
        paste::paste! {
            impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::[<OptionTerminal $ast_type>]<'db>>
                for bool
            {
                fn from_ast(
                    ast: cairo_lang_syntax::node::ast::[<OptionTerminal $ast_type>]<'db>,
                    _db: &'db dyn salsa::Database,
                ) -> Self {
                    match ast {
                        cairo_lang_syntax::node::ast::[<OptionTerminal $ast_type>]::[<Terminal $ast_type>](_) => true,
                        cairo_lang_syntax::node::ast::[<OptionTerminal $ast_type>]::Empty(_) => false,
                    }
                }
                fn from_syntax_node(db: &'db dyn salsa::Database, node: cairo_lang_syntax::node::SyntaxNode<'db>) -> Self {
                    use cairo_lang_syntax::node::kind::SyntaxKind;
                    let kind = node.kind(db);
                    match kind {
                        SyntaxKind::[<Terminal $ast_type>] => true,
                        SyntaxKind::[<OptionTerminal $ast_type Empty>] => false,
                        _ => panic!("Node {:?} Not an OptionTerminal{}", kind, stringify!($ast_type)),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! syntax_option {
    // Pattern with explicit empty variant: OptionType { SomeVariant: InnerType, EmptyVariant }
    (
        $ast_type:ident {
            $some_variant:ident: $inner_type:ty,
            $none_variant:ident
        }
    ) => {
        paste::paste! {
            impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_type<'db>> for Option<$inner_type> {
                fn from_ast(
                    ast: cairo_lang_syntax::node::ast::$ast_type<'db>,
                    db: &'db dyn salsa::Database
                ) -> Self {
                    match ast {
                        cairo_lang_syntax::node::ast::$ast_type::[<$some_variant>](val) =>
                            Some(val.ast_into(db)),
                        cairo_lang_syntax::node::ast::$ast_type::[<$none_variant>](_) =>
                            None,
                    }
                }
            }
        }
    };

    // Pattern without empty variant - defaults to "Empty"
    (
        $ast_type:ident {
            $some_variant:ident: $inner_type:ty
        }
    ) => {
        paste::paste! {
            impl<'db> $crate::FromAst<'db, cairo_lang_syntax::node::ast::$ast_type<'db>> for Option<$inner_type> {
                fn from_ast(
                    ast: cairo_lang_syntax::node::ast::$ast_type<'db>,
                    db: &'db dyn salsa::Database
                ) -> Self {
                    use crate::AstInto;
                    match ast {
                        cairo_lang_syntax::node::ast::$ast_type::[<$some_variant>](val) =>
                            Some(val.ast_into(db)),
                        cairo_lang_syntax::node::ast::$ast_type::Empty(_) =>
                            None,
                    }
                }
            }
        }
    };
}
