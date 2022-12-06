serve: web
	python -m http.server 8080 --directory www

web: src/ examples/
	cargo build --target wasm32-unknown-unknown -p des-asm-web --release
	wasm-bindgen target/wasm32-unknown-unknown/release/des_asm_web.wasm --out-dir www --target no-modules --no-typescript
	cp -r examples www/
	./examples2html.sh | sed -f - src/index.html >www/index.html
