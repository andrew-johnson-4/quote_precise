use proc_macro::{TokenStream};
use proc_macro2::{TokenTree};
use quote::{quote,ToTokens};

#[proc_macro]
pub fn print_precise(input: TokenStream) -> TokenStream {
    println!("print precise");

    for t in input.into_iter() {
       println!("{:?}: {:?} p2({:?})", t, t.span(), proc_macro2::Span::from(t.span()));
    }

    TokenStream::from(quote! {})
}

fn copy_paste(ts: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
   let mut os = proc_macro2::TokenStream::new();

   for t in ts.into_iter() {
      match t {
         TokenTree::Group(_g) => {
            quote!(()).to_tokens(&mut os); 
         },
         TokenTree::Ident(_i) => {
            quote!(a).to_tokens(&mut os); 
         },
         TokenTree::Punct(p) => {
            quote!(
               let p = proc_macro2::TokenTree::Punct(
                  proc_macro2::Punct::new('!', proc_macro2::Spacing::Joint)
               );
               //TODO set Span of p
               s.extend([p].iter().cloned());
            ).to_tokens(&mut os); 
         }
         TokenTree::Literal(_l) => {
            quote!().to_tokens(&mut os); 
         }
      }
   }

   os
}

#[proc_macro]
pub fn quote_precise(input: TokenStream) -> TokenStream {
   let input = proc_macro2::TokenStream::from(input);

   let ts = copy_paste(input);

   let output = quote! {
      {
         let mut s = proc_macro2::TokenStream::new();
         #ts
         s
      }
   };

   TokenStream::from(output)
}
