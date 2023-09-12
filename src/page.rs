use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const PAGE_SIZE: usize = 4 * 1024;

pub type PageId = u32;

pub struct Page(RwLock<PageInner>);

impl Default for Page {
    fn default() -> Self {
        let inner = PageInner::default();

        Self(RwLock::new(inner))
    }
}

impl Page {
    pub fn read(&self) -> RwLockReadGuard<'_, PageInner> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, PageInner> {
        self.0.write().unwrap()
    }
}

pub struct PageInner {
    id: PageId,
    pub data: [u8; PAGE_SIZE],
}

impl Default for PageInner {
    fn default() -> Self {
        Self {
            id: 0,
            data: [0; PAGE_SIZE],
        }
    }
}

impl PageInner {
    pub fn reset(&mut self) {
        self.id = 0;
        self.data.fill(0);
    }
}

#[cfg(test)]
mod test {
    use crate::page::*;

    #[test]
    pub fn test() {
        let page = Page::default();

        let mut page_w = page.write();

        let new_data = [1; PAGE_SIZE];

        page_w.data[..].copy_from_slice(&new_data);

        assert!(page_w.data == new_data);
    }
}
