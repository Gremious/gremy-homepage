#!/bin/bash
sudo systemctl stop homepage
sh ./tools/release/build.sh
sudo systemctl start homepage
