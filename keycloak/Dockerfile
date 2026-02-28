FROM alpine:latest
RUN apk add --no-cache curl bash
COPY dev-token-init.sh /dev-token-init.sh
RUN chmod +x /dev-token-init.sh
CMD ["/dev-token-init.sh"]