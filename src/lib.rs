use std::mem::replace;
use std::ptr;

const RANK:usize = 5;
struct Node<K:Ord, V>
{
    members: Vec<(K,V)>,
    children: Option<Vec<Box<Self>>>,
    parent: Option<(*mut Self, usize)>,
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

    /// 传入一个插入目标节点的引用, 如果不产生新的根节点则返回 None, 如果有新的跟节点, 则返回新根节点的 Box 指针.
    fn insert(this: &mut Self, index: usize, key: K, value: V) -> Option<Box<Self>>
    {
        this.members.insert(index, (key,value));

        if this.members.len() < RANK { None }
        else {
            let right_members = this.members.split_off((RANK + 1) / 2);
            let mid_member = this.members.pop().unwrap();

            let mut new_right_node = Box::new(Self{
                members: right_members,
                children: None,
                parent: None
            });

            if let Some(ref mut children) = this.children {
                let mut right_children = children.split_off((RANK + 1) / 2);

                right_children.iter_mut().enumerate().for_each(|(i,child)| {
                    child.parent = Some((box_as_mut_ptr(&mut new_right_node), i))
                });

                new_right_node.children = Some(right_children);
            }

            match this.parent
            {
                None => {
                    let new_right_node_parent = ptr::from_mut(&mut new_right_node.parent); 
                    // 绕过借用检查, 实现环形引用, 但是因为 parent 的 *mut Self 本质上只是简单的整数, 其类型是指针, 
                    // 没有实现 Drop trait, 因此在 Node 释放的时候不会去释放 parent 所指向的地址

                    let mut new_root_node = Box::new(Self{
                        members: vec![mid_member],
                        children: Some(vec![ unsafe{ Box::from_raw(ptr::from_mut(this)) }, new_right_node]),
                        parent: None
                    });
                    unsafe { *new_right_node_parent  = Some((box_as_mut_ptr(&mut new_root_node), 1)); }
                    this.parent = Some((box_as_mut_ptr(&mut new_root_node), 0));
                    
                    Some(new_root_node)
                }
                Some((parent,parent_idx)) => {
                    new_right_node.parent = Some((parent, parent_idx + 1));

                    unsafe{ 
                        (*parent).children.as_mut().unwrap()[parent_idx + 1 ..].iter_mut().for_each(|child|
                            child.parent.as_mut().unwrap().1 += 1
                        );
                        (*parent).children.as_mut().unwrap().insert(parent_idx + 1, new_right_node);

                        Self::insert(parent.as_mut().unwrap(), parent_idx, mid_member.0, mid_member.1)
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
                parent: None
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
                    if let Some(new_root) = Node::insert(p.as_mut().unwrap(), idx, key, value) {
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
    current_node: &'a Node<K,V>,
    idx: usize,
    is_first: bool
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
                    Some((parent, index)) => unsafe { Self::get_next( parent, index, true) }
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
            if self.current_node.members.is_empty() { None }
            else { Some( &self.current_node.members[self.idx] ) }
        }
        else {
            match unsafe { Node::get_next(ptr::from_ref(self.current_node) as *mut Node<K,V>, self.idx, false) }
            {
                None => None,
                Some((pt,i)) => {
                    self.current_node = unsafe { pt.as_ref().unwrap() };
                    self.idx = i;
                    Some( &self.current_node.members[i] )
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
            None => Iter{ current_node: unsafe { self.root.as_ref().unwrap() }, idx:0,  is_first: true },
            Some(children) => {
                let mut ptr = &mut children[0];
                while let Some(ref mut child) = (*ptr).children {
                    ptr = &mut child[0];
                }

                Iter{ current_node: (*ptr).as_ref() , idx: 0, is_first: true }
            }
        }
    }
}


impl<K: Ord, V> Node<K,V>
{
    /// 从兄弟节点移动成员到本节点, origin 是 true 表示右边节点减少成员, origin 是 false 表示左边节点减少成员. 本函数不检查左边或者右边是否有兄弟节点.
    fn get_from_sibling(&mut self, origin: bool)
    {
        let (parent, parent_idx)  = unsafe { 
            let (a,b) = self.parent.expect("必须要有父节点");
            (a.as_mut().unwrap(), b)
        };
        if origin {
            let right_sibling = &mut parent.children.as_mut().unwrap()[parent_idx + 1];
            let new_mid_member = right_sibling.members.remove(0); //提取右兄弟的第一个成员
            let new_member = replace(&mut parent.members[parent_idx], new_mid_member);
            self.members.push(new_member);

            if let Some(ref mut right_children) = right_sibling.children {
                let mut child = right_children.remove(0);
                right_children.iter_mut().for_each(|item| item.parent.as_mut().unwrap().1 -= 1);

                child.parent = Some((ptr::from_mut(self), self.members.len()));
                self.children.as_mut().unwrap().push(child);
            }
        }
        else {
            let left_sibling = &mut parent.children.as_mut().unwrap()[parent_idx - 1];
            let new_mid_member = left_sibling.members.pop().unwrap();
            let new_member = replace(&mut parent.members[parent_idx -1], new_mid_member);
            self.members.insert(0, new_member);

            if let Some(ref mut left_children) = left_sibling.children {
                let mut child = left_children.pop().unwrap();
                child.parent = Some((ptr::from_mut(self), 0));

                self.children.as_mut().unwrap().iter_mut().for_each(|item| item.parent.as_mut().unwrap().1 += 1);
                self.children.as_mut().unwrap().push(child);
            }
        }
    }

    /// 合并同级两个兄弟节点, 把第二个参数指向的节点和两节点之间的成员合并到第一个参数指向的节点, 并返回第一个节点.
    fn merge(left_node: &mut Box<Self>, mid_member:(K,V), right_node: Box<Self>)
    {
        todo!()
    }
}