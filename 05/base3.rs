/*
 HashSet 是一组 T 类型的唯一值。
 添加和删除值很快，询问给定值是否在集合中也很快。
 HashSet 结构在 std::collections 模块中定义。应显式导入此模块以访问 HashSet 结构。
 语法：创建一个 HashSet
 let mut hash_set_name = HashSet::new();
*/

/*
 1	insert()
 pub fn insert(&mut self, value: T) -> bool
 向集合中添加一个值。如果该集合不存在此值，则返回 true，否则返回 false。
 2	len()
 pub fn len(&self) -> usize
 返回集合中元素的数量。
 3	get()
 pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow,Q: Hash + Eq,
 返回对集合中值的引用，如果有任何等于给定值。
 4	iter()
 pub fn iter(&self) -> Iter
 返回以任意顺序访问所有元素的迭代器。迭代器元素类型是 &'a T。
 5	contains_key
 pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
 如果集合包含一个值，则返回 true。
 6	remove()
 pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool
 从集合中删除一个值。如果该值存在于集合中，则返回 true。
*/
