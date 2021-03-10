use quote_precise::quote_precise;

#[test]
fn simple1() {
   let ts = quote_precise!{ ! ! };
   for t in ts.into_iter() {
      assert_ne!(t.span().start(), t.span().end());
   }
}
