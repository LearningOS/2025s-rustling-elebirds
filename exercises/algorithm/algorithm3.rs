/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: std::cmp::PartialOrd>(array: &mut [T]) -> usize { // 分区函数
    let len = array.len();
    let pivot = len / 2; // 选择基准元素，这里选择中间
    array.swap(pivot, len - 1); // 将基准元素放到最后
    let mut i = 0; // i 用来记录小于基准元素的位置
    for j in 0..len - 1 { // 遍历除基准元素外的所有元素
        if &array[j] <= &array[len - 1] { // 如果当前元素小于基准元素
            array.swap(i, j); // 交换 i 和 j 位置的元素，使得小于基准元素的元素都在左边
            i += 1;
        }
    }
    array.swap(i, len - 1); // 将基准元素放到正确的位置，使得左边的元素都小于基准元素，右边的元素都大于基准元素
    i // 返回基准元素的位置
}

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    let len = array.len();
    if len < 2 {
        return;
    }
    let pivot = partition(array);
    sort(&mut array[0..pivot]);
    sort(&mut array[pivot + 1..len]);
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