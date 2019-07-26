use iter::iter_test;

fn main() {
   let counter = iter_test::Counter::new();
   
   for count in counter {
       println!("{}", count);
   }
    
}
