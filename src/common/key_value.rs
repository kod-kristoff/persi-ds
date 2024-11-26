use core::cmp::Ordering;

#[derive(Debug)]
pub struct KeyValue<K, V>(pub K, pub V);

impl<K, V> Copy for KeyValue<K, V>
where
    K: Copy,
    V: Copy,
{
}

impl<K, V> Clone for KeyValue<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        KeyValue(self.0.clone(), self.1.clone())
    }
}

impl<K, V> PartialOrd for KeyValue<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K, V> PartialEq for KeyValue<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K, V> PartialOrd<K> for KeyValue<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl<K, V> PartialEq<K> for KeyValue<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &K) -> bool {
        self.0 == *other
    }
}
