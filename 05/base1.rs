/*
  Rust 的标准集合库提供了最常见的通用编程数据结构的高效实现
  本章讨论了常用集合的实现——Vector、HashMap 和 HashSet。
  向量 Vector
  Vector 是一个可调整大小的数组。它将值存储在连续的内存块中
  预定义的结构 Vec 可用于创建向量。Vector 的一些重要特征是 -
  Vector 可以在运行时增长或缩小。
  Vector 是同构的集合。
  Vector 将数据存储为按特定顺序排列的元素序列
  Vector 中的每个元素都分配了一个唯一的索引号。
  索引从 0 开始一直到 n-1，其中 n 是集合的大小。
  例如，在 5 个元素的集合中，第一个元素将位于索引 0，最后一个元素将位于索引 4。
  Vector 只会将值附加到（或接近）末尾。换句话说，Vector 可用于实现堆栈。
  Vector 的内存分配在堆中。
  let mut instance_name = Vec::new(); 创建向量
  let vector_name = vec![val1,val2,val3]
*/

/*
  pub fn push(&mut self, value: T)，将元素附加到集合的后面。
  remove  pub fn remove(&mut self, index: usize) -> T 移除并返回向量中位于 index 位置的元素，将其后的所有元素向左移动。
  pub fn contains(&self, x: &T) -> bool 如果切片包含具有给定值的元素，则返回 true。
  pub fn len(&self) -> usize 返回向量中元素的数量，也称为它的“长度”。
*/
fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    println!("size of vector is :{}", v.len());
    println!("{:?}", v);
    remove_func();
    contains_func();
    index_func();
    object_func();
}

fn remove_func() {
    let mut v = vec![10, 20, 30];
    v.remove(1);
    println!("{:?}", v);
}

fn contains_func() {
    let v = vec![10, 20, 30];
    if v.contains(&10) {
        println!("found 10");
    }
    println!("{:?}", v);
}

// 从向量访问值
// 可以使用相应的索引号访问向量中的各个元素。下面的示例创建一个矢量，打印第一个元素的值。
fn index_func() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    println!("{:?}", v[0]);
}

// 也可以使用对集合的引用来获取向量中的值。
fn object_func() {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(500);
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
}
