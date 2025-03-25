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

#[test]
fn iter_mut_works()
{
    let mut btree = init_test();
    btree.iter_mut().for_each(|(_,v)| *v = -(*v));

    DATA.iter().zip(btree.iter()).for_each(|((_, v1), (_, v2))| assert_eq!(*v1, -(*v2)));
}

#[test]
fn remove_work()
{
    let mut btree = init_test();

    for (key, value) in DATA {
        let (rem_key, rem_value) = btree.remove(&key).expect("移除后不能传递出被移除值");

        assert_eq!(key, rem_key);
        assert_eq!(value, rem_value);

        assert!(btree.get(&key).is_none(), "移除值之后依然能找到");
    }
}

#[test]
fn reverse_remove_work()
{
    let mut btree = init_test();
    DATA.iter().rev().for_each(|(key,value)| {
        let (rem_key, rem_value) = btree.remove(key).expect("移除后不能传递出值");

        assert_eq!(*key, rem_key);
        assert_eq!(*value, rem_value);

        assert!(btree.get(key).is_none(), "移除值后依然能找到");
    }); 
}

#[test]
fn index_work()
{
    let btree = init_test();

    DATA.iter().for_each(|&(key,value)| assert_eq!(value, btree[key]));

    let mut btree2 = init_test();

    for (k,v) in DATA.into_iter() {
        btree2[k] += 1;
        assert_eq!(btree2[k], v + 1);
    }
}

#[test]
#[should_panic]
fn index_nonexist()
{
    let mut btree = init_test();
    btree[20] = 5;
}