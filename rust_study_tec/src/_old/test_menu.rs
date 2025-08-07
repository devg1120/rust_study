/*
title:Rust ツアー
https://tourofrust.com/TOC_ja.html
*/

mod _10_generic;
mod _12_ownership_borrow;
mod _13_lifetime;
mod _20_oop;
mod _30_closure;
mod _40_adt_enum;
mod _50_async_tokio;

pub static r#REF: &str = r#"
title:Rust ツアー
https://tourofrust.com/TOC_ja.html
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "10 generic",                  func: _10_generic::main                   },
 Func{ name: "12 ownership_borrow",         func: _12_ownership_borrow::main          },
 Func{ name: "13 lifetime",                 func: _13_lifetime::main                  },
 Func{ name: "20 oop",                      func: _20_oop::main                       },
 Func{ name: "30 closure",                  func: _30_closure::main                   },
 Func{ name: "40 adt_enum",                 func: _40_adt_enum::main                  },
 Func{ name: "50 async_tokio",              func: _50_async_tokio::main               },

];
