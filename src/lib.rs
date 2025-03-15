use std::mem::replace;
use std::ptr;

const RANK:usize = 5;
struct Node<K:Ord, V>
{
    members: Vec<(K,V)>,
    children: Option<Vec<Box<Self>>>,
    parent: Option<*mut Self>,
    parent_idx: usize
}
enum SearchResult<K:Ord, V>
{
    Found(*mut Node<K,V>, usize),
    NonFound(*mut Node<K,V>, usize)
}

fn box_as_mut_ptr<T>(b: &mut Box<T>) -> *mut T
{
    ptr::from_mut(b.as_mut())
}

impl<K:Ord, V> Node<K,V>
{
    fn search(this: &Self, key: &K) -> SearchResult<K,V>
    {
        let index = match this.members.iter().position(|(k,_)| k >= key )
        {
            None => this.members.len(),
            Some(idx) if this.members[idx].0 == *key => return SearchResult::Found(ptr::from_ref(this) as *mut Self, idx),
            Some(idx) => idx
        };

        match this.children
        {
            None => SearchResult::NonFound(ptr::from_ref(this) as *mut Self, index),
            Some(ref children) => Self::search(&children[index], key)
        }
    }

    /// 传入一个插入目标节点的指针, 如果不产生新的根节点则返回 None, 如果有新的跟节点, 则返回新根节点的 Box 指针.
    unsafe fn insert(this: *mut Self, index: usize, key: K, value: V) -> Option<Box<Self>>
    {
        let this_ref = unsafe { this.as_mut().expect("核心 insert 不能传入空指针") };
        this_ref.members.insert(index, (key,value));

        if this_ref.members.len() < RANK { None }
        else {
            let right_members = this_ref.members.split_off((RANK + 1) / 2);
            let mid_member = this_ref.members.pop().unwrap();

            let mut new_right_node = Box::new(Self{
                members: right_members,
                children: None,
                parent: None,
                parent_idx: this_ref.parent_idx + 1
            });

            if let Some(ref mut children) = this_ref.children {
                let mut right_children = children.split_off((RANK + 1) / 2);

                right_children.iter_mut().enumerate().for_each(|(i,child)| {
                    child.parent_idx = i;
                    
                    child.parent = Some(box_as_mut_ptr(&mut new_right_node));
                });

                new_right_node.children = Some(right_children);
            }

            match this_ref.parent
            {
                None => {
                    let new_right_node_parent = ptr::from_mut(&mut new_right_node.parent); 
                    // 绕过借用检查, 实现环形引用, 但是因为 parent 的 *mut Self 本质上只是简单的整数, 其类型是指针, 
                    // 没有实现 Drop trait, 因此在 Node 释放的时候不会去释放 parent 所指向的地址

                    let mut new_root_node = Box::new(Self{
                        members: vec![mid_member],
                        children: Some(vec![ unsafe{ Box::from_raw(this) } , new_right_node]),
                        parent: None,
                        parent_idx: 0
                    });
                    unsafe { *new_right_node_parent  = Some(box_as_mut_ptr(&mut new_root_node)); }
                    this_ref.parent = Some(box_as_mut_ptr(&mut new_root_node));
                    
                    Some(new_root_node)
                }
                Some(parent) => {
                    new_right_node.parent = Some(parent);

                    unsafe{ 
                        (*parent).children.as_mut().unwrap()[this_ref.parent_idx + 1 ..].iter_mut().for_each(|child| child.parent_idx += 1);
                        (*parent).children.as_mut().unwrap().insert(new_right_node.parent_idx, new_right_node);

                        Self::insert(parent, this_ref.parent_idx, mid_member.0, mid_member.1)
                    }
                }
            }
        }
    }
}

pub struct Btree<K:Ord, V>
{
    root: *mut Node<K,V>
}

