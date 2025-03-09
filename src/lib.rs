pub mod heap{
    pub struct Heap<'a>{
        pub arr : &'a mut[i32],
        pub size : usize,
    }

    fn parent(i: usize) -> usize{
        return (i-1)/2;
    }

    fn left(i: usize) -> usize{
        return 2*(i+1) - 1;
    }

    fn right(i: usize) -> usize{
        return 2*(i+1);
    }

    pub fn swapNode(heap: &mut Heap, i : usize, j : usize){
        let tmp: i32 = heap.arr[i];
        heap.arr[i] = heap.arr[j];
        heap.arr[j] = tmp;
    }

    pub fn maxHeapify(heap: &mut Heap, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest: usize;

        if(l < heap.size && heap.arr[l] > heap.arr[i]){
            largest = l;
        } else{
            largest = i;
        }

        if(r < heap.size && heap.arr[r] > heap.arr[largest]){
            largest = r;
        }

        if(largest != i){
            //swap nodes
            swapNode(heap, i, largest);
            //call recursively max heapify
            maxHeapify(heap, largest);
        }
    }
}
