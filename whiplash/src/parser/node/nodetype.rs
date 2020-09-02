use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, Clone, EnumIter)]
pub enum NodeType {
    simple_stmt,
    expr_stmt,
    annassign,
    testlist_star_expr,
    test,
    or_test,
    and_test,
    not_test,
    comparison,
    comp_op,
    expr,
    xor_expr,
    and_expr,
    shift_expr,
    arith_expr,
    term,
    factor,
    power,
    atom_expr,
    atom,
    testlist_comp,
    trailer,
    subscriptlist,
    subscript,
    sliceop,
    exprlist,
    testlist,
    dictorsetmaker,
    lambdef,
    star_expr,
    augassign,
    flow_stmt,
    break_stmt,
    continue_stmt,
    return_stmt,
    raise_stmt,
    yield_expr,
    yield_arg,
    namedexpr_test,
    arglist,
    argument,
    comp_iter,
    sync_comp_for,
    comp_for,
    comp_if,
    test_nocond,
    lambdef_nocond,
    varargslist,
    vfpdef,
}
