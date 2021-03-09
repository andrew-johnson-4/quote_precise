use quote::quote;

#[test]
fn simple1() {
   println!("junk spans");
   let ts = quote!{ ! ! };
   for t in ts.into_iter() {
      println!("{:?}: {:?}", t, t.span());
   }

   println!("useful spans");
   quote_precise::print_precise!( ! ! );
}
