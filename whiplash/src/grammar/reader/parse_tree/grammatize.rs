// Converts a generated parse tree into a grammar
use super::{Tree, Node, NodeType};
use crate::grammar::production::{Atoms, Rule, Atom};
use crate::grammar::Symbol;

impl Tree {
    pub fn get_rules(self, start_symbol: Symbol) -> Vec<Rule> {
        let mut out = vec![];
        let rhs_list = Self::to_rhs_list(self.root);

        for rhs in rhs_list.into_iter() {
            out.push(
                Rule::from(start_symbol.clone(), rhs)
            );
        }

        out
    }

    fn to_rhs_list(current_node: Box<Node>) -> Vec<Atoms> {
        let mut out = vec![];
        for child in current_node.children.into_iter() {
            let is_separator = child.atoms == Atoms::from_single_token("|".to_string());

            if let NodeType::X = child.nodetype {
                out = Self::permute_concat(out, Self::to_rhs_list(child));
            } else if is_separator {
                continue;
            } else {
                out = Self::permute_concat(out, vec![child.atoms]);
            }
        }

        out
    }

    fn permute_concat(accumulated: Vec<Atoms>, adjuncts: Vec<Atoms>) -> Vec<Atoms> {
        let mut result: Vec<Atoms> = vec![];

        for prependee in &accumulated {
            for adjunct in &adjuncts {
                // Check if adjunct or prependee are epsilon
                let is_adjunct_epsilon = *adjunct == Atoms::from_single_atom(Atom::Epsilon);
                let is_prependee_epsilon = *prependee == Atoms::from_single_atom(Atom::Epsilon);

                if !is_adjunct_epsilon && !is_prependee_epsilon {
                    result.push(Atoms::from(
                        [&prependee.vals[..], &adjunct.vals[..]].concat()
                    ));
                    
                } else if !is_prependee_epsilon {
                    result.push(Atoms::from(prependee.vals.clone()));
                    result.push(Atoms::from(vec![]));

                } else if !is_adjunct_epsilon {
                    result.push(Atoms::from(adjunct.vals.clone()));
                    result.push(Atoms::from(vec![]));

                } else {
                    result.push(Atoms::from(vec![]));
                }
            }
        }
        
        result
    }
}