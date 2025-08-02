/*
title:お気楽 Rust プログラミング超入門
https://www.nct9.ne.jp/m_hiroi/linux/rust.html
*/

mod _10_ownership;
mod _11_trait;
mod _12_generic;
mod _13_box;
mod _14_trait_object;
mod _15_closure;
mod _16_lifetime;
mod _17_iterator;
mod _18_error_operation;
mod _19_fmt_display;
mod _20_rc;
mod _21_cell;
mod _30_thread_channel;

pub static r#REF: &str = r#"
title:お気楽 Rust プログラミング超入門
https://www.nct9.ne.jp/m_hiroi/linux/rust.html
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "10 ownership",                func: _10_ownership::main                 },
 Func{ name: "11 trait",                    func: _11_trait::main                     },
 Func{ name: "12 generic",                  func: _12_generic::main                   },
 Func{ name: "13 box",                      func: _13_box::main                       },
 Func{ name: "14 trait_object",             func: _14_trait_object::main              },
 Func{ name: "15 closure",                  func: _15_closure::main                   },
 Func{ name: "16 lifetime",                 func: _16_lifetime::main                  },
 Func{ name: "17 iterator",                 func: _17_iterator::main                  },
 Func{ name: "18 error_operation",          func: _18_error_operation::main           },
 Func{ name: "19 fmt_display",              func: _19_fmt_display::main               },
 Func{ name: "20 rc",                       func: _20_rc::main                        },
 Func{ name: "21 cell",                     func: _21_cell::main                      },
 Func{ name: "30 thread_channel",           func: _30_thread_channel::main            },

];
