pub struct DisjointSets {
    parent_pointer: Vec<usize>,
    set_size: Vec<usize>    // warning: sizes are only valid for set roots!
}

impl DisjointSets {

    /// Create a new disjoint sets data structure where every element represents a separate set.
    pub fn new(element_count: usize) -> DisjointSets {
        return DisjointSets {
            // Initially, each parent pointer loops to the same element (all elements are disjoint)
            parent_pointer: (0..element_count).collect(),
            // Initially, each set has size one (the element itself)
            set_size: vec![1; element_count]
        }
    }

    /// Find a unique representing element for the set where the given element resides.
    /// As a side-effect, the find operation can flatten the visited path and hence needs
    /// to borrow the data structure mutably.
    ///
    /// TODO: Consider adding a variant of find which doesn't need mutable borrow.
    /// TODO: Using recursion for path compaction is potentially costly, try using path halving, etc.
    pub fn find(&mut self, el: usize) -> usize {
        return if self.parent_pointer[el] == el {
            el
        } else {
            let root = self.find(self.parent_pointer[el]);
            self.parent_pointer[el] = root;
            root
        }
    }

    /// Union two sets given by their two arbitrary elements.
    ///
    /// TODO: Consider adding a faster variant where we already know the elements are roots.
    pub fn union(&mut self, el1: usize, el2: usize) {
        let root1 = self.find(el1);
        let root2 = self.find(el2);
        if self.set_size[root1] > self.set_size[root2] {
            // attach set 2 under set 1
            self.parent_pointer[root2] = root1;
            self.set_size[root1] += self.set_size[root2]
        } else {
            // attach set 1 under set 2
            self.parent_pointer[root1] = root2;
            self.set_size[root2] += self.set_size[root1]
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn make_test() {
        let mut ds = DisjointSets::new(10);
        for el in 0..10 {
            let root = ds.find(el);
            assert_eq!(root, el);
        }
    }

    #[test]
    fn union_test() {
        let mut ds = DisjointSets::new(5);

        assert_ne!(ds.find(0), ds.find(1));
        ds.union(0, 1);
        assert_eq!(ds.find(0), ds.find(1));

        assert_ne!(ds.find(2), ds.find(3));
        ds.union(2, 3);
        assert_eq!(ds.find(2), ds.find(3));

        assert_ne!(ds.find(0), ds.find(2));
        assert_ne!(ds.find(0), ds.find(3));
        assert_ne!(ds.find(1), ds.find(2));
        assert_ne!(ds.find(1), ds.find(3));
        ds.union(2, 0);
        assert_eq!(ds.find(1), ds.find(3));

        assert_ne!(ds.find(0), ds.find(4));
    }

}
