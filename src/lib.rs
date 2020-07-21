use quote::{quote,ToTokens};
use proc_macro2::{TokenStream,TokenTree};

fn copy_paste(ts: TokenStream) -> TokenStream {
   let mut os = TokenStream::new();

   for t in ts.into_iter() {
      match t {
         TokenTree::Group(_g) => {
            quote!().to_tokens(&mut os); 
         },
         TokenTree::Ident(_i) => {
            quote!().to_tokens(&mut os); 
         },
         TokenTree::Punct(p) => {
            quote!(
               let p = proc_macro2::Punct::new('!', Spacing::Joint);
               s.append(p);
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
pub fn quote_precise(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
   let input = TokenStream::from(input);

   let ts = copy_paste(input);

   let output = quote! {
      use quote::{ToTokens};
      let mut s = proc_macro2::TokenStream::new();
      #ts
      s
   };

   proc_macro::TokenStream::from(output)
}
