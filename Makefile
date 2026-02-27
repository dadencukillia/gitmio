MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT_ROOT := $(dir $(MKFILE_PATH))
UID := $(shell id -u)
GID := $(shell id -g)

build_client:
	docker run --rm \
		-u $(UID):$(GID) \
		-v $(PROJECT_ROOT)/client:/home/bun/app \
		-w /home/bun/app \
		oven/bun:latest \
		sh -c "bun install && bun run build"
