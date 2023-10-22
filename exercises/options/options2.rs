// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // 使用if let来检查optional_target是否是Some
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // 使用while let来迭代处理optional_integers中的值
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(cursor));
            cursor -= 1;
        
            if cursor == 0 {
                break; // Exit the loop when cursor reaches 0
            }
        }
        

        assert_eq!(cursor, 0);
    }
}
