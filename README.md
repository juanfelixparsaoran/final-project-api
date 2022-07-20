Nama : Juan Felix Parsaoran Tarigan<br>
Kode Peserta : KSAT006ONL017
# final-project-api

## Overview
This is project is a katalon project for testing https://restful-booker.herokuapp.com/apidoc/index.html API for final project BTDP bootcamp for Katalon

## Test Scenario

### Auth
- Request post with valid credential and header to https://restful-booker.herokuapp.com/auth and verify the status code

### Booking
- Request get to https://restful-booker.herokuapp.com/booking with valid header and verify all the result and the status code
- Request get to https://restful-booker.herokuapp.com/booking?firstname=sally&lastname=brown with valid header and verify all the result and the status code
- Request get to https://restful-booker.herokuapp.com/booking?checkin=2014-03-13&checkout=2014-05-21 with valid header and verify all the result and the status code
- Request post to https://restful-booker.herokuapp.com/booking with valid header and valid request body and verify the response and the status code
- Request put to https://restful-booker.herokuapp.com/booking/{id} with valid header and request body and verify the response and the status code
- Request patch to https://restful-booker.herokuapp.com/booking/{id} with valid header and request body and verify the response and the status code
- Request delete to https://restful-booker.herokuapp.com/booking/{id} with valid header and verify the response and the status code

### Ping
- Request get to https://restful-booker.herokuapp.com/ping and verify the status code

## Summary
All the test case is 100% passed
<br><br><br><br>

#### Notes : mungkin id yang dipakai pada TC pada project ini untuk get/put/patch/delete sudah dihapus di databasenya, jd jika TC failed karena terkena method not allowed, itu dikarenakan idnya sudah termodifikasi di databasenya. Pakai id yg baru untuk mencoba kembali.
