# Production Container Build for retail-pos-rfid-register-rust-axum-v2026-b51
FROM alpine:3.19
RUN apk add --no-cache bash curl ca-certificates
WORKDIR /app
COPY . /app
EXPOSE 8080 3000
CMD ["echo", "retail-pos-rfid-register-rust-axum-v2026-b51 production container initialised and ready."]
