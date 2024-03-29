use bitcoin::hashes::sha256;
use secp256k1::hashes::Hash;
use tauri::State;

use crate::db::PoolWrapper;

/// Save password
/// - Save user password to database
///
/// * `password`: User password
#[tauri::command]
pub async fn save_password(pool: State<'_, PoolWrapper>, password: &str) -> Result<(), String> {
    let hash = sha256::Hash::hash(&password.as_bytes());
    pool.set_password(&hash.to_string())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn save_room_id(state: State<'_, PoolWrapper>, room_id: String) {
    // let _ = state
    //     .sled
    //     .insert(b"roomID", bincode::serialize(&room_id).unwrap())
    //     .expect("Cannot insert room id");
}

#[tauri::command]
pub async fn signin(state: State<'_, PoolWrapper>, password: &str) -> Result<bool, String> {
    let hash = sha256::Hash::hash(&password.as_bytes());
    let pw = state
        .get_password()
        .await
        .map_err(|_| "Cannot get password".to_string())?;

    match pw {
        Some(pw) => Ok(hash.to_string() == pw),
        None => Err("Password not found".to_string()),
    }
}
