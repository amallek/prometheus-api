#!/bin/env bash
SVC_FILE=/etc/systemd/system/prometheus-api.service
if [ -f $SVC_FILE ]; then
    systemctl stop prometheus-api
fi

tee $SVC_FILE <<EOF
[Unit]
Description=Prometheus API
Wants=network-online.target
[Service]
Type=simple
Restart=always
ExecStart=/usr/local/bin/prometheus-api
ExecStop=/bin/kill -2 \$MAINPID
[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl start prometheus-api
systemctl enable prometheus-api
