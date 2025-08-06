/*
Rustのトレイトとトレイト境界を理解する
https://qiita.com/Leapcell/items/4afd7c69f26cde56a009
*/

mod _01_trait_basic;

pub static r#REF: &str = r#"
Rustのトレイトとトレイト境界を理解する
https://qiita.com/Leapcell/items/4afd7c69f26cde56a009
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "01 trait_basic",              func: _01_trait_basic::main               },

];
