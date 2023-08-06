use pest::Parser;


#[derive(Parser)]
#[grammar = "bff_asm.pest"]
struct BffAsmBareParser;