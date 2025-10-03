
// 假设从小到大排列
pub fn index_of(v: &[i64], tar: i64, from_index: usize, end_index: usize) -> isize {
    let (start_idx, end_idx) = (from_index, end_index); // [start_idx, end_idx]
    if start_idx == end_index {
        if v[start_idx] == tar {
            return start_idx as isize;
        } else {
            return -1;
        }
    } else {
        let mid_idx = start_idx + end_idx >> 1;
        if v[mid_idx] < tar {
            return index_of(v, tar, mid_idx + 1, end_index);
        } else if v[mid_idx] > tar {
            return index_of(v, tar, from_index, mid_idx - 1);
        }
        return mid_idx as isize;
    }
}



#[test]
fn test() {
    assert_eq!(index_of(&[1, 2, 3, 5], 5, 0, 3), 3);
    assert_eq!(index_of(&[1, 2, 3, 5, 6], 5, 0, 4), 3);
    assert_eq!(index_of(&[1, 2, 3, 5, 5, 6, 7], 1, 0, 6), 0);
    assert_eq!(index_of(&[1, 2, 3, 5], 2, 0, 3), 1);
}
