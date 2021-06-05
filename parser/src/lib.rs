use std::rc::Rc;
use std::str::FromStr;
use std::convert::From;

use peg::error::ParseError;

use crate::ast::{AnyDefinition, Ident, ScalarType, ContainerType, VariableType, TypeDeclaration};
use crate::ast::{Annotation, Visibility, ClassQualifier, FuncQualifier, ParamQualifier, FieldQualifier};
use crate::ast::{CommentDefinition, EnumDefinition, EnumValDefinition, ClassDefinition};
use crate::ast::{MemberDefinition, FuncDefinition, ParamDefinition, FieldDefinition, FileDefinition};

pub mod ast;

fn VecToStringTrimmed(v: Vec<&str>) -> String {
    String::from(v.concat().trim_end())
}

peg::parser! {
    grammar redscript() for str {
        use peg::ParseLiteral;

        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        // Any amount of whitespace
        rule _() = quiet!{ ([' ' | '\n' | '\r' | '\t'])* }
        rule space_sep<T>(r: rule<T>) -> Vec<T> = v:(r() ** _)          { v }
        rule comma_sep<T>(r: rule<T>) -> Vec<T> = v:(r() ** (_ "," _))  { v }
        rule dot_sep<T>(r: rule<T>) -> Vec<T> = v:(r() ** (_ "." _))    { v }
        // Line end
        rule endl() = "\n"

        // Documentation comments
        rule block_comment_start() = "/**"
        rule block_comment_end() = "*/"
        // For each line in a comment block, if it starts with:
        //  - Any amount of whitespace,
        //  - A single asterisk,
        //  - At least one space character
        // Those characters will be stripped from the comment before being output.
        rule block_comment_line() -> Rc<String>
            = (_ "* ")? s:$(!block_comment_end() !endl() [_])*
            { Rc::new(VecToStringTrimmed(s)) }

        rule block_comment() -> CommentDefinition
            = block_comment_start() _ lines:(block_comment_line() ** endl()) _ block_comment_end()
            { CommentDefinition{ lines } }

        // For each single-line comment, if there is one or more space characters between the three
        // slashes `///` and any number of non-space characters, the first of those whitespace
        // characters will be trimmed from the output.
        rule line_comment() -> Rc<String>
            = _ "///" [' ']? s:$(!endl() [_])* endl() { Rc::new(VecToStringTrimmed(s)) }

        pub rule comments() -> CommentDefinition
            = block:block_comment() { block }
            / lines:line_comment()+ { CommentDefinition{ lines } }



        // Recursive collection of curly-braced scope blocks to be ignored (ie. function body)
        rule scope_begin()   = "{"
        rule scope_content() = quiet!{ (!['}'] [_])* }
        rule scope_end()     = "}"
        rule scope_block()
            = scope_begin() scope_content() scope_block()* scope_content() scope_end()

        // parses a keyword string but makes sure it's not part of an identifier
        rule keyword(id: &'static str) -> () =
            ##parse_string_literal(id) !['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']

        // redscript uses C-style identifiers
        rule ident() -> Ident
            = x:$(['a'..='z' | 'A'..='Z' | '_']) xs:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '_']*)
            { Ident::new(format!("{}{}", x, xs)) }
            / expected!("Valid identifier")
       

        rule anno_type() -> Ident
            = "(" _ c:ident() _ ")" { c }
        rule annotation() -> Annotation
            = keyword("@replaceGlobal") _ "(" _ ")"     { Annotation::ReplaceGlobal }
            / keyword("@replaceMethod") _ t:anno_type() { Annotation::ReplaceMethod(t) }
            / keyword("@addMethod") _ t:anno_type()     { Annotation::AddMethod(t) }
            / keyword("@addField") _ t:anno_type()      { Annotation::AddField(t) }
            / "@" id:ident() _ "(" params:$(![')'] [_])* ")"
                { Annotation::Unsupported(id, Rc::new(params.concat())) }

        
        rule visibility() -> Visibility
            = keyword("public")     { Visibility::Public }
            / keyword("protected")  { Visibility::Protected }
            / keyword("private")    { Visibility::Private }
        
        rule class_qualifier() -> ClassQualifier
            = keyword("abstract")   { ClassQualifier::Abstract }
            / keyword("struct")     { ClassQualifier::Struct }
            / keyword("native")     { ClassQualifier::Native }

        rule func_qualifier() -> FuncQualifier
            = keyword("abstract")   { FuncQualifier::Abstract }
            / keyword("cb")         { FuncQualifier::Callback }
            / keyword("const")      { FuncQualifier::Const }
            / keyword("exec")       { FuncQualifier::Exec }
            / keyword("final")      { FuncQualifier::Final }
            / keyword("native")     { FuncQualifier::Native }
            / keyword("static")     { FuncQualifier::Static }

        rule param_qualifier() -> ParamQualifier
            = keyword("out")        { ParamQualifier::Out }
            / keyword("opt")        { ParamQualifier::Optional }

        rule field_qualifier() -> FieldQualifier
            = keyword("const")      { FieldQualifier::Const }
            / keyword("edit")       { FieldQualifier::Edit }
            / keyword("final")      { FieldQualifier::Final }
            / keyword("native")     { FieldQualifier::Native }
            / keyword("persistent") { FieldQualifier::Native }
            / keyword("static")     { FieldQualifier::Static }


        pub rule scalar_type() -> ScalarType
            = keyword("Void")       { ScalarType::Void }
            / keyword("Variant")    { ScalarType::Variant }
            / keyword("Bool")       { ScalarType::Bool }
            / keyword("Int32")      { ScalarType::I32 }
            / keyword("Int64")      { ScalarType::I64 }
            / keyword("Uint32")     { ScalarType::U32 }
            / keyword("Uint64")     { ScalarType::U64 }
            / keyword("Float")      { ScalarType::F32 }
            / keyword("Double")     { ScalarType::F64 }
            / keyword("String")     { ScalarType::String }
            / keyword("CName")      { ScalarType::CName }
            / keyword("ResRef")     { ScalarType::Resource }
            / keyword("TweakDBID")  { ScalarType::TweakDBID }


        pub rule type_args() -> TypeDeclaration
            = "<" _ t:type_() _ ">" { t }

        pub rule type_() -> TypeDeclaration
            = scal:scalar_type()
                { TypeDeclaration::leaf(VariableType::Scalar(scal)) }
            / comp:ident()
                { TypeDeclaration::leaf(VariableType::Compound(comp)) }
            / keyword("array") targ:type_args()
                { TypeDeclaration::node(VariableType::Container(ContainerType::Array), targ) }
            / keyword("ref") targ:type_args()
                { TypeDeclaration::node(VariableType::Container(ContainerType::Ref), targ) }
            / keyword("wref") targ:type_args()
                { TypeDeclaration::node(VariableType::Container(ContainerType::WeakRef), targ) }
            / keyword("script_ref") targ:type_args()
                { TypeDeclaration::node(VariableType::Container(ContainerType::ScriptRef), targ) }
        

        rule field_type() -> TypeDeclaration 
            = ":" _ t:type_() { t }
        rule func_type() -> TypeDeclaration 
            = "->" _ t:type_() { t }

        
        pub rule field() -> FieldDefinition
            = comments:comments()?
            _ annotations:space_sep(<annotation()>)
            _ visibility:visibility()
            _ qualifiers:space_sep(<field_qualifier()>)
            _ keyword("let")
            _ name:ident()
            _ type_:field_type()
            _ ";"
            { FieldDefinition { comments, annotations, visibility, qualifiers, name, type_ } }

        pub rule param() -> ParamDefinition
            = qualifiers:space_sep(<param_qualifier()>)
            _ name:ident()
            _ type_:field_type()
            { ParamDefinition { qualifiers, name, type_ } }

        pub rule func() -> FuncDefinition
            = comments:comments()?
            _ annotations:space_sep(<annotation()>)
            _ visibility:visibility()
            _ qualifiers:space_sep(<func_qualifier()>)
            _ keyword("func")
            _ name:ident()
            _ "(" _ params:comma_sep(<param()>) _ ")"
            _ returns:func_type()
            { FuncDefinition{ comments, annotations, visibility, qualifiers, name, params, returns } }

        pub rule field_traced() -> FieldDefinition = traced(<field()>)
        pub rule type_traced() -> TypeDeclaration = traced(<type_()>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_comment_line() {
        let comment = redscript::comments("/// Line comment\n").unwrap();

        assert_eq!(
            format!("{:?}", comment.lines),
            format!("{:?}", vec!["Line comment"])
        )
    }

    #[test]
    fn parse_comment_block() {
        let comments = redscript::comments("/**
Unprefixed line
 * Prefixed line
* Also prefixed line

Empty line above
    Important leading whitespace
Trailing whitespace    
No whitespace at end*/").unwrap();

        assert_eq!(
            format!("{:?}", comments.lines),
            format!("{:?}", vec![
                "Unprefixed line",
                "Prefixed line",
                "Also prefixed line",
                "",
                "Empty line above",
                "    Important leading whitespace",
                "Trailing whitespace",
                "No whitespace at end"
            ])
        )
    }


    #[test]
    fn parse_field_uncommented() {
        let field = redscript::field_traced("private static const let m_field: Int32;").unwrap();

        assert_eq!(
            format!("{:?}", field),
            format!("{:?}", FieldDefinition {
                comments: None,
                annotations: vec![],
                visibility: Visibility::Private,
                qualifiers: vec![FieldQualifier::Static, FieldQualifier::Const],
                name: Ident::new("m_field".to_string()),
                type_: TypeDeclaration::leaf(VariableType::Scalar(ScalarType::I32))
            })
        );
    }

    #[test]
    fn parse_type_args() {
        let scalar = redscript::type_traced("array<CName> butts").unwrap();
        assert_eq!(
            format!("{:?}", scalar),
            format!("{:?}", TypeDeclaration::leaf(VariableType::Scalar(ScalarType::CName)))
        )
    }

    #[test]
    fn parse_field_commented() {
        let field = redscript::field_traced(
            r#"/**
              * This field has several comments
              * with a bunch of whitespace
              * and **MARKDOWN** content
              *   - Like this list item
              */
            @addField(GameObject)
            protected native final let m_names: array<CName>;"#).unwrap();

        assert_eq!(
            format!("{:?}", field),
            format!("{:?}", FieldDefinition {
                comments: Some(CommentDefinition {
                    lines: vec![
                        Rc::new("This field has several comments".to_string()),
                        Rc::new("with a bunch of whitespace".to_string()),
                        Rc::new("and **MARKDOWN** content".to_string()),
                        Rc::new("  - Like this list item".to_string())
                    ]
                }),
                annotations: vec![Annotation::AddField(Ident::new("GameObject".to_string()))],
                visibility: Visibility::Protected,
                qualifiers: vec![FieldQualifier::Native, FieldQualifier::Final],
                name: Ident::new("m_names".to_string()),
                type_: TypeDeclaration::node(
                    VariableType::Container(ContainerType::Array),
                    TypeDeclaration::leaf(VariableType::Scalar(ScalarType::CName))
                )
            })
        );
    }
}
