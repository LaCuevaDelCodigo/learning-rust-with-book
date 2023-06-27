APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME_1=enums
PROJECT_FOLDER_NAME_2=matching

new-project/enums:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_1)

run/enums:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_1) && cargo run"

new-project/matching:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_2)

run/matching:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo run"
