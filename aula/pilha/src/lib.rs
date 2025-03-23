pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    if len == 0 {
        return 1; // ou qualquer valor que faça sentido para o seu caso
    }

    let mut product = 1;
    for i in 0..len {
        product *= *ptr.offset(i as isize);
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}

fn main() {
    println!("Hello, world!");
}