
/*
 *
 */

//mod config;   


#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

/*
#[derive(Debug)]
println!("{:#?}", foo);
*/

macro_rules! test_block {
    ( $op:expr, $sign:expr, $counter:expr, $target:expr, $block1:block, $block2:block ) => ( 
      if $op == "menu" {
         if $sign == "==" { if $counter == $target { $block1 };  }
         if $sign == "!=" { if $counter != $target { $block1 };  }
         if $sign == ">"  { if $counter >  $target { $block1 };  }
         if $sign == ">=" { if $counter >= $target { $block1 };  }
         if $sign == "<"  { if $counter <  $target { $block1 };  }
         if $sign == "<=" { if $counter <= $target { $block1 };  }
      }
      if $op == "exec" {
         if $sign == "==" { if $counter == $target { $block1 ; $block2};  }
         if $sign == "!=" { if $counter != $target { $block1 ; $block2};  }
         if $sign == ">"  { if $counter >  $target { $block1 ; $block2};  }
         if $sign == ">=" { if $counter >= $target { $block1 ; $block2};  }
         if $sign == "<"  { if $counter <  $target { $block1 ; $block2};  }
         if $sign == "<=" { if $counter <= $target { $block1 ; $block2};  }
      }
      $counter = $counter +1;)
}


use crate::Config;
pub fn main() {
    
    if Config::current().debug_mode {
        println!("*********************");
        println!("**** DEBUG MODE  ****");
        println!("*********************");
    }

let t = 0;
let sign = "!=";
//let op = "menu";  // menu / exec
let op = "exec";  // menu / exec


let mut c = 1;
/*
fn fis() -> bool {
  c == t
}
*/
/*
test_block!(c,t, {
println!("-------------------- 1");
});

test_block!(c,t, {
println!("-------------------- 2");
});

test_block!(c,t, {
println!("-------------------- 3");
});
*/

test_block!( op, sign, c,t, 
{
println!("-------------------- 1");
},
{
println!("exec 1");
}
);

test_block!( op , sign, c,t, 
{
println!("-------------------- 2");
},
{
println!("exec 2");
}
);

test_block!( op , sign, c,t, 
{
println!("-------------------- 3");
},
{
println!("exec 3");
}
);

test_block!( op , sign, c,t, 
{
println!("-------------------- 4");
},
{
println!("exec 4");
}
);

test_block!( op , sign, c,t, 
{
println!("-------------------- 5");
},
{
println!("exec 5");
}
);

}