impl<K:Ord, V> Btree<K,V>
{
    pub fn new() -> Self
    {
        Self
        {
            root: Box::into_raw(Box::new(Node{
                members: Vec::new(),
                children: None,
                parent: None,
                parent_idx: 0,
            }))
        }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    {
        unsafe{
            match Node::search(self.root.as_ref().unwrap(), key)
            {
                SearchResult::Found(p, idx) => Some(&(*p).members[idx].1),
                SearchResult::NonFound(_, _) => None
            }
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    {
        unsafe{
            match Node::search(self.root.as_ref().unwrap(), key)
            {
                SearchResult::Found(p, idx) => Some(&mut (*p).members[idx].1),
                SearchResult::NonFound(_, _) => None
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    {
        unsafe{
            match Node::search(self.root.as_ref().unwrap(), &key)
            {
                SearchResult::Found(p, idx) => Some(replace(&mut (*p).members[idx].1, value)),
                SearchResult::NonFound(p, idx) => {
                    if let Some(new_root) = Node::insert(p, idx, key, value) {
                        self.root = Box::into_raw(new_root);
                    }
                    None
                }
            }
        }
    }
}

impl<K:Ord, V> Drop for Btree<K,V>
{
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.root)) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test() -> Btree<i32, i32>
    {
        
        let mut btree = Btree::new();

        btree.insert(17, 1);
        btree.insert(6, 2);
        btree.insert(13, 3);
        btree.insert(23, 4);
        btree.insert(35, 5);
        btree.insert(47, 6);
        btree.insert(55, 7);
        btree.insert(1, 8);
        btree.insert(4, 9);
        btree.insert(8, 10);
        btree.insert(11, 11);
        btree.insert(14, 12);
        btree.insert(16, 13);
        btree.insert(19, 14);
        btree.insert(22, 15);
        btree.insert(27, 16);
        btree.insert(34, 17);
        btree.insert(38, 18);
        btree.insert(45, 19);
        btree.insert(49, 20);
        btree.insert(53, 21);
        btree.insert(65, 22);
        btree.insert(74, 23);
        btree.insert(79, 24);

        btree
    }

    #[test]
    fn read_works()
    {
        let btree_example = init_test();

        assert_eq!(*btree_example.get(&17).expect("本应找到而未找到"), 1);
        assert_eq!(*btree_example.get(&6).expect("本应找到而未找到"), 2);
        assert_eq!(*btree_example.get(&13).expect("本应找到而未找到"), 3);
        assert_eq!(*btree_example.get(&23).expect("本应找到而未找到"), 4);
        assert_eq!(*btree_example.get(&35).expect("本应找到而未找到"), 5);
        assert_eq!(*btree_example.get(&47).expect("本应找到而未找到"), 6);
        assert_eq!(*btree_example.get(&55).expect("本应找到而未找到"), 7);
        assert_eq!(*btree_example.get(&1).expect("本应找到而未找到"), 8);
        assert_eq!(*btree_example.get(&4).expect("本应找到而未找到"), 9);
        assert_eq!(*btree_example.get(&8).expect("本应找到而未找到"), 10);
        assert_eq!(*btree_example.get(&11).expect("本应找到而未找到"), 11);
        assert_eq!(*btree_example.get(&14).expect("本应找到而未找到"), 12);
        assert_eq!(*btree_example.get(&16).expect("本应找到而未找到"), 13);
        assert_eq!(*btree_example.get(&19).expect("本应找到而未找到"), 14);
        assert_eq!(*btree_example.get(&22).expect("本应找到而未找到"), 15);
        assert_eq!(*btree_example.get(&27).expect("本应找到而未找到"), 16);
        assert_eq!(*btree_example.get(&34).expect("本应找到而未找到"), 17);
        assert_eq!(*btree_example.get(&38).expect("本应找到而未找到"), 18);
        assert_eq!(*btree_example.get(&45).expect("本应找到而未找到"), 19);
        assert_eq!(*btree_example.get(&49).expect("本应找到而未找到"), 20);
        assert_eq!(*btree_example.get(&53).expect("本应找到而未找到"), 21);
        assert_eq!(*btree_example.get(&65).expect("本应找到而未找到"), 22);
        assert_eq!(*btree_example.get(&74).expect("本应找到而未找到"), 23);
        assert_eq!(*btree_example.get(&79).expect("本应找到而未找到"), 24);

        assert!(btree_example.get(&5).is_none(), "找到不存在的值");

    }

    #[test]
    fn write_works()
    {
        let mut btree_example = init_test();
        *btree_example.get_mut(&55).unwrap() = -7;
        assert_eq!(*btree_example.get(&55).expect("本应找到而未找到"), -7);

        *btree_example.get_mut(&45).unwrap() = -19;
        assert_eq!(*btree_example.get(&45).expect("本应找到而未找到"), -19);

    }
}
