APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME_1=hello_world
PROJECT_FOLDER_NAME_2=hello_cargo

run_rustc:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_1) && rustc main.rs && ./main"

new-project:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_2)
	
build:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo build"

run_cargo:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo run"

build/prod:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo build --release"

check:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo check"
