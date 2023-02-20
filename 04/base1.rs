// use public_module_name::function_name; use 关键字使用公共模块
// pub mod movies {
//     pub fn play(name: String) {
//         println!("Playing movie {}", name);
//     }
// }
// fn main() {
//     movies::play("Herold and Kumar".to_string());
// }

pub mod movies {
    pub fn play(name: String) {
        println!("Playing movie {}", name);
    }
}
use movies::play;
fn main() {
    play("Herold and Kumar ".to_string());
}
