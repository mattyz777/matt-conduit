//! 敏感字段 Debug 打印工具
//!
//! 为包含敏感字段的实体提供统一的 Debug 实现

/// 为实体实现自动隐藏敏感字段的 Debug
///
/// # 使用方式
/// ```
/// use crate::impl_redacted_debug;
///
/// #[derive(Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
/// pub struct Model {
///     pub id: i32,
///     pub username: String,
///     #[serde(skip_serializing)]
///     pub password: String,
///     pub age: Option<i32>,
/// }
///
/// impl_redacted_debug!(User, password, [id, username, age]);
/// ```
///
/// // 现在打印时会自动隐藏 password
/// let user = Model { ... };
/// println!("{:?}", user);
/// // 输出: User { id: 1, username: "admin", password: "[REDACTED]", age: Some(25) }
/// ```

#[macro_export]
macro_rules! impl_redacted_debug {
    // 语法： impl_redacted_debug!(EntityName, sensitive_field, [other_fields])
    (
        $entity_name:ident,
        $sensitive_field:ident,
        [$($field:ident),* $(,)?]
    ) => {
        impl std::fmt::Debug for $entity_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($entity_name))
                    .field(stringify!($sensitive_field), &"[REDACTED]")
                    $(
                        .field(stringify!($field), &self.$field)
                    )*
                    .finish()
            }
        }
    };
}
