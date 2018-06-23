use memory::PAGE_SIZE;
use super::*;

// TODO: use BitAllocator & alloc fixed size stack
pub struct StackAllocator {
    range: PageRange,
}

impl StackAllocator {
    pub fn new(page_range: PageRange) -> StackAllocator {
        StackAllocator { range: page_range }
    }
}

impl StackAllocator {
    pub fn alloc_stack(&mut self, active_table: &mut ActivePageTable,
                       size_in_pages: usize) -> Option<Stack> {
        if size_in_pages == 0 {
            return None; /* a zero sized stack makes no sense */
        }

        // clone the range, since we only want to change it on success
        let mut range = self.range.clone();

        // try to allocate the stack pages and a guard page
        let guard_page = range.next();
        let stack_start = range.next();
        let stack_end = if size_in_pages == 1 {
            stack_start
        } else {
            // choose the (size_in_pages-2)th element, since index
            // starts at 0 and we already allocated the start page
            range.nth(size_in_pages - 2)
        };

        match (guard_page, stack_start, stack_end) {
            (Some(_), Some(start), Some(end)) => {
                // success! write back updated range
                self.range = range;

                // map stack pages to physical frames
                for page in Page::range_inclusive(start, end) {
                    let frame = alloc_frame();
                    active_table.map_to(page, frame);
                }

                // create a new stack
                let top_of_stack = end.start_address() + PAGE_SIZE;
                Some(Stack::new(top_of_stack.as_u64() as usize, start.start_address().as_u64() as usize))
            }
            _ => None, /* not enough pages */
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    top: usize,
    bottom: usize,
}

impl Stack {
    pub(super) fn new(top: usize, bottom: usize) -> Stack {
        assert!(top > bottom);
        Stack {
            top,
            bottom,
        }
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn bottom(&self) -> usize {
        self.bottom
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        warn!("stack leak: {:x?}", self);
    }
}