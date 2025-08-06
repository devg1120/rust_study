/*
Rustの新しいWEBフレームワークaxumを触ってみた
https://zenn.dev/techno_tanoc/articles/99e54c82cb049f

AxumでのExtensionの使い方 - ミドルウェアとデータを共有する
https://zenn.dev/techno_tanoc/articles/a295da5efcd494#user-%E3%81%AB-fromrequestparts-%E3%82%92%E5%AE%9F%E8%A3%85%E3%81%99%E3%82%8B
*/

mod _10_hello_world;
mod _11_json_api;
mod _12_path_params;
mod _13_path_params_json;
mod _14_custom_extractor;
mod _15_state;
mod _16_response;
mod _17_api_version;
mod _20_auth;

pub static r#REF: &str = r#"
Rustの新しいWEBフレームワークaxumを触ってみた
https://zenn.dev/techno_tanoc/articles/99e54c82cb049f

AxumでのExtensionの使い方 - ミドルウェアとデータを共有する
https://zenn.dev/techno_tanoc/articles/a295da5efcd494#user-%E3%81%AB-fromrequestparts-%E3%82%92%E5%AE%9F%E8%A3%85%E3%81%99%E3%82%8B
"#;

pub struct  Func<'a>{
    pub name: &'a str,
    pub func: fn(),
}

pub static  TEST_ARRAY: &[Func] = &[

 Func{ name: "10 hello_world",              func: _10_hello_world::main               },
 Func{ name: "11 json_api",                 func: _11_json_api::main                  },
 Func{ name: "12 path_params",              func: _12_path_params::main               },
 Func{ name: "13 path_params_json",         func: _13_path_params_json::main          },
 Func{ name: "14 custom_extractor",         func: _14_custom_extractor::main          },
 Func{ name: "15 state",                    func: _15_state::main                     },
 Func{ name: "16 response",                 func: _16_response::main                  },
 Func{ name: "17 api_version",              func: _17_api_version::main               },
 Func{ name: "20 auth",                     func: _20_auth::main                      },

];
