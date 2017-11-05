extern crate syn;
use syn::*;

#[test]
fn test_unit() {
    let raw = "struct Unit;";

    let expected = MacroInput {
        ident: "Unit".into(),
        vis: Visibility::Inherited,
        attrs: Vec::new(),
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    assert_eq!(expected, parse_macro_input(raw).unwrap());
}

#[test]
fn test_struct() {
    let raw = "
        #[derive(Debug, Clone)]
        pub struct Item {
            pub ident: Ident,
            pub attrs: Vec<Attribute>,
        }
    ";

    let expected = MacroInput {
        ident: "Item".into(),
        vis: Visibility::Public,
        attrs: vec![Attribute {
                        style: AttrStyle::Outer,
                        value: MetaItem::List("derive".into(),
                                              vec![
                    NestedMetaItem::MetaItem(MetaItem::Word("Debug".into())),
                    NestedMetaItem::MetaItem(MetaItem::Word("Clone".into())),
                ]),
                        is_sugared_doc: false,
                    }],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Struct(vec![Field {
                                                        ident: Some("ident".into()),
                                                        vis: Visibility::Public,
                                                        attrs: Vec::new(),
                                                        ty: Ty::Path(None, "Ident".into()),
                                                    },
                                                    Field {
                                                        ident: Some("attrs".into()),
                                                        vis: Visibility::Public,
                                                        attrs: Vec::new(),
                                                        ty: Ty::Path(None,
                                                                     Path {
                                                                         global: false,
                                                                         segments: vec![
                        PathSegment {
                            ident: "Vec".into(),
                            parameters: PathParameters::AngleBracketed(
                                AngleBracketedParameterData {
                                    lifetimes: Vec::new(),
                                    types: vec![Ty::Path(None, "Attribute".into())],
                                    bindings: Vec::new(),
                                },
                            ),
                        }
                    ],
                                                                     }),
                                                    }])),
    };

    assert_eq!(expected, parse_macro_input(raw).unwrap());
}

#[test]
#[cfg(feature = "full")]
fn test_enum() {
    let raw = r#"
        /// See the std::result module documentation for details.
        #[must_use]
        pub enum Result<T, E> {
            Ok(T),
            Err(E),
            Surprise = 0isize,

            // Smuggling data into a proc_macro_derive,
            // in the style of https://github.com/dtolnay/proc-macro-hack
            ProcMacroHack = (0, "data").0,
        }
    "#;

    let expected = MacroInput {
        ident: "Result".into(),
        vis: Visibility::Public,
        attrs: vec![
            Attribute {
                style: AttrStyle::Outer,
                value: MetaItem::NameValue(
                    "doc".into(),
                    Lit::Str(
                        "/// See the std::result module documentation for details.".into(),
                        StrStyle::Cooked,
                    ),
                ),
                is_sugared_doc: true,
            },
            Attribute {
                style: AttrStyle::Outer,
                value: MetaItem::Word("must_use".into()),
                is_sugared_doc: false,
            },
        ],
        generics: Generics {
            lifetimes: Vec::new(),
            ty_params: vec![TyParam {
                                attrs: Vec::new(),
                                ident: "T".into(),
                                bounds: Vec::new(),
                                default: None,
                            },
                            TyParam {
                                attrs: Vec::new(),
                                ident: "E".into(),
                                bounds: Vec::new(),
                                default: None,
                            }],
            where_clause: WhereClause { predicates: Vec::new() },
        },
        body: Body::Enum(vec![
            Variant {
                ident: "Ok".into(),
                attrs: Vec::new(),
                data: VariantData::Tuple(vec![
                    Field {
                        ident: None,
                        vis: Visibility::Inherited,
                        attrs: Vec::new(),
                        ty: Ty::Path(None, "T".into()),
                    },
                ]),
                discriminant: None,
            },
            Variant {
                ident: "Err".into(),
                attrs: Vec::new(),
                data: VariantData::Tuple(vec![
                    Field {
                        ident: None,
                        vis: Visibility::Inherited,
                        attrs: Vec::new(),
                        ty: Ty::Path(None, "E".into()),
                    },
                ]),
                discriminant: None,
            },
            Variant {
                ident: "Surprise".into(),
                attrs: Vec::new(),
                data: VariantData::Unit,
                discriminant: Some(ConstExpr::Lit(Lit::Int(0, IntTy::Isize))),
            },
            Variant {
                ident: "ProcMacroHack".into(),
                attrs: Vec::new(),
                data: VariantData::Unit,
                discriminant: Some(ConstExpr::Other(Expr {
                    node: ExprKind::TupField(
                        Box::new(Expr {
                            node: ExprKind::Tup(vec![
                                Expr {
                                    node: ExprKind::Lit(Lit::Int(0, IntTy::Unsuffixed)),
                                    attrs: Vec::new(),
                                },
                                Expr {
                                    node: ExprKind::Lit(Lit::Str("data".into(), StrStyle::Cooked)),
                                    attrs: Vec::new(),
                                },
                            ]),
                            attrs: Vec::new(),
                        }),
                        0
                    ),
                    attrs: Vec::new(),
                })),
            },
        ]),
    };

    assert_eq!(expected, parse_macro_input(raw).unwrap());
}

