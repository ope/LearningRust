/// 引数に渡された2つの値を合算した値を取得します。
///
/// # Arguments
///
/// * `x` - 合算する1つ目の値。
/// * `y` - 合算する2つ目の値。
///
/// # Examples
///
/// ```
/// let summed: i32 = doclib::add(10, 20);
///
/// assert_eq!(summed, 30);
/// ```
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/