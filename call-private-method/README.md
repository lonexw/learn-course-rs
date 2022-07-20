以这个场景为例:

> 举个例子：假设有一个包，公开函数是接受两个地名，然后返回两个地名之间的直线距离。具体实现是：公开函数接受两个地名，然后调用一个私有函数将两个地名转换为经纬度，然后调用另一个私有函数计算两个经纬度之间的距离。
> 如何去直接调用这个私有函数

简单实现代码：

```rust
pub mod map {
    pub fn calc_distance(_start_place: &str, _end_place: &str) -> f32 {
        // TODO 转换地名为经纬度

        // 方便演示就不传参数啦
        inner_calc_distance()
    }

    fn inner_calc_distance() -> f32 {
        println!("Call inner private function!");

        0.0
    }
}
```

我们用下面的黑科技去直接调用包的内部私有函数：

```rust
fn main() {
    unsafe {
        std::mem::transmute::<usize, fn()>(
            // 这里就是魔法 🪄 发生的地方
            (map::calc_distance("北极", "南极") as usize) + 4314503456
        );
    }
}

# 程序运行打印结果：
Running `target/debug/call-private-method`
Call inner private function!
```

是不是很好奇，transmute 的解释，看一下作者的文章：类型转换, 但是这串数字 4314503456 是什么鬼 🐢，解释一下：

1. 我们首先将公开的方法 calc_distance 转换成了 usize 类型；
2. 然后加上这串神秘数字，函数在内存中调用这个内部函数的位置；
3. 再通过 std::mem::transmute 将得到的数字转换为函数执行；

逻辑不复杂，但是这串数字怎么获取？我们需要临时修改一下包的代码，把内部方法先改为 pub , 再用 dbg! 宏打印计算结果就可以拿到（参数变化也会影响内存位置的结果）：

```rust
    pub fn inner_calc_distance() -> f32 {
        println!("Call inner private function!");

        0.0
    }

fn main() {
    dbg!((map::inner_calc_distance as usize) - (map::calc_distance("北极", "南极") as usize));
}
```

最后，这种技巧临时 hack 可以，项目慎用。