# httplz (http please)

A simple commandline dev webserver to replace php -S 

php has been remove from macos. With all the bad, it was so simple to create
a webserver, serve a directory, to overcome any CORS related issues (loading assets through the file:// protocol) in the browser, when developing/prototyping.

## httplz is a wrapper around the rust library tiny_http.
It sole purpose is to serve any directory (and subdirs) on a specified local host address

## Pick a local ip and a free port
The following commands will serve the current working directory of your terminal

if you want other machines on your network to be able to reach it (fw might be an issue)
```
httplz 0.0.0.0:5678
```

if you only want your own machine to be able to reach it
```
httplz 127.0.0.1:8763
httplz localhost:4345
```

## Define a path to be served
The following command will serve the directory at the example path 
```
httplz 0.0.0.0:5678 /Users/test/example/dir
```