use std::rc::Rc;

#[derive(Debug)]
pub enum AnyDefinition {
    Comments(CommentDefinition),
    Enum(EnumDefinition),
    EnumVal(EnumValDefinition),
    Class(ClassDefinition),
    Func(FuncDefinition),
    Param(ParamDefinition),
    Field(FieldDefinition),
    File(FileDefinition),
}

#[derive(Debug)]
pub enum Ident {
    Static(&'static str),
    Owned(Rc<String>),
}

impl Ident {
    pub fn new(str: String) -> Ident {
        Ident::Owned(Rc::new(str))
    }

    pub fn to_owned(&self) -> Rc<String> {
        match self {
            Ident::Static(str) => Rc::new(str.to_string()),
            Ident::Owned(rc) => rc.clone(),
        }
    }
}

#[derive(Debug)]
pub enum ScalarType {
    Void,
    Variant,
    Bool,
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    String,
    CName,
    Resource,
    TweakDBID,
}

#[derive(Debug)]
pub enum ContainerType {
    Array,
    Ref,
    WeakRef,
    ScriptRef,
}

#[derive(Debug)]
pub enum VariableType {
    Scalar(ScalarType),
    Compound(Ident),
    Container(ContainerType),
}

#[derive(Debug)]
pub struct TypeDeclaration {
    pub type_: VariableType,
    pub subtype: Option<Box<TypeDeclaration>>
}

impl TypeDeclaration {
    pub const fn leaf(type_: VariableType) -> Self {
        TypeDeclaration { type_, subtype: None }
    }

    pub fn node(type_: VariableType, subtype: TypeDeclaration) -> Self {
        TypeDeclaration { type_, subtype: Some(Box::new(subtype)) }
    }
}

#[derive(Debug)]
pub enum Annotation {
    ReplaceGlobal,
    ReplaceMethod(Ident),
    AddMethod(Ident),
    AddField(Ident),
    Unsupported(Ident, Rc<String>)
}

/// Object visibility shared by classes and members
#[derive(Debug)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug)]
pub enum ClassQualifier {
    Abstract,
    Struct,
    Native,
}

#[derive(Debug)]
pub enum FuncQualifier {
    Abstract,
    Callback,
    Const,
    Exec,
    Final,
    Native,
    Static,
}

#[derive(Debug)]
pub enum ParamQualifier {
    Out,
    Optional,
}

#[derive(Debug)]
pub enum FieldQualifier {
    Const,
    Edit,
    Final,
    Native,
    Persistent,
    Static,
}

#[derive(Debug)]
pub enum MemberDefinition {
    Field(FieldDefinition),
    Function(FuncDefinition),
}

//------------------------------------------------------------------------------
// Definitions

#[derive(Debug)]
pub struct CommentDefinition {
    pub lines: Vec<Rc<String>>,
}

#[derive(Debug)]
pub struct EnumDefinition {
    pub comments: Option<CommentDefinition>,
    pub name: Ident,
    pub values: Vec<EnumValDefinition>,
}

#[derive(Debug)]
pub struct EnumValDefinition {
    pub comments: Option<CommentDefinition>,
    pub name: Ident,
    pub value: i64,
}

#[derive(Debug)]
pub struct ClassDefinition {
    pub comments: Option<CommentDefinition>,
    pub visibility: Visibility,
    pub qualifiers: Vec<ClassQualifier>,
    pub name: Ident,
    pub base: Option<Ident>,
    pub members: Vec<MemberDefinition>,
}

#[derive(Debug)]
pub struct FuncDefinition {
    pub comments: Option<CommentDefinition>,
    pub annotations: Vec<Annotation>,
    pub visibility: Visibility,
    pub qualifiers: Vec<FuncQualifier>,
    pub name: Ident,
    pub params: Vec<ParamDefinition>,
    pub returns: TypeDeclaration,
}

#[derive(Debug)]
pub struct ParamDefinition {
    pub qualifiers: Vec<ParamQualifier>,
    pub name: Ident,
    pub type_: TypeDeclaration,
}

#[derive(Debug)]
pub struct FieldDefinition {
    pub comments: Option<CommentDefinition>,
    pub annotations: Vec<Annotation>,
    pub visibility: Visibility,
    pub qualifiers: Vec<FieldQualifier>,
    pub name: Ident,
    pub type_: TypeDeclaration,
}

#[derive(Debug)]
pub struct FileDefinition {
    pub name: Ident,
    pub defs: Vec<AnyDefinition>
}
