use crate::{print, println};
use core::ptr;
use core::alloc::{Layout, GlobalAlloc};
use core::cell::UnsafeCell;

pub struct BumpPointerAlloc {
    pub head: UnsafeCell<usize>, // メモリ領域の先頭
    pub end: usize // メモリ領域の末尾
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    // メモリの割り当て
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // layout 型にほしいメモリ領域が書いてある (layout.align())
        let head = self.head.get(); // メモリ領域の先頭
        let align = layout.align(); // 必要なアラインメント
        let size = layout.size();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };

        if start + align > self.end {
            ptr::null_mut() // メモリ割当不可 → NullPointer を返す
        } else {
            *head = start + size;
            println!("head:{:?}, align:{:?}, res:{:?}, start:{:?}", head, align, res, start as *mut u8);
            start as *mut u8 // メモリ割当可能 → メモリの先頭アドレスを返す
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // メモリの開放はしない
    }
}

// #[alloc_error_handler]
// fn on_oom(_layout: Layout) -> ! {
//     loop {}
// }
