frontend-setup:
	@cd front && yarn

fe:
	@cd front && cargo tauri dev

be:
	@cd api/actix && cargo watch -x run -p node-service

start:
	tmux new-session -d -s mySession 'cd front && yarn && cargo tauri dev'
	tmux split-window -h -t mySession 'cargo run -p node-service'
	tmux attach-session -t mySession
