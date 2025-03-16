use std::marker::PhantomData;
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

pub struct Iter<'a, K: Ord, V>
{
    pointer: *mut Node<K,V>,
    idx: usize,
    is_first: bool,
    _maker: PhantomData<&'a (K,V)>
}

impl<K:Ord, V> Node<K,V> {
    /// 传入节点指针和成员下标, 得到对应成员的键在 Ord Trait 意义下的下一个键的成员, 如果没有更大的成员则返回 None.
    /// is_child_index 表示下标是否是 children 数组的下标.
    unsafe fn get_next(this: *mut Self, index: usize, is_child_index: bool) -> Option<(*mut Self, usize)>
    {
        if is_child_index {
            if index < unsafe { (*this).members.len() }  { Some((this, index)) }
            else {
                match unsafe {(*this).parent } {
                    None => None, 
                    Some(parent) => unsafe { Self::get_next( parent, (*this).parent_idx, true) }
                }
            }
        }
        else {
            unsafe {
                match &mut (*this).children {
                    None => if index + 1 < (*this).members.len() { Some((this, index + 1)) }
                            else { Self::get_next(this, index + 1, true) }
                    Some(children) => {
                        let mut ptr = &mut children[index + 1];
                        while let Some(ref mut child) = (*ptr).children {
                            ptr = &mut child[0];
                        }

                        Some((box_as_mut_ptr(ptr), 0))
                    }
                }
            }
        }
    }
}

impl<'a, K:Ord, V> Iterator for Iter<'a, K,V>
{
    type Item = &'a (K,V);

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.is_first {
            self.is_first = false;
            unsafe {
                if (*self.pointer).members.is_empty() { None }
                else { Some( &(*self.pointer).members[self.idx] ) }
            }
        }
        else {
            match unsafe { Node::get_next(self.pointer, self.idx, false) }
            {
                None => None,
                Some((pt,i)) => {
                    self.pointer = pt;
                    self.idx = i;
                    Some( unsafe { &(*self.pointer).members[i] })
                }
            }
        }
    }
}

impl<'a, K:Ord, V> Btree<K,V> {
    pub fn iter(&self) -> Iter<'a, K,V>
    {
        match unsafe{ &mut (*self.root).children }
        {
            None => Iter{ pointer: self.root, idx:0,  is_first: true, _maker:PhantomData },
            Some(children) => {
                let mut ptr = &mut children[0];
                while let Some(ref mut child) = (*ptr).children {
                    ptr = &mut child[0];
                }

                Iter{ pointer: box_as_mut_ptr(ptr), idx: 0, is_first: true, _maker:PhantomData }
            }
        }
    }
}