use rand;

fn main() {
    println!("{}", ZundokoKiyoshi::build_str());
}

enum Zundoko {
    Zun,
    Doko,
}

impl ToString for Zundoko {
    fn to_string(&self) -> String {
        match self {
            Self::Zun => "ズン".to_string(),
            Self::Doko => "ドコ".to_string(),
        }
    }
}

struct ZundokoKiyoshi;

impl ZundokoKiyoshi {
    fn zundoko() -> Zundoko {
        match rand::random() {
            true => Zundoko::Zun,
            false => Zundoko::Doko,
        }
    }

    fn kiyoshi() -> String {
        "\nキ・ヨ・シ！".to_string()
    }

    fn build_str() -> String {
        let mut result = String::new();
        let mut cnt = 0;

        loop {
            let zundoko = Self::zundoko();
            result += &zundoko.to_string();

            match zundoko {
                Zundoko::Zun => cnt += 1,
                Zundoko::Doko if cnt < 4 => cnt = 0,
                _ => {
                    result += &Self::kiyoshi();
                    break;
                }
            }
        }

        result
    }
}
