# HTTP echo with Spin
A simple echo http server with Spin

## Build
Build a spin application
```shell
cd echo
spin build
```

## Run
Run the application
```shell
spin up
```

## Send a request
Send a request
```shell
curl -X POST -d 'Hello, Fermyon!' http://127.0.0.1:3000
```
Then, you get a response as below:
```shell
Hello, Fermyon!
```