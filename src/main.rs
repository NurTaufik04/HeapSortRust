mod lib;

fn main() {
   //println!("Hello World");
   let mut arr: [i32;10] = [4,1,3,2,16,9,10,14,8,7];
   println!("Original array: {:?}",arr);
   let mut alen = arr.len();
   let mut heap : lib::heap::Heap = lib::heap::Heap{arr: &mut arr, size: alen};
   lib::heap::swapNode(&mut heap, 0,1);
   println!("after swap: {:?}", heap.arr);

  let mut arr = [16,4,10,14,7,9,3,2,8,1];
  alen = arr.len();
  heap = lib::heap::Heap{arr: &mut arr, size: alen};
  lib::heap::maxHeapify(&mut heap, 1);
  println!("after heapify: {:?}", heap.arr);
}
