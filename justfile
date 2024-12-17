build-frontend: 
    dx clean
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css 
    dx build --release --platform web
    rm -r docs
    cp -r target/dx/homepage/release/web/public docs


test-frontend: build-frontend
    http-server docs docs/assets --cors -g -b -P http://127.0.0.1:8080\? 
