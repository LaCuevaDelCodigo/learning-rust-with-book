APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME=structs

new-project:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME)

run:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo run"
