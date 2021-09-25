use std::collections::HashSet;

use syn::{Expr, Type, punctuated::Punctuated};



pub fn tuple_type(types: &[Type]) -> Type {
   let res = if types.len() == 0 {
      quote! {()}
   } else if types.len() == 1 {
      let ty = &types[0];
      quote! { ( #ty, ) }
   } else {
      quote! { ( #(#types),* ) }
   };
   syn::parse2(res).unwrap()
}

pub fn tuple(exprs: &[Expr]) -> Expr {
   let res = if exprs.len() == 0 {
      quote! {()}
   } else if exprs.len() == 1 {
      let ty = &exprs[0];
      quote! { ( #ty, ) }
   } else {
      quote! { ( #(#exprs),* ) }
   };
   syn::parse2(res).unwrap()
}

pub fn exp_cloned(exp: &Expr) -> Expr {
   let res = match exp {
      Expr::Path(_) |
      Expr::Field(_) |
      Expr::Paren(_) =>
         quote! {#exp.clone()},
      _ => quote! {(#exp).clone()}
   };
   syn::parse2(res).unwrap()
}

pub fn collect_set<T: Eq + std::hash::Hash>(iter : impl Iterator<Item = T>) -> HashSet<T> {
   iter.collect()
}

pub fn into_set<T: Eq + std::hash::Hash>(iter : impl IntoIterator<Item = T>) -> HashSet<T> {
   iter.into_iter().collect()
}

pub fn map_punctuated<T, P, U>(punc: Punctuated<T,P>, mut f: impl FnMut (T) -> U) -> Punctuated<U,P> {
   let mut res = Punctuated::new();
   for pair in punc.into_pairs() {
      let (t, p) = pair.into_tuple();
      res.push_value(f(t));
      if let Some(p) = p {res.push_punct(p)}
   };
   res
}