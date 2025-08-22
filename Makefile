copy-config:
	cp -r backend/resources/config ${HOME}/.config/tanglit/
build-backend:
	cd backend && cargo build
	mkdir -p build && cp backend/target/release/backend build/tanglit-backend
cli: build-backend copy-config
build-app:
	cd frontend && make build
	cp frontend/src-tauri/target/release/bundle/appimage/frontend_*.AppImage build/tanglit-app
app: build-app copy-config
