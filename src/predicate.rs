
pub fn is_point(c: char) -> bool {
    c == '.'
}

pub fn is_delimiter(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t' || c == '\r'
    || c == '(' || c == ')' || c == '[' || c == ']'
}

pub fn is_underscore(c: char) -> bool {
    c == '_'
}

pub fn is_op(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/'
}