use tauri::State;

use crate::db::PoolWrapper;

/// Save password
/// - Save user password to database
///
/// * `password`: User password
#[tauri::command]
pub async fn save_password(pool: State<'_, PoolWrapper>, password: String) -> Result<(), String> {
    let _ = pool
        .sled
        .insert(b"password", bincode::serialize(&password).unwrap())
        .expect("Cannot insert password");

    pool.set_password(&password)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn save_room_id(state: State<'_, PoolWrapper>, room_id: String) {
    let _ = state
        .sled
        .insert(b"roomID", bincode::serialize(&room_id).unwrap())
        .expect("Cannot insert room id");
}
