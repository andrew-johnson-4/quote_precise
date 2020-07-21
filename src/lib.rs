use quote::quote;

#[proc_macro]
pub fn quote_precise(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
   let _input = proc_macro2::TokenStream::from(input);

   let output = quote! {
      let mut s = proc_macro2::TokenStream::new();
      s
   };

   proc_macro::TokenStream::from(output)
}
