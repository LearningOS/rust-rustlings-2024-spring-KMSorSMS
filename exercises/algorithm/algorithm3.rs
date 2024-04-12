/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DON

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]) {
    //使用快排算法
    if array.len() == 0 {
        return;
    }
    // 以第一个元素为参考，要求左边都是比他小或者等于，右边都是比他大
    let (mut left_index, mut right_index) = (0usize, (array.len() - 1) as usize);
    loop {
        // 找到第一个大于base_num的最左元素
        while left_index + 1 < array.len() && array[left_index] <= array[0] {
            left_index += 1;
        }
        // 再去找第一个大于base_num的元素
        while array[right_index] > array[0] && right_index >= left_index {
            right_index -= 1;
        }
        // 鉴别索引的有效性，再进行交换操作
        if right_index >= 0 && left_index <= array.len() - 1 && left_index < right_index {
            array.swap(right_index, left_index);
        } else {
            break;
        }
    }
    // 再把出来后的right和[0]做交换
    array.swap(right_index, 0);
    // 分情况左右递归
    sort(&mut array[0..right_index]);
    sort(&mut array[right_index + 1..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
