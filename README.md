# quote_precise
A version of quote that preserves Span locations

```rust
//junk Spans
let t: Token![!] = parse_quote! { ! };
println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());

//useful Spans
let t: Token![!] = parse_quote_precise! { ! };
println!("{:?}:{:?}-{:?}", t.span, t.span.start(), t.span.end());
assert_ne!(
   t.span.start(),
   t.span.end()
)
```

Waiting on [proc_macro::Span](https://doc.rust-lang.org/beta/proc_macro/struct.Span.html) to stabilize a bit
more to see if further work on this crate is not entirely redundant effort.
