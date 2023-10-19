/// **Gnome sort** is an insertion sort variant that has no inner loop.
///
/// https://en.wikipedia.org/wiki/Gnome_sort
pub fn gnome_sort<T: Ord>(v: &mut [T]) {
    let mut i = 0;
    while i < v.len() {
        if i == 0 || v[i] >= v[i - 1] {
            i += 1;
        } else {
            v.swap(i, i - 1);
            i -= 1;
        }
    }
}

/// **Bubble sort** repeatly swaps elements from left to right until
/// the largest element is at its position.  This version detects
/// the position of the last swap, that marks the "already sorted"
/// region, to avoid unnecessary work on next iterations.
pub fn bubble_sort<T: Ord>(v: &mut [T]) {
    let mut n = v.len();
    while n > 0 {
        let mut nmax = 0;
        let mut i = 1;
        while i < n {
            if v[i - 1] > v[i] {
                v.swap(i - 1, i);
                nmax = i;
            }
            i += 1;
        }
        n = nmax;
    }
}

/// **Insertion sort** splits the vector on an "already sorted" region,
/// initially with only the leftmost element, and a "not sorted" region.
/// Elements are inserted, one by one, from left to right, from the
/// "not sorted" region into the "already sorted" region.
pub fn insertion_sort<T: Ord>(v: &mut [T]) {
    // initially "already sorted" has `1` element, and iterate until we
    // have `v.len()` elements.
    for i in 1..v.len() {
        // `i` is the first not sorted, `j` will be where it should go
        // move left until find the first element larger than the one at `i`
        let mut j = i;
        while j > 0 && v[j - 1] > v[i] {
            j -= 1;
        }

        // this rotate takes the element at `i` (the first of "not sorted"),
        // puts it at `j` (its final location, found on the loop above),
        // shifting all elements in range as necessary.
        v[j..=i].rotate_right(1);
    }
}

/// **Shell sort** is a variant of insertion sort that moves elements further
/// away, reducing the distance in each iteraction.
pub fn shell_sort<T: Ord + Copy>(v: &mut [T]) {
    // find the distance between elements
    let mut h = 1;
    while h <= v.len() / 9 {
        h = 3 * h + 1;
    }

    while h > 0 {
        // for each distance `h`, runs an insertion sort, but
        // compare `v[i]` with `v[i - h]` (instead of with `v[i - 1]`)
        let mut i = h;
        while i < v.len() {
            let mut j = i;
            let a = v[i];
            while j >= h && v[j - h] > a {
                v[j] = v[j - h];
                j -= h;
            }
            v[j] = a;
            i += 1;
        }

        h /= 3;
    }
}

/// **Selection sort** is a more direct implementation of the "find the
/// smallest element and put on the start" idea: from left to right
/// scan the array for the smallest element on the "not sorted"
/// region and swap it with the first of the "not sorted", thus growing
/// the "already sorted" region by one.
pub fn selection_sort<T: Ord>(v: &mut [T]) {
    for i in 0..v.len() - 1 {
        // find the smallest element on `v[i+1..]` and swap with the one at `v[i]`.
        let mut min = i;
        let mut min_value = &v[i];
        for j in i + 1..v.len() {
            if v[j] < *min_value {
                min = j;
                min_value = &v[j];
            }
        }
        v.swap(i, min);
    }
}

/// **Three-way Quicksort with random pivot**, recurse only on smallest partition
/// and insertion sort on small sub-arrays.
/// Does way better than binary Quicksort with many equal elements.
pub fn quick_sort_3<T: Ord>(mut v: &mut [T]) {
    fn choose_pivot<T: Ord>(v: &[T]) -> usize {
        fastrand::usize(..v.len())
    }

    fn partition<T: Ord>(v: &mut [T]) -> (usize, usize) {
        let mut mid1 = 1;
        let mut mid2 = 1;
        let mut j = 1;
        while j < v.len() {
            if v[j] < v[0] {
                v.swap(mid2, j);
                v.swap(mid2, mid1);
                mid1 += 1;
                mid2 += 1;
            } else if v[j] == v[0] {
                v.swap(mid2, j);
                mid2 += 1;
            }
            j += 1;
        }
        v.swap(mid1 - 1, 0);
        (mid1 - 1, mid2)
    }

    while v.len() > 30 {
        let pivot = choose_pivot(v);
        v.swap(pivot, 0);

        let (mid1, mid2) = partition(v);
        if mid1 < v.len() - mid2 {
            quick_sort_3(&mut v[..mid1]);
            v = &mut v[mid2..];
        } else {
            quick_sort_3(&mut v[mid2..]);
            v = &mut v[..mid1];
        }
    }

    insertion_sort(v);
}

