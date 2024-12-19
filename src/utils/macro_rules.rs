#[macro_export]
macro_rules! add_field {
    ($query:ident, $query_params:ident, $param_index:ident, $field:expr, $value:expr) => {
        if let Some(value) = $value {
            if !$query_params.is_empty() {
                $query.push_str(", ");
            }
            $query.push_str($field);
            $query.push_str(&format!(" = ${}", $param_index));
            $query_params.push(value);
            $param_index += 1;
        }
    };
}
