#!/bin/bash
HOST=`cat HOST`
#Me = 80.5.14.235

ssh root@$HOST "mv server server-old" || true && \
scp target/release/server root@$HOST:~/server && \
ssh root@$HOST "chmod +x /root/server" && \
scp -r public root@$HOST:~/
ssh root@$HOST "pidof /root/server | xargs kill" || true && \
ssh root@$HOST "nohup /root/server > /dev/null 2>&1 &" && \
ssh root@$HOST "rm -rf /root/server-old" && \
scp server/nginx.conf root@$HOST:/etc/nginx/sites-available/default && \
ssh root@$HOST "systemctl restart nginx"
