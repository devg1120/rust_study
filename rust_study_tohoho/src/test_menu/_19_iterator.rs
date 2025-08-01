#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
   println!("-------------------- iterator");
   struct Counter {
       max: u32,
       count: u32,
   }
   
   impl Counter {
       fn new(max: u32) -> Counter {
           Counter { max: max, count: 0 }
       }
   }
   
   impl Iterator for Counter {
       type Item = u32;
       fn next(&mut self) -> Option<Self::Item> {
           self.count += 1;
           if self.count < self.max {
               Some(self.count)
           } else {
               None
           }
       }
   }
   
   let counter = Counter::new(10);
   for c in counter {
       println!("{}", c);
   }
}
