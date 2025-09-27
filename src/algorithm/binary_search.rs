
// 假设输入的数据是正向排序的
pub fn index_of(v: &[i64], tar: i64) -> isize {
    assert!(v.len() >= 1, "数组长度必须大于等于1");
    let (mut i, mut j) = (0, v.len() - 1);
    let mut m;
    loop {
        // m = (i + j) / 2;   如果是除的话 m （i + j）可能会因为超出范围而导致m为负值
        m = (i + j) >> 1;
        if i > j {
            return -1;
        }
        if v[m] < tar {
            i = m + 1;
        } else if v[m] > tar {
            j = m - 1;
        } else {
            return m as isize;
        }
    }
}

pub fn index_of_advanced(v: &[i64], tar: i64) -> isize {
    assert!(v.len() >= 1, "数组长度必须大于等于1");
    let (mut i, mut j) = (0, v.len());
    let mut m = (i + j) >> 1;
    while j - i > 1 {
        if v[m] > tar {
            j = m;
        } else {
            i = m;
        }
        m = (i + j) >> 1;
    }
    if tar == v[i] {
        return i as isize;
    } else {
        return -1;
    }
}



pub unsafe fn index_of_advanced_unchecked(v: &[i64], from_index: usize, end_index: usize, tar: i64) -> isize {
    let (mut low, mut high) = (from_index, end_index + 1);
    while high - low > 1 { // 如果两个指针之间还有值就继续判断
        let mid = (low + high) >> 1;
        if v[mid] > tar {
            high = mid;
        } else {
            low = mid;
        }
    }
    if v[low] == tar {
        return low as isize;
    } else {
        return -((low + 1) as isize);
    }
}

pub enum SearchMod {
    LeftMost,
    RightMost,
    Common,
}

pub unsafe fn index_of_left_right_most_advanced_unchecked(v: &[i64], from_index: usize, end_index: usize, tar: i64, smod: SearchMod) -> isize {
    match smod {
        SearchMod::Common => {
            unsafe {
                index_of_advanced_unchecked(v, from_index, end_index, tar)   
            }
        },
        SearchMod::LeftMost => {
            let (mut low, mut high) = (from_index, end_index + 1);
            while high - low > 1 { // 如果两个指针之间还有值就继续判断
                let mid = (low + high) >> 1;
                if v[mid] > tar {
                    high = mid;
                } else {
                    low = mid;
                }
            }
            if v[low] == tar {
                loop {
                    if low == from_index {
                        return low as isize;
                    } else {
                        low -= 1;
                        if v[low] != tar {
                            return low as isize + 1;
                        }
                    }
                }
            } else {
                return -((low + 1) as isize);
            }
        },
        SearchMod::RightMost => {
            let (mut low, mut high) = (from_index, end_index + 1);
            while high - low > 1 { // 如果两个指针之间还有值就继续判断
                let mid = (low + high) >> 1;
                if v[mid] > tar {
                    high = mid;
                } else {
                    low = mid;
                }
            }
            if v[low] == tar {
                loop {
                    if low == end_index {
                        return low as isize;
                    } else {
                        low += 1;
                        if v[low] != tar {
                            return low as isize - 1;
                        }
                    }
                }
            } else {
                return -((low + 1) as isize);
            }
        }
    }
}

pub fn index_of_advanced00(v: &[i64], from_index: usize, end_index: usize, tar: i64) -> isize {
    assert!(v.len() >= 1, "数组长度必须大于等于1");
    assert!(end_index <= v.len() - 1, "结束索引必须小于数组的长度减1");
    unsafe {index_of_advanced_unchecked(v, from_index, end_index, tar)}
}

pub fn index_of_left_right_most_advanced00(v: &[i64], from_index: usize, end_index: usize, tar: i64, smod: SearchMod) -> isize {
    assert!(v.len() >= 1, "数组长度必须大于等于1");
    assert!(end_index <= v.len() - 1, "结束索引必须小于数组的长度减1");
    unsafe {index_of_left_right_most_advanced_unchecked(v, from_index, end_index, tar, smod)}
}

pub fn index_of_advanced0(v: &[i64], tar: i64) -> isize {
    index_of_advanced00(v, 0, v.len() - 1, tar)
}