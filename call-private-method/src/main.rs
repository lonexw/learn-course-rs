pub mod map {
    pub fn calc_distance(_start_place: &str, _end_place: &str) -> f32 {
        // TODO 转换地名为经纬度

        // 方便演示就不传参数啦
        inner_calc_distance()
    }

    pub fn inner_calc_distance() -> f32 {
        println!("Call inner private function!");

        0.0
    }
}

fn main() {
    // map::calc_distance("北极", "南极");
    // dbg!((map::inner_calc_distance as usize) - (map::calc_distance("北极", "南极") as usize));

    unsafe {
        std::mem::transmute::<usize, fn()>(
            (map::calc_distance("北极", "南极") as usize) + 4314503456
        );
    }
}
