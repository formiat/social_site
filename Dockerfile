FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/social_site .
RUN apt-get update
RUN apt-get install libpq-dev -y
CMD ["/app/social_site"]
