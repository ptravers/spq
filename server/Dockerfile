# Build Stage
FROM golang:1.12-alpine3.10 as build

RUN adduser -D -g '' spq

WORKDIR /go/src/spq_server
COPY . .

RUN go mod download && \
    go mod verify
RUN GOOS=linux GOARCH=amd64 go build -a -tags netgo -ldflags="-w -s" -o /go/bin/spq_server

# Minimize it
FROM scratch

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /go/bin/spq_server /go/bin/spq_server
USER spq

EXPOSE 8080
ENTRYPOINT ["/go/bin/spq_server"]
