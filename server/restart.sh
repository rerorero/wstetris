ps afx | grep wstetris | grep -v grep | awk '{ print $1}' | xargs kill
nohup cargo run > rust.log &

tail -f rust.log
