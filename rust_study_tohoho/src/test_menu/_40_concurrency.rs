
/*
 * 並行性（Concurrence）
 */

/*
並行性（Concurrence）と並列性（Parallel）の概念


 - 並行性（Concurrence）ある時間の範囲において、複数のタスクを扱うこと
         
      マルチスレッド
      非同期  async/await

 - 並列性（Parallel）   ある時間の点において、複数のタスクを扱うこと

*/

pub fn main() {


#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]


println!("--------------------  concurrence 1");
{
    use std::thread;

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

}

println!("--------------------  concurrence 2");
{

}
println!("--------------------  concurrence 3");
{

}

}

