lsof -i :1420 | awk 'NR>1 {print $2}' | xargs kill -9
