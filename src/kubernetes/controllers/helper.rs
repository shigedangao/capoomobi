/// Kube Enum Helper
/// 
/// # Description
/// Trait use on controller's enum
pub trait KubeEnumHelper<T> {
    /// From Str
    /// 
    /// # Arguments
    /// * `value` slice of string
    /// 
    /// # Return
    /// * `option` T
    fn from_str(value: &str) -> Option<T>;
}