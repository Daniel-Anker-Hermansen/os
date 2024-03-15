pub struct Module<'src> {
    pub items: Vec<Item<'src>>,
}

pub enum Item<'src> {
    Intrinsic(Intrinsic<'src>),
    Interupt(Interupt<'src>),
    Function(Function<'src>),
    Struct(Struct<'src>),
}

pub struct Intrinsic<'src> {
    pub name: &'src str,
    pub signature: Signature<'src>,
}

pub struct Function<'src> {
    pub name: &'src str,
    pub signature: Signature<'src>,
    pub body: Block<'src>,
}

pub struct Interupt<'src> {
    pub name: &'src str,
    pub body: Block<'src>,
}

pub struct Struct<'src> {
    pub name: &'src str,
    pub fields: Vec<Argument<'src>>,
}

pub struct Signature<'src> {
    pub arguments: Vec<Argument<'src>>,
    pub return_type: Type<'src>,
}

pub struct Argument<'src> {
    pub identifier: &'src str,
    pub r#type: Type<'src>,
}

pub struct Block<'src> {
    pub statements: Vec<Expression<'src>>,
    pub is_semicolon_terminated: bool,
}

enum Type<'src> {
    U8,
    U16,
    U32,
    U64,
    Usize,
    Pointer {
        inner: Box<Type<'src>>,
    },
    Custom {
        name: &'src str,
    },
}

enum Expression<'src> {
    Let {
        name: &'src str,
        right_hand_side: Box<Expression<'src>>,
    },
    InfixBinaryOperator {
        left_hand_side: Box<Expression<'src>>,
        operator: InfixBinaryOperator,
        right_hand_side: Box<Expression<'src>>,
    },
    PreUnaryOperator {
        operator: PreUnaryOperator,
        expression: Box<Expression<'src>>,
    },
    FunctionCall {
        function: &'src str,
        parameters: Vec<Expression<'src>>,
    },
    If {
        condition: Box<Expression<'src>>,
        then_body: Block<'src>,
        else_body: Block<'src>,
    },
    Loop {
        body: Block<'src>,
    },
    Break,
}

enum InfixBinaryOperator {
    Plus,
    Minus,
    Multiplication,
    Division,
    ShiftRight,
    ShiftLeft,
    Equal,
    Greater,
    Less,
    Arrow,
}

enum PreUnaryOperator {
    Not,
    Neg,
    Deref,
}
