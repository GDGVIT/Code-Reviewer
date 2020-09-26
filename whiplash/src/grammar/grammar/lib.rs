use crate::grammar::production::{Rule, Atom};
use std::collections::HashSet;
use std::fmt;
use std::ops::{Index, IndexMut};

/// A subset of the actual python grammar is implemented
/// Each of the symbols here have a corresponding value in the NodeType enum
/// 
/// Grammar: 
/// Start symbol: simple_stmt
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

pub struct Grammar {
    pub productions: Vec<Rule>
}

impl Grammar {
    pub fn from(productions: Vec<Rule>) -> Grammar {
        Grammar {
            productions
        }
    }

    pub fn get_all_atoms(&self) -> HashSet<Atom> {
        let mut out = HashSet::new();
        
        for rule in &self.productions {
            out.insert(Atom::from_symbol(&rule.start_symbol));

            for atom in rule.rhs.iter() {
                out.insert(atom.clone());
            }
        }

        out
    } 
}

impl fmt::Debug for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.productions.iter().fold(
                String::new(),
                |acc, production| acc + &format!("{:?}", &production)[..] + "\n"
            )
        )
    }
}

impl Index<usize> for Grammar {
    type Output = Rule;

    fn index(&self, i: usize) -> &Self::Output {
        &self.productions[i]
    }
}

impl IndexMut<usize> for Grammar {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.productions[i]
    }
}