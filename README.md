# Implementing metrics for rust and golang projects

## LICENSE

[Apache-2.0](https://github.com/dimitrmo/metrics?tab=Apache-2.0-1-ov-file#readme)

## Commands

### Running Go server

Server will start running at port 8080

```sh
make go
# Starting golang http server
```

### Running Rust server

Server will start running at port 8081

```sh
make rust
# Starting rust http server
```

### Run both servers with docker-compose

```sh
make run
```

## Endpoints

### Home

```
http://localhost:8080
http://localhost:8081
```

### Websocket

```
http://localhost:8080/ws
http://localhost:8081/ws
```

### Metrics

```
http://localhost:8080/metrics
http://localhost:8081/metrics
```
