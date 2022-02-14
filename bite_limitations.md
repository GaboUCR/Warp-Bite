# Profiler.js test

## 1 character message 10s - 30s, no subs, 1000 clients:
Lost 133 connections at the beginning (timeout error).
The same thing happen on large messages.

## 1 character message 10s - 30s, no subs, 500 clients:

Worked perfectly.

![Capture](https://user-images.githubusercontent.com/69367406/153915014-ce4f5151-627c-466f-98f3-e09f8bbbc97a.PNG)

## 1 character message 10s - 30s, no subs, 600 clients:
Lost 104 connections. Error message:

![Capture1](https://user-images.githubusercontent.com/69367406/153917554-cae77ad4-a28e-4c3b-8ecf-db3513d6f46f.PNG)
