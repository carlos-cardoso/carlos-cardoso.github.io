build-frontend: 
    dx clean
    rm -r docs
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css 
    dx build --release 
    cp -r target/dx/homepage/release/web/public docs
    cp docs/index.html docs/404.html
    dx clean
    
test-frontend: build-frontend
    http-server docs docs/assets --cors -g -b -P http://127.0.0.1:8080\? 
