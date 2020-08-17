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

struct AST {
    root: Node,
}

impl AST {
    pub fn parse_line(&self, line: &str) {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.parse_line(line);
    }

}