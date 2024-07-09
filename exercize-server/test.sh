#!/bin/bash

echo "===================="
echo "[TEST] /ping"
echo 'curl -X GET http://localhost:8080/ping'
curl -X GET "http://localhost:8080/ping"
echo ""
echo "===================="
echo "[TEST] /fizzbuzz 1of3"
echo '-X GET http://localhost:8080/fizzbuzz?count=20'
curl -X GET "http://localhost:8080/fizzbuzz?count=20"
echo ""
echo "===================="
echo "[TEST] /fizzbuzz 2of3"
echo 'curl -X GET http://localhost:8080/fizzbuzz'
curl -X GET "http://localhost:8080/fizzbuzz"
echo ""
echo "===================="
echo "[TEST] /fizzbuzz 3of3"
echo 'curl -X GET http://localhost:8080/fizzbuzz?count=a'
curl -X GET "http://localhost:8080/fizzbuzz?count=a"
echo ""
echo "===================="
echo "[TEST] /add 1of4"
echo 'curl -X POST http://localhost:8080/add -H "Content-Type: application/json" -d "{\"left\": 18781, \"right\": 18783}"'
curl -X POST "http://localhost:8080/add" -H "Content-Type: application/json" -d '{"left": 18781, "right": 18783}'
echo ""
echo "===================="
echo "[TEST] /add 2of4"
echo 'curl -X POST http://localhost:8080/add -H "Content-Type: application/json" -d "{\"left\": 0, \"right\": -0}"'
curl -X POST "http://localhost:8080/add" -H "Content-Type: application/json" -d '{"left": 0, "right": -0}'
echo ""
echo "===================="
echo "[TEST] /add 3of4"
echo 'curl -X POST http://localhost:8080/add -H "Content-Type: application/json" -d "{\"left\": a, \"right\": b}"'
curl -X POST "http://localhost:8080/add" -H "Content-Type: application/json" -d '{"left": a, "right": b}'
echo ""
echo "===================="
echo "[TEST] /add 4of4"
echo 'curl -X POST http://localhost:8080/add -H "Content-Type: application/json" -d "{\"left\": 100}"'
curl -X POST "http://localhost:8080/add" -H "Content-Type: application/json" -d '{"left": 100}'
echo ""
echo "===================="
echo "[TEST] /students 1of3"
echo 'curl -X GET http://localhost:8080/students/1/1'
curl -X GET "http://localhost:8080/students/1/1"
echo ""
echo "===================="
echo "[TEST] /students 2of3"
echo 'curl -X GET http://localhost:8080/students/3/4'
curl -X GET "http://localhost:8080/students/3/4"
echo ""
echo "===================="
echo "[TEST] /students 3of3"
echo 'curl -X GET http://localhost:8080/students/5/1'
curl -X GET "http://localhost:8080/students/5/1"
echo ""