/*
title:とほほのRust入門
https://www.tohoho-web.com/ex/rust.html#crate

title:Rust入門
https://zenn.dev/mebiusbox/books/22d4c1ed9b0003
*/

mod _03_hw;
mod _04_type;
mod _05_fn;
mod _06_control;
mod _07_match;
mod _11_closure;
mod _12_struct;
mod _13_enum;
mod _14_generic;
mod _15_result;
mod _16_option;
mod _17_impl;
mod _18_trait;
mod _19_iterator;
mod _20_thread;
mod _21_async;
mod _23_mod;
mod _24_lib;
mod _25_ref;
mod _26_ownership;
mod _27_string_borrow;
mod _28_lifetime;
mod _40_concurrency;

pub static r#REF: &str = r#"
title:とほほのRust入門
https://www.tohoho-web.com/ex/rust.html#crate

title:Rust入門
https://zenn.dev/mebiusbox/books/22d4c1ed9b0003
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "03 hw",                       func: _03_hw::main                        },
 Func{ name: "04 type",                     func: _04_type::main                      },
 Func{ name: "05 fn",                       func: _05_fn::main                        },
 Func{ name: "06 control",                  func: _06_control::main                   },
 Func{ name: "07 match",                    func: _07_match::main                     },
 Func{ name: "11 closure",                  func: _11_closure::main                   },
 Func{ name: "12 struct",                   func: _12_struct::main                    },
 Func{ name: "13 enum",                     func: _13_enum::main                      },
 Func{ name: "14 generic",                  func: _14_generic::main                   },
 Func{ name: "15 result",                   func: _15_result::main                    },
 Func{ name: "16 option",                   func: _16_option::main                    },
 Func{ name: "17 impl",                     func: _17_impl::main                      },
 Func{ name: "18 trait",                    func: _18_trait::main                     },
 Func{ name: "19 iterator",                 func: _19_iterator::main                  },
 Func{ name: "20 thread",                   func: _20_thread::main                    },
 Func{ name: "21 async",                    func: _21_async::main                     },
 Func{ name: "23 mod",                      func: _23_mod::main                       },
 Func{ name: "24 lib",                      func: _24_lib::main                       },
 Func{ name: "25 ref",                      func: _25_ref::main                       },
 Func{ name: "26 ownership",                func: _26_ownership::main                 },
 Func{ name: "27 string_borrow",            func: _27_string_borrow::main             },
 Func{ name: "28 lifetime",                 func: _28_lifetime::main                  },
 Func{ name: "40 concurrency",              func: _40_concurrency::main               },

];
