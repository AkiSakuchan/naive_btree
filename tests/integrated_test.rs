use naive_btree::*;

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

assert_eq!(*btree_example.get(&17).unwrap(), 1);
assert_eq!(*btree_example.get(&6).unwrap(), 2);
assert_eq!(*btree_example.get(&13).unwrap(), 3);
assert_eq!(*btree_example.get(&23).unwrap(), 4);
assert_eq!(*btree_example.get(&35).unwrap(), 5);
assert_eq!(*btree_example.get(&47).unwrap(), 6);
assert_eq!(*btree_example.get(&55).unwrap(), 7);
assert_eq!(*btree_example.get(&1).unwrap(), 8);
assert_eq!(*btree_example.get(&4).unwrap(), 9);
assert_eq!(*btree_example.get(&8).unwrap(), 10);
assert_eq!(*btree_example.get(&11).unwrap(), 11);
assert_eq!(*btree_example.get(&14).unwrap(), 12);
assert_eq!(*btree_example.get(&16).unwrap(), 13);
assert_eq!(*btree_example.get(&19).unwrap(), 14);
assert_eq!(*btree_example.get(&22).unwrap(), 15);
assert_eq!(*btree_example.get(&27).unwrap(), 16);
assert_eq!(*btree_example.get(&34).unwrap(), 17);
assert_eq!(*btree_example.get(&38).unwrap(), 18);
assert_eq!(*btree_example.get(&45).unwrap(), 19);
assert_eq!(*btree_example.get(&49).unwrap(), 20);
assert_eq!(*btree_example.get(&53).unwrap(), 21);
assert_eq!(*btree_example.get(&65).unwrap(), 22);
assert_eq!(*btree_example.get(&74).unwrap(), 23);
assert_eq!(*btree_example.get(&79).unwrap(), 24);

assert!(btree_example.get(&5).is_none(), "找到不存在的值");

}

#[test]
fn write_works()
{
let mut btree_example = init_test();
*btree_example.get_mut(&55).unwrap() = -7;
assert_eq!(*btree_example.get(&55).unwrap(), -7);

*btree_example.get_mut(&45).unwrap() = -19;
assert_eq!(*btree_example.get(&45).unwrap(), -19);

}

#[test]
fn iter_works()
{
    let btree0: Btree<i32,i32> = Btree::new();
    let mut iter0 = btree0.iter();
    assert_eq!(iter0.next(), None);
    assert_eq!(iter0.next(), None);

    let btree = init_test();
    for it in btree.iter() {
        println!("({}, {})", it.0, it.1);
    }
}