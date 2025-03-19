pub mod heap {
    pub struct Heap<'a> {
        pub arr: &'a mut [i32],
        pub size: usize,
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(i: usize) -> usize {
        2 * (i + 1) - 1
    }

    fn right(i: usize) -> usize {
        2 * (i + 1)
    }

    pub fn swapNode(heap: &mut Heap, i: usize, j: usize) {
        let tmp = heap.arr[i];
        heap.arr[i] = heap.arr[j];
        heap.arr[j] = tmp;
    }

    pub fn maxHeapify(heap: &mut Heap, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest = i;

        if l < heap.size && heap.arr[l] > heap.arr[i] {
            largest = l;
        }

        if r < heap.size && heap.arr[r] > heap.arr[largest] {
            largest = r;
        }

        if largest != i {
            swapNode(heap, i, largest);
            maxHeapify(heap, largest);
        }
    }

    pub fn buildMaxHeap(heap: &mut Heap) {
        let start = (heap.size / 2) as isize - 1;
        for i in (0..=start).rev() {
            maxHeapify(heap, i as usize);
        }
    }

    pub fn heapSort(heap: &mut Heap) {
        buildMaxHeap(heap);

        for i in (1..heap.size).rev() {
            swapNode(heap, 0, i);
            heap.size -= 1;
            maxHeapify(heap, 0);
        }
    }
}