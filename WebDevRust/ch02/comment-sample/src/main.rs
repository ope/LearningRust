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
/// let summed: i32 = comment-sample::add(10, 20);
///
/// assert_eq!(summed, 30);
/// ```
pub fn add(x: i32, y: i32) -> i32{
    x + y
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        assert_eq!(4, add(2, 2));
    }
}*/