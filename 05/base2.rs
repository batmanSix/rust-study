/*
 映射是键值对（称为条目）的集合。映射中的两个条目不能具有相同的键。
 简而言之，map是一个查找表。HashMap 将键和值存储在哈希表中。
 条目以任意顺序存储。该键用于在 HashMap 中搜索值。
 HashMap 结构定义在std::collections模块。应显式导入此模块以访问 HashMap 结构。
 语法：创建 HashMap
*/
/*
  1	insert()
  pub fn insert(&mut self, k: K, v: V) -> Option
  插入一个键/值对，如果没有键，则返回 None 。更新后，返回旧值。
  2	len()
  pub fn len(&self) -> usize
  返回map中元素的数量。
  3	get()
  pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq
  返回对与键对应的值的引用。
  4	iter()
  pub fn iter(&self) -> Iter<K, V>
  以任意顺序访问所有键值对的迭代器。迭代器元素类型是 (&'a K, &'a V)。
  5	contains_key
  pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
  如果映射包含指定键的值，则返回 true。
  6	remove()
  pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
  从map中删除一个键，如果该键以前在map中，则返回存储的键和值。
*/

use std::collections::HashMap;
fn main() {
    let mut codes = HashMap::new();
    codes.insert("KL", "Kerala");
    codes.insert("MH", "Maharashtra");
    println!("{:?}", codes);
    len_func();
    get_key();
    iter_func();
    contains_key_func();
}

fn len_func() {
    let mut codes = HashMap::new();
    codes.insert("KL", "Kerala");
    codes.insert("MH", "Maharashtra");
    println!("size of map is {}", codes.len());
}

fn get_key() {
    let mut code = HashMap::new();
    code.insert("KL", "Kerala");
    code.insert("MH", "Maharashtra");
    println!("size of map is {}", code.len());
    println!("{:?}", code);
    match code.get(&"KL") {
        Some(value) => {
            println!("Value for key KL is value {}", value);
        }
        None => {
            println!("nothing found");
        }
    }
}

fn iter_func() {
    let mut code = HashMap::new();
    code.insert("KL", "Kerala");
    code.insert("MH", "Maharashtra");
    for (key, val) in code.iter() {
        println!("key: {} val: {}", key, val);
    }
}

fn contains_key_func() {
    let mut code = HashMap::new();
    code.insert("KL", "Kerala");
    code.insert("MH", "Maharashtra");
    code.insert("GJ", "Gujarat");
    if code.contains_key(&"GJ") {
        println!("found key");
    }
}
