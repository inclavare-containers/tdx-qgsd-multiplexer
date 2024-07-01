#! /bin/bash
bash -c 'cat > /etc/sgx_default_qcnl.conf' << EOF
# PCCS server address
PCCS_URL=https://sgx-dcap-server.${REGION_ID}.aliyuncs.com/sgx/certification/v4/
# To accept insecure HTTPS cert, set this option to FALSE
USE_SECURE_CERT=TRUE
EOF
/usr/bin/supervisord -c /etc/supervisor/conf.d/supervisord.conf