use tauri::State;

use crate::db::PoolWrapper;

/// Save password
/// - Save user password to database
///
/// * `password`: User password
#[tauri::command]
pub fn save_password(state: State<'_, PoolWrapper>, password: String) {
    let _ = state
        .pool
        .insert(b"password", bincode::serialize(&password).unwrap())
        .expect("Cannot insert password");
}

#[tauri::command]
pub fn save_room_id(state: State<'_, PoolWrapper>, room_id: String) {
    let _ = state
        .pool
        .insert(b"roomID", bincode::serialize(&room_id).unwrap())
        .expect("Cannot insert room id");
}
