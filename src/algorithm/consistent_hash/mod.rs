use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, RandomState};
use std::sync::{Arc, Mutex, RwLock};

pub struct ConsistentHash<T, S = RandomState> {
    hash_builder: S,
    replicas: usize,
    inner: RwLock<Arc<ConsistentHashInner<T>>>,
    inner_mutex: Mutex<()>,
}

struct ConsistentHashInner<T> {
    physical_id: u64,
    physical_nodes: HashMap<u64, T>,
    virtual_nodes: Vec<u64>,
    ring: HashMap<u64, Vec<u64>>,
}

impl<T> ConsistentHash<T, RandomState> {
    pub fn new() -> Self {
        Self::with_replicas(Self::MAX_REPLICAS)
    }

    pub fn with_replicas(replicas: usize) -> Self {
        ConsistentHash {
            hash_builder: RandomState::new(),
            replicas,
            ..Default::default()
        }
    }
}

impl<T, S> ConsistentHash<T, S> {
    const MAX_WEIGHT: usize = 100;
    const MAX_REPLICAS: usize = 100;

    pub fn with_hasher(hash_builder: S) -> Self {
        Self::with_replicas_and_hasher(Self::MAX_REPLICAS, hash_builder)
    }

    pub fn with_replicas_and_hasher(replicas: usize, hash_builder: S) -> Self {
        ConsistentHash {
            hash_builder,
            replicas,
            inner: Default::default(),
            inner_mutex: Default::default(),
        }
    }

    pub fn replicas(&self) -> usize {
        self.replicas
    }
}

impl<T, S> ConsistentHash<T, S>
where
    S: Clone,
{
    pub fn hasher(&self) -> S {
        self.hash_builder.clone()
    }
}

impl<T, S> ConsistentHash<T, S>
where
    T: Clone,
{
    pub fn nodes(&self) -> Vec<T> {
        self.inner
            .read()
            .unwrap()
            .physical_nodes
            .values()
            .cloned()
            .collect()
    }

    pub fn clear(&self) {
        let new_inner = Arc::new(ConsistentHashInner::default());

        let _guard = self.inner_mutex.lock().unwrap();
        *self.inner.write().unwrap() = new_inner;
    }
}

impl<T, S> ConsistentHash<T, S>
where
    S: BuildHasher,
{
    fn hash<V: Hash>(&self, value: V) -> u64 {
        self.hash_builder.hash_one(value)
    }
}

impl<T, S> ConsistentHash<T, S>
where
    T: Clone,
    S: BuildHasher,
{
    pub fn add_node(&self, node: T) {
        self.add_node_replicas(node, self.replicas);
    }

    pub fn add_node_weight(&self, node: T, mut weight: usize) {
        weight = weight.min(Self::MAX_WEIGHT);
        self.add_node_replicas(node, self.replicas * weight / Self::MAX_WEIGHT);
    }

    pub fn add_node_replicas(&self, node: T, replicas: usize) {
        if replicas == 0 {
            return;
        }
        let replicas = replicas.min(self.replicas);

        let _guard = self.inner_mutex.lock().unwrap();

        let inner = self.inner.read().unwrap().clone();
        let mut inner = inner.as_ref().clone();

        inner.physical_nodes.insert(inner.physical_id, node);
        for i in 0..replicas {
            let key = if self.replicas == 1 {
                self.hash(inner.physical_id)
            } else {
                self.hash((inner.physical_id, i))
            };
            inner.virtual_nodes.push(key);
            inner.ring.entry(key).or_default().push(inner.physical_id);
        }
        inner.physical_id += 1;
        inner.virtual_nodes.sort_unstable();

        *self.inner.write().unwrap() = Arc::new(inner);
    }
}

impl<T, S> ConsistentHash<T, S>
where
    T: Clone + PartialEq,
    S: BuildHasher,
{
    pub fn remove_node(&self, node: T) {
        let mut existing_keys = HashSet::with_capacity(self.replicas);

        let _guard = self.inner_mutex.lock().unwrap();

        let inner = self.inner.read().unwrap().clone();
        let mut inner = inner.as_ref().clone();

        let Some(idx) =
            inner
                .physical_nodes
                .iter()
                .find_map(|(i, n)| if *n == node { Some(*i) } else { None })
        else {
            return;
        };
        for i in 0..self.replicas {
            let key = if self.replicas == 1 {
                self.hash(idx)
            } else {
                self.hash((idx, i))
            };
            if inner.virtual_nodes.binary_search(&key).is_ok() {
                existing_keys.insert(key);
            }
        }
        inner.virtual_nodes.retain(|k| !existing_keys.contains(k));
        inner.physical_nodes.remove(&idx);
        inner.ring.retain(|k, v| {
            if !existing_keys.contains(k) {
                return true;
            }
            v.retain(|n| *n != idx);
            !v.is_empty()
        });

        *self.inner.write().unwrap() = Arc::new(inner);
    }
}