#[test]
fn test_attr_with_path() {
    let raw =r#"
        #[::attr_args::identity
            fn main() { assert_eq!(foo(), "Hello, world!"); }]
        struct Dummy;
    "#;

    let expected = MacroInput {
        ident: "Dummy".into(),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            style: AttrStyle::Outer,
            path: Path { global: true, segments: vec!["attr_args".into(), "identity".into()] },
            tts: vec![
                TokenTree::Token(Token::Ident("fn".into())),
                TokenTree::Token(Token::Ident("main".into())),
                TokenTree::Delimited(Delimited { delim: DelimToken::Paren, tts: vec![] }),
                TokenTree::Delimited(Delimited { delim: DelimToken::Brace, tts: vec![
                    TokenTree::Token(Token::Ident("assert_eq".into())),
                    TokenTree::Token(Token::Not),
                    TokenTree::Delimited(Delimited { delim: DelimToken::Paren, tts: vec![
                        TokenTree::Token(Token::Ident("foo".into())),
                        TokenTree::Delimited(Delimited { delim: DelimToken::Paren, tts: vec![] }),
                        TokenTree::Token(Token::Comma),
                        TokenTree::Token(Token::Literal(Lit::Str("Hello, world!".into(), StrStyle::Cooked))),
                    ]}),
                    TokenTree::Token(Token::Semi),
                ]})
            ],
            is_sugared_doc: false,
        }],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);

    assert!(actual.attrs[0].meta_item().is_none());
}

#[test]
fn test_attr_with_non_mod_style_path() {
    let raw =r#"
        #[inert <T>]
        struct S;
    "#;

    let expected = MacroInput {
        ident: "S".into(),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            style: AttrStyle::Outer,
            path: Path { global: false, segments: vec!["inert".into()] },
            tts: vec![
                TokenTree::Token(Token::Lt),
                TokenTree::Token(Token::Ident("T".into())),
                TokenTree::Token(Token::Gt),
            ],
            is_sugared_doc: false,
        }],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);

    assert!(actual.attrs[0].meta_item().is_none());
}

#[test]
fn test_attr_with_mod_style_path_with_self() {
    let raw =r#"
        #[foo::self]
        struct S;
    "#;

    let expected = MacroInput {
        ident: "S".into(),
        vis: Visibility::Inherited,
        attrs: vec![Attribute {
            style: AttrStyle::Outer,
            path: Path { global: false, segments: vec!["foo".into(), "self".into()] },
            tts: vec![],
            is_sugared_doc: false,
        }],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);

    assert!(actual.attrs[0].meta_item().is_none());
}

#[test]
fn test_pub_restricted() {
    // Taken from tests/rust/src/test/ui/resolve/auxiliary/privacy-struct-ctor.rs
    let raw =r#"
        pub(in m) struct Z(pub(in m::n) u8);
    "#;

    let expected = MacroInput {
        ident: "Z".into(),
        vis: Visibility::Restricted(Box::new("m".into())),
        attrs: vec![],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Tuple(vec![Field {
            ident: None,
            vis: Visibility::Restricted(Box::new(Path { global: false, segments: vec!["m".into(), "n".into()] })),
            attrs: vec![],
            ty: Ty::Path(None, "u8".into()),
        }])),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_pub_restricted_crate() {
    let raw =r#"
        pub(crate) struct S;
    "#;

    let expected = MacroInput {
        ident: "S".into(),
        vis: Visibility::Crate,
        attrs: vec![],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_pub_restricted_super() {
    let raw =r#"
        pub(super) struct S;
    "#;

    let expected = MacroInput {
        ident: "S".into(),
        vis: Visibility::Restricted(Box::new("super".into())),
        attrs: vec![],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn test_pub_restricted_in_super() {
    let raw =r#"
        pub(in super) struct S;
    "#;

    let expected = MacroInput {
        ident: "S".into(),
        vis: Visibility::Restricted(Box::new("super".into())),
        attrs: vec![],
        generics: Generics::default(),
        body: Body::Struct(VariantData::Unit),
    };

    let actual = parse_macro_input(raw).unwrap();

    assert_eq!(expected, actual);
}
