#[macro_export]
macro_rules! cn {
    ($($class:expr),*) => {{
        // Count non-empty classes first to estimate size
        let mut count = 0;
        let mut total_len = 0;
        $(
            if !$class.is_empty() {
                count += 1;
                total_len += $class.len();
            }
        )*
        
        // Add space for spaces between classes
        total_len += if count > 0 { count - 1 } else { 0 };
        
        let mut result = String::with_capacity(total_len);
        let mut first = true;
        
        $(
            if !$class.is_empty() {
                if !first {
                    result.push(' ');
                }
                first = false;
                //println!("{first}");
                result.push_str($class);
            }
        )*
        
        result
    }};
}

#[macro_export]
macro_rules! merge_styles {
    ($($style:expr),*) => {{
        use std::collections::HashMap;
        
        let mut result = HashMap::new();
        
        $(
            if let Some(style_str) = $style {
                for pair in style_str.split(';') {
                    let trimmed = pair.trim();
                    if !trimmed.is_empty() {
                        if let Some((key, value)) = trimmed.split_once(':') {
                            result.insert(key.trim().to_string(), value.trim().to_string());
                        }
                    }
                }
            }
        )*
        
        if result.is_empty() {
            None
        } else {
            Some(
                result.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        }
    }};
}