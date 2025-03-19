mod lib;

fn main() {
    let mut arr: [i32; 10] = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
    println!("Original array: {:?}", arr);

    let mut alen = arr.len();
    let mut heap = lib::heap::Heap { arr: &mut arr, size: alen };
    
    // Build max heap
    lib::heap::buildMaxHeap(&mut heap);
    println!("After buildMaxHeap: {:?}", heap.arr);
    
    // Perform heap sort
    lib::heap::heapSort(&mut heap);
    println!("Sorted array: {:?}", heap.arr);
}