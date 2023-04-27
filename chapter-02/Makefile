APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME=guessing_game

new-project:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME)
	
build:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo build"

run:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo run"

doc:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo doc"

build/prod:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo build --release"

check:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME) && cargo check"
