use crate::parser::node::node::Node;
use crate::lexical_analyser::tokenizer::tokenizer::Tokenizer;
// use crate::lexical_analyser::token::token::Token;

/// The AST struct implements the following grammar, which is a subset of the grammar found here: https://docs.python.org/3/reference/grammar.html
/// Grammar: 
/// The grammar starts at simple_stmt
/// simple_stmt: (expr_stmt | flow_stmt)
/// expr_stmt: testlist_star_expr (annassign | augassign (yield_expr|testlist) |
///             [('=' (yield_expr|testlist_star_expr))+ [TYPE_COMMENT]] )
/// annassign: ':' test ['=' (yield_expr|testlist_star_expr)]
/// testlist_star_expr: (test|star_expr) (',' (test|star_expr))* [',']
/// test: or_test ['if' or_test 'else' test] | lambdef
/// or_test: and_test ('or' and_test)*
/// and_test: not_test ('and' not_test)*
/// not_test: 'not' not_test | comparison
/// comparison: expr (comp_op expr)*
/// comp_op: '<'|'>'|'=='|'>='|'<='|'<>'|'!='|'in'|'not' 'in'|'is'|'is' 'not'
/// expr: xor_expr ('|' xor_expr)*
/// xor_expr: and_expr ('^' and_expr)*
/// and_expr: shift_expr ('&' shift_expr)*
/// shift_expr: arith_expr (('<<'|'>>') arith_expr)*
/// arith_expr: term (('+'|'-') term)*
/// term: factor (('*'|'@'|'/'|'%'|'//') factor)*
/// factor: ('+'|'-'|'~') factor | power
/// power: atom_expr ['**' factor]
/// atom_expr: [AWAIT] atom trailer*
/// atom: ('(' [yield_expr|testlist_comp] ')' |
///     '[' [testlist_comp] ']' |
///     '{' [dictorsetmaker] '}' |
///     NAME | NUMBER | STRING+ | '...' | 'None' | 'True' | 'False')
/// testlist_comp: (namedexpr_test|star_expr) ( comp_for | (',' (namedexpr_test|star_expr))* [','] )
/// trailer: '(' [arglist] ')' | '[' subscriptlist ']' | '.' NAME
/// subscriptlist: subscript (',' subscript)* [',']
/// subscript: test | [test] ':' [test] [sliceop]
/// sliceop: ':' [test]
/// exprlist: (expr|star_expr) (',' (expr|star_expr))* [',']
/// testlist: test (',' test)* [',']
/// dictorsetmaker: ( ((test ':' test | '**' expr)
///                    (comp_for | (',' (test ':' test | '**' expr))* [','])) |
///                   ((test | star_expr)
///                    (comp_for | (',' (test | star_expr))* [','])) )
/// lambdef: 'lambda' [varargslist] ':' test
/// testlist: test (',' test)* [',']
/// star_expr: '*' expr
/// augassign: ('+=' | '-=' | '*=' | '@=' | '/=' | '%=' | '&=' | '|=' | '^=' |
///             '<<=' | '>>=' | '**=' | '//=')
/// flow_stmt: break_stmt | continue_stmt | return_stmt | raise_stmt | yield_expr
/// break_stmt: 'break'
/// continue_stmt: 'continue'
/// return_stmt: 'return' [testlist_star_expr]
/// raise_stmt: 'raise' [test ['from' test]]
/// yield_expr: 'yield' [yield_arg]
/// yield_arg: 'from' test | testlist_star_expr
/// namedexpr_test: test [':=' test]
/// arglist: argument (',' argument)*  [',']
/// argument: ( test [comp_for] |
///     test ':=' test |
///     test '=' test |
///     '**' test |
///     '*' test )
/// comp_iter: comp_for | comp_if
/// sync_comp_for: 'for' exprlist 'in' or_test [comp_iter]
/// comp_for: [ASYNC] sync_comp_for
/// comp_if: 'if' test_nocond [comp_iter]
/// test_nocond: or_test | lambdef_nocond
/// lambdef_nocond: 'lambda' [varargslist] ':' test_nocond
/// varargslist: vfpdef ['=' test ](',' vfpdef ['=' test])* ',' '/' [',' [ (vfpdef ['=' test] (',' vfpdef ['=' test])* [',' [
///     '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
///     | '**' vfpdef [',']]]
///     | '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
///     | '**' vfpdef [',']) ]] | (vfpdef ['=' test] (',' vfpdef ['=' test])* [',' [
///     '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
///     | '**' vfpdef [',']]]
///     | '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
///     | '**' vfpdef [',']
///     )
/// vfpdef: NAME

struct AST {
    root: Node,
}

impl AST {
    pub fn parse_line(&self, line: &str) {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.parse_line(line);
    }

}

// namedexpr_test, arglist, comp_for 