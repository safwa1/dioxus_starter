use lazy_static::lazy_static;

pub struct AppName; 


impl AppName {
    pub fn get_formatted_value() -> &'static str {
        &FORMATTED_APP_NAME
    }

    pub fn get_value() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}

lazy_static! {
    static ref FORMATTED_APP_NAME: String = {
        let name = env!("CARGO_PKG_NAME");
        format_app_name(name)
    };
}

fn format_app_name(name: &str) -> String {
    let temp_name = name.replace('_', "-");

    temp_name.split('-')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => {
                    first_char.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}