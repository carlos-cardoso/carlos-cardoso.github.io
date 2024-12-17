build-frontend: 
    dx clean
    rm -r docs
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css 
    dx build --release --platform web
    cp -r target/dx/homepage/release/web docs
    cp docs/public/index.html docs/public/404.html
    dx clean
    
    


test-frontend: build-frontend
    http-server docs docs/assets --cors -g -b -P http://127.0.0.1:8080\? 
