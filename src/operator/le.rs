/// 运算符 `le` 的 trait 和相关实现。
use crate::matches::{GetSingleValue, Values};
use crate::result::Result;

pub trait LeOperator<T> {
    fn le_ope(&self, target: T) -> Result<bool>;
}
pub trait LeOperatorForContentLen<T> {
    fn le_ope_for_content_len(&self, target: T) -> Result<bool>;
}

impl LeOperator<&Values> for i64 {
    fn le_ope(&self, target: &Values) -> Result<bool> {
        Ok(*self <= target.get_an_integer()?)
    }
}

impl LeOperator<&Values> for i32 {
    fn le_ope(&self, target: &Values) -> Result<bool> {
        (*self as i64).le_ope(target)
    }
}

impl LeOperator<&Values> for Option<i32> {
    fn le_ope(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.le_ope(target)
        } else {
            Ok(false)
        }
    }
}

impl LeOperator<&Values> for f64 {
    fn le_ope(&self, target: &Values) -> Result<bool> {
        Ok(*self <= target.get_a_decimal()?)
    }
}

impl LeOperatorForContentLen<&Values> for String {
    fn le_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        let self_len = self.chars().collect::<Vec<_>>().len() as i64;

        Ok(self_len <= target.get_an_integer()?)
    }
}

impl LeOperatorForContentLen<&Values> for Option<String> {
    fn le_ope_for_content_len(&self, target: &Values) -> Result<bool> {
        if let Some(self_data) = self {
            self_data.le_ope_for_content_len(target)
        } else {
            Ok(false)
        }
    }
}
