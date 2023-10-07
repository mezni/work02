build:
	go build -o bin/main main.go

run:
	go run main.go

format:
	gofmt -w main.go
