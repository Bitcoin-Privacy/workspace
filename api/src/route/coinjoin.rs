use crate::controller::coinjoin;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    /* Input:
     * - Utxos and proofs
     * - Blinded output
     * - Change output
     * Output:
     * - Signed blinded output
     * - Room id
     */
    cfg.route("/register", web::post().to(coinjoin::register));

    /* Input:
     * - Room id
     * - Output address - plain text
     * - Signed output address - unblinded
     * Output:
     * - Room - current state
     */
    cfg.route("/output", web::post().to(coinjoin::set_output));

    /* Input:
     * - Room id
     * - Transaction (hex - string)
     * Output:
     * - Room - current state
     */
    cfg.route("/sign", web::post().to(coinjoin::set_signature));

    /* Input:
     * - Room id
     * Output:
     * - Room[]
     */
    cfg.route("/room/list", web::get().to(coinjoin::get_room_list));

    /* Input:
     * - Room id
     * Output:
     * - Room
     */
    cfg.route("/room/{id}", web::get().to(coinjoin::get_room_by_id));

    /* Input:
     * - Room id
     * Output:
     * - Room status ({endtime - read..})
     */
    cfg.route("/room/{id}/status", web::get().to(coinjoin::get_status));

    /* Input:
     * - Room id
     * Output:
     * - Transaction (hex - string)
     */
    cfg.route("/room/{id}/txn", web::get().to(coinjoin::get_txn));
}