/// **Binary Quicksort with random pivot**, recurse only on smallest partition
/// and insertion sort on small sub-arrays.
pub fn quick_sort<T: Ord>(mut v: &mut [T]) {
    fn choose_pivot<T: Ord>(v: &[T]) -> usize {
        fastrand::usize(..v.len())
    }

    fn partition<T: Ord>(v: &mut [T]) -> usize {
        let mut i = 1;
        let mut j = 1;
        while j < v.len() {
            if v[j] < v[0] {
                v.swap(i, j);
                i += 1;
            }
            j += 1;
        }
        v.swap(i - 1, 0);
        i - 1
    }

    while v.len() > 30 {
        let pivot = choose_pivot(v);
        v.swap(pivot, 0);

        let mid = partition(v);
        let n = v.len();
        if mid < n - mid {
            quick_sort(&mut v[..mid]);
            if mid < n {
                v = &mut v[mid + 1..];
            } else {
                break;
            }
        } else {
            if mid < n {
                quick_sort(&mut v[mid + 1..]);
            }
            v = &mut v[..mid];
        }
    }

    insertion_sort(v);
}

/// Sort by converting the vector into a heap and repeatedly removing the largest element.
pub fn heap_sort<T: Ord>(v: &mut [T]) {
    // move the element at `v[start]` down, swapping with the smallest children,
    // as much as possible, to find its final position in the heap.
    fn sift_down<T: Ord>(v: &mut [T], start: usize) {
        let mut i = start;
        loop {
            let mut child = i * 2 + 1;
            if child >= v.len() {
                break;
            } else if child + 1 < v.len() && v[child + 1] > v[child] {
                child += 1;
            }

            if v[i] < v[child] {
                v.swap(i, child);
                i = child;
            } else {
                break;
            }
        }
    }

    // transform `v` into a heap with largest element on `v[0]`
    for i in (0..=v.len() / 2).rev() {
        sift_down(v, i);
    }

    // iterating from the last element to the first, swap the
    // largest `v[0]` element with it and rebuild the heap state.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

fn merge<T: Ord + Clone>(w: &[T], half: usize, v: &mut [T]) {
    let mut i = 0;
    let mut j = half;
    for k in 0..v.len() {
        if i < half && (j >= w.len() || w[i] <= w[j]) {
            v[k] = w[i].clone();
            i += 1;
        } else {
            v[k] = w[j].clone();
            j += 1;
        }
    }
}

pub fn merge_sort_top_down<T: Ord + Clone>(v: &mut [T]) {
    fn split_merge<T: Ord + Clone>(w: &mut [T], v: &mut [T]) {
        if w.len() > 1 {
            let half = w.len() / 2;
            split_merge(&mut v[..half], &mut w[..half]);
            split_merge(&mut v[half..], &mut w[half..]);
            merge(w, half, v);
        }
    }

    let mut w: Vec<_> = v.iter().cloned().collect();
    split_merge(&mut w, v);
}

pub fn merge_sort_bottom_up<T: Ord + Clone>(v: &mut [T]) {
    let mut w: Vec<_> = v.iter().cloned().collect();

    let n = v.len();
    let mut v_to_w = true;

    let mut width = 1;
    while width < n {
        // for i in (0..n).skip(2 * width) {
        // }
        let mut i = 0;
        while i < n {
            let end = (i + 2 * width).min(n);
            if v_to_w {
                merge(&v[i..end], width, &mut w[i..end]);
            } else {
                merge(&w[i..end], width, &mut v[i..end]);
            }
            i += 2 * width;
        }

        v_to_w = !v_to_w;
        width *= 2;
    }

    if !v_to_w {
        v.clone_from_slice(&w);
    }
}

pub fn native_sort<T: Ord>(v: &mut [T]) {
    v.sort();
}

pub fn native_unstable_sort<T: Ord>(v: &mut [T]) {
    v.sort_unstable();
}
