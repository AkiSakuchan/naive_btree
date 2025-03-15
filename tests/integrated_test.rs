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