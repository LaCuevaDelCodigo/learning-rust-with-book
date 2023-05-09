APP_CONTAINER_NAME=rust-book
PROJECT_FOLDER_NAME_1=variables
PROJECT_FOLDER_NAME_2=dataTypes
PROJECT_FOLDER_NAME_3=functions
PROJECT_FOLDER_NAME_4=branches
PROJECT_FOLDER_NAME_5=loops

new-project/variables:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_1)

new-project/dataTypes:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_2)

new-project/functions:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_3)

new-project/branches:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_4)

new-project/loops:
	@docker compose run --rm $(APP_CONTAINER_NAME) cargo new $(PROJECT_FOLDER_NAME_5)

run/variables:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_1) && cargo run"
	
run/dataTypes:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_2) && cargo run"

run/functions:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_3) && cargo run"

run/branches:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_4) && cargo run"

run/loops:
	@docker compose run --rm $(APP_CONTAINER_NAME) /bin/sh -c " cd $(PROJECT_FOLDER_NAME_5) && cargo run"
