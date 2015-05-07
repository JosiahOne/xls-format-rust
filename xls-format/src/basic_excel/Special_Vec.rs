// Special_Vec. Methods for adding to the Vec type.

pub fn resize_vec<T>(a: &mut Vec<T>, new_size: usize, type_value: T) {
    if a.len() == new_size {
        return;
    } else if a.len() < new_size {
        let elements_to_add = new_size - a.len();
        for _ in 0..elements_to_add {
            a.push(type_value);
        }
    } else {
        let elements_to_remove = a.len() - new_size;
        for _ in 0..elements_to_remove {
            a.pop();
        }
    }
}