extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, ItemFn, Ident, Token, Expr, Pat, Local, parse_quote, Stmt, Block};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace};
use syn::punctuated::Punctuated;
use std::collections::HashSet;
use syn::Result;
use syn::fold;
use syn::fold::Fold;
use quote::ToTokens;


pub struct MatrixArgs{
    vars:HashSet<Ident>
}

impl Parse for MatrixArgs{
    fn parse(input: ParseStream) -> Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(MatrixArgs {
            vars: vars.into_iter().collect(),
        })
    }
}

impl MatrixArgs {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
 // variable shouldn't start wiht ::
                if e.path.leading_colon.is_some() {
                    false
// should be a single variable like `x=8` not n::x=0 
                } else if e.path.segments.len() != 1 {
                    false
                } else {
// get the first part
                    let first = e.path.segments.first().unwrap();
// check if the variable name is in the Args.vars hashset
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

// used for checking if to print let i=0 etc or not
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
// check if variable name is present in set
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

// manipulate tree to insert print statement
    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
 // recurive call on right of the assigment statement
        let right = fold::fold_expr(self, right);
// returning manipulated sub-tree
        parse_quote!({
            #left #op #right;
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        })
    }

// manipulating let statement
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
// get the variable name of assigned variable
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
// new sub tree
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}

impl Fold for MatrixArgs {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
// for changing assignment like a=5
            Expr::Assign(e) => {
// check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
// continue with default travesal using default methods
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
// for changing assigment and operation like a+=1
            Expr::AssignOp(e) => {
// check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
// continue with default behaviour
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
// continue with default behaviour for rest of expressions
            _ => fold::fold_expr(self, e),
        }
    }

// for let statements like let d=9
    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }   

    fn fold_block(&mut self, node: Block) -> Block
    {
        let count = node.stmts.len();
        //println!("Entering block with {} statements", node.stmts.len());
        let mut stmts: Vec<Stmt>;
        for val in node.stmts.iter() {
            println!("Got: {}", stmt_to_str(val));
        }
        stmts = node.stmts.iter().map(|s| fold::fold_stmt(self, s.clone())).collect();
        let begin:Stmt = parse_quote!{
            println!("Entering block with {} statements", #count);
        };
        stmts.insert(0, begin);
        let end:Stmt = parse_quote!{
            println!("Leaving block with {} statements", #count);
        };
        stmts.push(end);
        Block {
            brace_token: node.brace_token,
            stmts
        }
    }
}

fn stmt_to_str(s: &Stmt) -> &str {
    match s {
        Stmt::Local(_) => "Local",
        Stmt::Item(_) => "Item",
        Stmt::Expr(_) => "Expr",
        Stmt::Semi(_, _) => "Semi",
    }
}

