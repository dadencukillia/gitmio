build_client:
	cd client; (bun install && bun run build)

move_client_bundle:
	mv ./client/build/ ./server/client-static/
	chmod -R 0777 ./server/client-static/

prepare_server_dev: build_client move_client_bundle
	- echo "Done!"

dev_server:
	@if [ ! -d "./server/client-static" ]; then \
		$(MAKE) prepare_server_dev; \
	fi
	cd server; cargo run

dev_client:
	cd client; bun run dev
