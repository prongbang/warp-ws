# warp-ws

Learning by doing from [https://github.com/zupzup/warp-websockets-example](https://github.com/zupzup/warp-websockets-example)

### Error

```shell
WebSocket protocol error: Connection reset without closing handshake
```

### Register

```shell
curl -X POST 'http://localhost:8000/register' -H 'Content-Type: application/json' -d '{ "user_id": 1 }'
```

Response

```json
{"url":"ws://127.0.0.1:8000/ws/e1fadcd3919a4b78aad0251eed908b70"}
```

### Unregister

```shell
curl -X DELETE 'http://localhost:8000/register/e1fadcd3919a4b78aad0251eed908b70'
```

### Connect to the WebSocket

```shell
ws://127.0.0.1:8000/ws/e1fadcd3919a4b78aad0251eed908b70
```

### Publish Messages

```shell
curl -X POST 'http://localhost:8000/publish' \
    -H 'Content-Type: application/json' \
    -d '{"user_id": 1, "topic": "cats", "message": "are awesome"}'
```