impl<T, S> ConsistentHash<T, S>
where
    T: Clone,
    S: BuildHasher,
{
    pub fn get_node<V: Hash>(&self, key: V) -> Option<T> {
        let hash = self.hash(key);

        let inner = self.inner.read().unwrap().clone();
        if inner.physical_nodes.is_empty() {
            return None;
        }
        let pos = match inner.virtual_nodes.binary_search(&hash) {
            Ok(pos) => pos,
            Err(pos) => pos % inner.virtual_nodes.len(),
        };
        let key = inner.virtual_nodes[pos];

        // TODO: use load balancing
        let physical_idx = inner.ring.get(&key).unwrap()[0];

        inner.physical_nodes.get(&physical_idx).cloned()
    }
}

impl<T, S> Default for ConsistentHash<T, S>
where
    S: Default,
{
    fn default() -> Self {
        ConsistentHash::with_hasher(Default::default())
    }
}

impl<T> Default for ConsistentHashInner<T> {
    fn default() -> Self {
        ConsistentHashInner {
            physical_id: 0,
            physical_nodes: HashMap::new(),
            virtual_nodes: Vec::new(),
            ring: HashMap::new(),
        }
    }
}

impl<T> Clone for ConsistentHashInner<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ConsistentHashInner {
            physical_id: self.physical_id,
            physical_nodes: self.physical_nodes.clone(),
            virtual_nodes: self.virtual_nodes.clone(),
            ring: self.ring.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hasher;

    #[test]
    fn test_base_consistent_hash() {
        let ch = ConsistentHash::new();

        let node1 = "node1";
        let node2 = "node2";
        ch.add_node(node1);
        ch.add_node(node2);

        let random_key = "key1";

        let get1 = ch.get_node(random_key).unwrap();
        ch.remove_node(get1);
        let get2 = ch.get_node(random_key).unwrap();
        if get1 == node1 {
            assert_eq!(get2, node2);
        } else {
            assert_eq!(get2, node1);
        }
    }

    #[test]
    fn test_self_defined_hash_builder_and_one_replicas() {
        #[derive(Default)]
        struct Multiple2U64Hasher(u64);

        impl Hasher for Multiple2U64Hasher {
            fn write(&mut self, bytes: &[u8]) {
                self.0 = u64::from_ne_bytes(bytes.try_into().unwrap()).wrapping_mul(2);
            }

            fn finish(&self) -> u64 {
                self.0
            }
        }

        struct Multiple2U64HasherBuilder;

        impl BuildHasher for Multiple2U64HasherBuilder {
            type Hasher = Multiple2U64Hasher;

            fn build_hasher(&self) -> Self::Hasher {
                Multiple2U64Hasher::default()
            }
        }

        let ch = ConsistentHash::with_replicas_and_hasher(1, Multiple2U64HasherBuilder);

        let node = "node";
        let node1 = "node1";
        let node2 = "node2";
        let node3 = "node3";
        ch.add_node(node1);
        ch.add_node(node);
        ch.remove_node(node);
        ch.add_node(node2);

        assert_eq!(ch.get_node(0u64).unwrap(), node1);
        assert_eq!(ch.get_node(1u64).unwrap(), node2);
        assert_eq!(ch.get_node(2u64).unwrap(), node2);
        assert_eq!(ch.get_node(3u64).unwrap(), node1);

        ch.add_node(node);
        ch.remove_node(node);
        ch.add_node(node3);
        assert_eq!(ch.get_node(3u64).unwrap(), node3);
        assert_eq!(ch.get_node(4u64).unwrap(), node3);
        assert_eq!(ch.get_node(5u64).unwrap(), node1);

        ch.remove_node(node2);
        assert_eq!(ch.get_node(0u64).unwrap(), node1);
        assert_eq!(ch.get_node(1u64).unwrap(), node3);
        assert_eq!(ch.get_node(2u64).unwrap(), node3);
        assert_eq!(ch.get_node(3u64).unwrap(), node3);
        assert_eq!(ch.get_node(4u64).unwrap(), node3);
        assert_eq!(ch.get_node(5u64).unwrap(), node1);
    }
}
