# syntax=docker/dockerfile:1
# https://github.com/grafana/alloy/blob/main/Dockerfile

FROM grafana/alloy:v1.2.1

COPY config.alloy /etc/alloy/config.alloy

CMD [ "run", "/etc/alloy/config.alloy", \
      "--storage.path=/var/lib/alloy/data", \
      "--server.http.listen-addr=0.0.0.0:12345" ]
