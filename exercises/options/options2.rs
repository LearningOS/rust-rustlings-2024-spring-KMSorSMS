// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DON

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        println!("{}", 1);
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        println!("{}", 1);
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 这里第一个integer的类型是通过匹配Some(Option<i8>)拿出来的Option<i8>,第二个是i8
        while let Some(integer) = optional_integers.pop() {
            if let Some(integer) = integer {
                assert_eq!(integer, cursor);
                println!("{:?}", integer);
                println!("{}", cursor);
                cursor -= 1;
            }
        }
        println!("{}", cursor);

        assert_eq!(cursor, 0);
    }
}
