use std::ops::Deref;
use web_sys::Node;

/// Ensures that the wrapped node is automatically removed from the document tree when it goes out of scope.
pub struct DomRef<T>
where
    T: AsRef<Node> + Clone,
{
    inner: T,
}

impl<T> DomRef<T>
where
    T: AsRef<Node> + Clone,
{
    pub fn new(node: &T) -> Self {
        Self { inner: node.clone() }
    }
}

impl<T> Deref for DomRef<T>
where
    T: AsRef<Node> + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Drop for DomRef<T>
where
    T: AsRef<Node> + Clone,
{
    fn drop(&mut self) {
        let node = self.inner.as_ref();
        if let Some(parent) = node.parent_node() {
            let _ = parent.remove_child(node);
        }
    }
}
