use crate::error::Result;
use crate::types::{MeshCode, MeshLevel};

/// メッシュコードの親メッシュを取得する
///
/// 例えば、3次メッシュの親は2次メッシュになります。
/// 1次メッシュの場合は親が存在しないためNoneを返します。
///
/// # 引数
/// * `mesh` - 対象のメッシュコード
///
/// # 戻り値
/// 親メッシュコード、または親が存在しない場合はNone
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("53394611").unwrap();
/// let parent_mesh = parent(mesh).unwrap();
/// assert_eq!(parent_mesh.as_string(), "533946");
/// ```
pub fn parent(mesh: MeshCode) -> Option<MeshCode> {
    let level = mesh.level();
    let parent_level = level.parent()?;

    let code_str = mesh.as_string();
    let parent_len = parent_level.code_length();
    let parent_code_str = &code_str[0..parent_len];

    MeshCode::from_str(parent_code_str).ok()
}

/// メッシュコードの子メッシュをすべて取得する
///
/// 1次メッシュは64個の2次メッシュを、2次メッシュは100個の3次メッシュを、
/// 3次メッシュは4個の4次メッシュ（2分の1）を子として持ちます。
///
/// # 引数
/// * `mesh` - 対象のメッシュコード
///
/// # 戻り値
/// 子メッシュコードのベクター
///
/// # 例
///
/// ```
/// use jismeshcode::prelude::*;
///
/// let mesh = MeshCode::from_str("533946").unwrap();
/// let children_list = children(mesh);
/// assert_eq!(children_list.len(), 100); // 2次メッシュは100個の3次メッシュを持つ
/// ```
pub fn children(mesh: MeshCode) -> Vec<MeshCode> {
    let level = mesh.level();
    let code_str = mesh.as_string();

    match level {
        MeshLevel::First => {
            let mut result = Vec::with_capacity(64);
            for t in 0..8 {
                for u in 0..8 {
                    let child_str = format!("{}{}{}", code_str, t, u);
                    if let Ok(child) = MeshCode::from_str(&child_str) {
                        result.push(child);
                    }
                }
            }
            result
        }
        MeshLevel::Second => {
            let mut result = Vec::with_capacity(100);
            for v in 0..10 {
                for w in 0..10 {
                    let child_str = format!("{}{}{}", code_str, v, w);
                    if let Ok(child) = MeshCode::from_str(&child_str) {
                        result.push(child);
                    }
                }
            }
            result
        }
        MeshLevel::Third => {
            let mut result = Vec::with_capacity(4);
            for i in 1..=4 {
                let child_str = format!("{}{}", code_str, i);
                if let Ok(child) = MeshCode::from_str(&child_str) {
                    result.push(child);
                }
            }
            result
        }
        _ => Vec::new(),
    }
}

pub fn to_level(mesh: MeshCode, target_level: MeshLevel) -> Result<MeshCode> {
    let current_level = mesh.level();

    if current_level == target_level {
        return Ok(mesh);
    }

    if current_level < target_level {
        return Err(crate::error::MeshCodeError::InvalidFormat(
            "Cannot convert to finer level without additional information".to_string(),
        ));
    }

    let code_str = mesh.as_string();
    let target_len = target_level.code_length();

    if code_str.len() < target_len {
        return Err(crate::error::MeshCodeError::InvalidFormat(
            "Invalid level conversion".to_string(),
        ));
    }

    let target_code_str = &code_str[0..target_len];
    MeshCode::from_str(target_code_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let parent_mesh = parent(mesh).unwrap();
        assert_eq!(parent_mesh.as_string(), "533935");
        assert_eq!(parent_mesh.level(), MeshLevel::Second);

        let parent_mesh = parent(parent_mesh).unwrap();
        assert_eq!(parent_mesh.as_string(), "5339");
        assert_eq!(parent_mesh.level(), MeshLevel::First);

        assert!(parent(parent_mesh).is_none());
    }

    #[test]
    fn test_children_first() {
        let mesh = MeshCode::from_str("5339").unwrap();
        let children_list = children(mesh);
        assert_eq!(children_list.len(), 64);
        assert!(children_list.iter().all(|c| c.level() == MeshLevel::Second));
    }

    #[test]
    fn test_children_second() {
        let mesh = MeshCode::from_str("533935").unwrap();
        let children_list = children(mesh);
        assert_eq!(children_list.len(), 100);
        assert!(children_list.iter().all(|c| c.level() == MeshLevel::Third));
    }

    #[test]
    fn test_to_level() {
        let mesh = MeshCode::from_str("53393599").unwrap();
        let second = to_level(mesh, MeshLevel::Second).unwrap();
        assert_eq!(second.as_string(), "533935");

        let first = to_level(mesh, MeshLevel::First).unwrap();
        assert_eq!(first.as_string(), "5339");
    }
}
