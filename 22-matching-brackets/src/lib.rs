pub fn brackets_are_balanced(string: &str) -> bool {
    let valid_bracket_touples = [('(',')'), ('[',']'), ('{','}')];
    let mut stack = <Vec<char>>::new();
    for ch in string.chars(){
        match ch{
            '(' | '[' | '{'=>{
                stack.push(ch);
            }
            ')' | ']' | '}'=>{
                let Some(open) = stack.pop() else{
                    return false;
                };
                if !valid_bracket_touples.contains(&(open, ch)){
                    return false
                }

            }
            _=>{}
        }
    }
    stack.is_empty()
}
