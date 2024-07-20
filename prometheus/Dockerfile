# syntax=docker/dockerfile:1
# https://github.com/prometheus/prometheus/blob/main/Dockerfile

FROM prom/prometheus:v2.53.1

CMD [ "--config.file=/etc/prometheus/prometheus.yml", \
      "--storage.tsdb.path=/prometheus", \
      "--web.console.libraries=/usr/share/prometheus/console_libraries", \
      "--web.console.templates=/usr/share/prometheus/consoles", \
      "--web.enable-remote-write-receiver" ]
