pub enum Token {
    // Ops
    MOV,
    MOVSX,
    MOVZX,
    MOVSB,
    MOVSW,
    MOVSD,
    MOVAPS
    PUSH,
    POP,
    JMP,
    JE,
    JNE,
    JG,
    JL,
    JGE,
    JLE,
    JZ,
    JNZ,
    XCHG,
    LEA,
    CVTSI2SD,
    AND,
    OR,
    XOR,
    NOT,
    ADD,
    SUB,
    MUL,
    IMUL,
    DIV,
    IDIV,
    INC,
    DEC,
    NEG,
    CMP,
    CMPSB,
    CMPSW,
    CMPSD,
    SCASB,
    SCASW,
    SCASD,
    CALL,
    SYSCALL,
    RET,
    REP,
    FLD,
    FSTP,
    FADD,
    FSUB,
    FMUL,
    FDIV,
    SHL,
    SHR,
    SAL,
    SAR,
    ROL,
    ROR,

    // Registers
    RAX,
    RBX,
    RCX,
    RDX,
    RDI,
    RSI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    EAX,
    RBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    SP,
    BP,
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
    DH,
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,
    CR0,
    CR1,
    CR2,
    CR3,
    CR4,
    RFLAGS,
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
    YMM0,
    YMM1,
    YMM2,
    YMM3,
    YMM4,
    YMM5,
    YMM6,
    YMM7,
    YMM8,
    YMM9,
    YMM10,
    YMM11,
    YMM12,
    YMM13,
    YMM14,
    YMM15,
    ZMM0,
    ZMM1,
    ZMM2,
    ZMM3,
    ZMM4,
    ZMM5,
    ZMM6,
    ZMM7,
    ZMM8,
    ZMM9,
    ZMM10,
    ZMM11,
    ZMM12,
    ZMM13,
    ZMM14,
    ZMM15,
    RIP,

    // Types
    INTEGER(i64),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),

    // Label
    LABEL(String),

    // Additions
    COMMA,
    PARENTHESIS_OPEN,
    PARENTHESIS_CLOSE,
    BRACKET_OPEN,
    BRACKET_CLOSE,
    WHITESPACE,
    END,
}
