GET http://localhost:3001/

HTTP/* 200
[Asserts]
header "Content-Type" contains "text/html"
body contains "<h1>Welcome to the Advent of Spin!</h1>"
body contains "Santas on his way"