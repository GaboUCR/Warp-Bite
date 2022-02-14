# Profiler.js test

## 1 character message 10s - 30s, no subs, 1000 clients:
Lost 133 connections at the beginning (timeout error).
The same thing happen on large messages.

## no messages, 1000 clients:
lost 185 connections at the beginning (timeout error).
Lost more than 300, to many open files error.
Stabilized at 506.

## 1 character message 10s - 30s, no subs, 500 clients:

Worked perfectly.

![Capture](https://user-images.githubusercontent.com/69367406/153915014-ce4f5151-627c-466f-98f3-e09f8bbbc97a.PNG)

## 1024 character message 1s, no subs, 500 clients:
Worked perfectly

![Capture](https://user-images.githubusercontent.com/69367406/153964917-84c77f6e-60c6-47a3-ac31-afeb17080578.PNG)

## 12 character message 16ms, 1 sub for everyone, 500 clients:
Worked perfectly.

![Capture](https://user-images.githubusercontent.com/69367406/153966806-fa0825f9-c403-4ac9-9545-2d0447fd30c7.PNG)


## 1 character message 10s - 30s, no subs, 600 clients:
Lost 104 connections. Error message:

![Capture1](https://user-images.githubusercontent.com/69367406/153917554-cae77ad4-a28e-4c3b-8ecf-db3513d6f46f.PNG)

Stabilized at 506:

![2](https://user-images.githubusercontent.com/69367406/153918205-080baa2e-e2fc-4adb-8893-536afa1eb578.PNG)

## 1 character message 10s - 30s, no subs, 550 clients:

Lost 48 connections due to To Many open files error.

## 

