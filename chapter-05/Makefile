APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME_1=structs
PROJECT_FOLDER_NAME_2=rectangles

new-project/structs:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_1)

run/structs:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_1) && cargo run"

new-project/rectangles:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_2)

run/rectangles:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo run"
