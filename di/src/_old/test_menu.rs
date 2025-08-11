/*
Rustのトレイトとトレイト境界を理解する
https://qiita.com/Leapcell/items/4afd7c69f26cde56a009

[入門] トレイトオブジェクトとトレイト境界
https://zenn.dev/hakoten/articles/058a681ba6fe4a
*/

mod _01_di;

pub static r#REF: &str = r#"
Rustのトレイトとトレイト境界を理解する
https://qiita.com/Leapcell/items/4afd7c69f26cde56a009

[入門] トレイトオブジェクトとトレイト境界
https://zenn.dev/hakoten/articles/058a681ba6fe4a
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "01 di",                       func: _01_di::main                        },

];
