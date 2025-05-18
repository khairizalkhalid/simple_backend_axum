### 1. `/hello-world` (GET)
```bash
curl -X GET http://127.0.0.1:3000/hello-world
```

### 2. `/hello-world/{opt}` (GET)
Replace `{opt}` with the desired value for the `opt` path parameter:
```bash
curl -X GET http://127.0.0.1:3000/hello-world/some-option
curl -X GET http://127.0.0.1:3000/hello-world/0
curl -X GET http://127.0.0.1:3000/hello-world/1
curl -X GET http://127.0.0.1:3000/hello-world/22
```

### 3. `/hello-world-submit` (POST)
Send a JSON body that matches the `HelloWorldRequest` structure:
```bash
curl -X POST http://127.0.0.1:3000/hello-world-submit \
-H "Content-Type: application/json" \
-d '{
  "name": "John Doe",
  "message": "Hello, world!"
}'
```
