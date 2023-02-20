// match 语句和枚举
// match语句可用于比较存储在一个枚举值
//以下示例定义了一个函数print_size，它将CarType枚举作为参数
//该函数将参数值与一组预定义的常量进行比较，并显示相应的消息。

enum CarType {
    Hatch,
    Sedan,
    SUV,
}
fn print_size(car: CarType) {
    match car {
        CarType::Hatch => {
            println!("Small sized car");
        }
        CarType::Sedan => {
            println!("medium sized car");
        }
        CarType::SUV => {
            println!("Large sized Sports Utility car");
        }
    }
}
fn main() {
    print_size(CarType::SUV);
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
}
