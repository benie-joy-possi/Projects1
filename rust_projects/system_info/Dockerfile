
FROM rust:latest  as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache libgcc



COPY --from=build /app/target/release/system_info ./system_info/

CMD [ "./system_info" ]
