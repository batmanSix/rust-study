// 嵌套模块
pub mod movies {
    pub mod english {
        pub mod comedy {
            pub fn play(name: String) {
                println!("play come movie {}", name)
            }
        }
    }
}

use movies::english::comedy::play;
// 倒入模块
fn main() {
    play("Herold and Kumar".to_string());
    play("The Hangover".to_string());

    movies::english::comedy::play("Airplane!".to_string());
}
