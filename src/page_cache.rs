use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::page::{Page, PageId};

const DEFAULT_CACHE_SIZE: usize = 8;

pub type FrameId = usize;

#[derive(Clone)]
pub struct PageCache<const SIZE: usize = DEFAULT_CACHE_SIZE>(Arc<PageCacheInner<SIZE>>);

impl<const SIZE: usize> PageCache<SIZE> {
    pub fn new() -> Self {
        let inner = PageCacheInner::new();

        Self(Arc::new(inner))
    }

    pub fn fetch_page(&self, page_id: PageId) -> Option<&Page> {
        self.0.fetch_page(page_id)
    }

    pub fn new_page(&self) -> Option<&Page> {
        self.0.new_page()
    }

    #[cfg(test)]
    pub fn set_page(&self, frame_id: FrameId, page_id: PageId) {
        self.0.page_table.write().unwrap().insert(page_id, frame_id);
    }
}

struct PageCacheInner<const SIZE: usize> {
    page_table: RwLock<HashMap<PageId, FrameId>>,
    pages: [Page; SIZE],
}

impl<const SIZE: usize> PageCacheInner<SIZE> {
    pub fn new() -> Self {
        let page_table = RwLock::new(HashMap::new());
        let pages = std::array::from_fn(|_| Page::default());

        Self { page_table, pages }
    }

    pub fn fetch_page(&self, page_id: PageId) -> Option<&Page> {
        let page_table = self.page_table.read().unwrap();
        let Some(i) = page_table.get(&page_id) else { return None };

        Some(&self.pages[*i])
    }

    pub fn new_page(&self) -> Option<&Page> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use crate::page::PAGE_SIZE;
    use crate::page_cache::*;

    #[test]
    fn test_multithread_compiles() {
        let pc = PageCache::<DEFAULT_CACHE_SIZE>::new();
        pc.set_page(0, 1);

        const HALF_PAGE: usize = PAGE_SIZE / 2;

        let pc1 = pc.clone();
        let jh1 = thread::spawn(move || {
            let page = pc1.fetch_page(1).unwrap();

            let mut page_w = page.write();

            let data = [1; HALF_PAGE];
            page_w.data[..HALF_PAGE].copy_from_slice(&data);
        });

        let pc2 = pc.clone();
        let jh2 = thread::spawn(move || {
            let page = pc2.fetch_page(1).unwrap();

            let mut page_w = page.write();

            let data = [2; HALF_PAGE];
            page_w.data[HALF_PAGE..].copy_from_slice(&data);
        });

        jh1.join().unwrap();
        jh2.join().unwrap();

        let page = pc.fetch_page(1).unwrap();

        let page_r = page.read();

        let mut expected = vec![1; HALF_PAGE];
        expected.extend_from_slice(&[2; HALF_PAGE]);

        assert!(expected.as_slice() == page_r.data);
    }
}
