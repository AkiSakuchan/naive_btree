use naive_btree::*;

const DATA :[(i32,i32); 24] = [(1, 8), (4, 9), (6, 2), (8, 10), (11, 11), (13, 3), (14, 12), (16, 13),
                                (17, 1), (19, 14), (22, 15), (23, 4), (27, 16), (34, 17), (35, 5), 
                                (38, 18), (45, 19), (47, 6), (49, 20), (53, 21), (55, 7), (65, 22), (74, 23), (79, 24)];

fn init_test() -> Btree<i32, i32>
{

let mut btree = Btree::new();

DATA.iter().for_each(|(a,b)| {btree.insert(*a, *b);} );

btree
}

#[test]
fn read_works()
{
let btree_example = init_test();

DATA.iter().for_each(|(a,b)| assert_eq!(btree_example.get(a).unwrap(), b));

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
        println!("({}, {}),", it.0, it.1);
    }
}