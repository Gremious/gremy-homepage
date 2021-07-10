#!/bin/bash
HOST=`cat HOST`

ssh ubuntu@$HOST "sudo rm -rf /root/.ssh/authorized_keys && sudo cp /home/ubuntu/.ssh/authorized_keys /root/.ssh/authorized_keys"
ssh root@$HOST "apt-get update -y" && \
ssh root@$HOST "apt-get upgrade -y" && \
ssh root@$HOST "apt-get install nginx -y" && \
ssh root@$HOST "chmod 755 /root" && \
ssh root@$HOST "ssh-keygen -t rsa -N \"\" -f /root/.ssh/id_rsa -C \"\$HOSTNAME@ec2\""
