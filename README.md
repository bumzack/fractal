# fractal


## requests

## singlethreaded



```
curl -X POST  http://localhost:3000/api/singlethreaded -d '{ "colors": 16,   "max_iterations":1000, "width":1280,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```


```
curl -X POST  http://localhost:3000/api/singlethreaded -d '{ "colors": 256,   "max_iterations":1000,  "width":3840,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```

 

## multithreaded 



```
curl -X POST  http://localhost:3000/api/multithreaded -d '{ "colors": 16,   "max_iterations":50, "width":30,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```

```
curl -X POST  http://localhost:3000/api/multithreaded -d '{ "colors": 256,   "max_iterations":1000, "width":3840,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```

```
curl -X POST  http://localhost:3000/api/multithreaded -d '{ "colors": 256,   "max_iterations":2000, "width":7680,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```



## rayon

```
curl -X POST  http://localhost:3000/api/rayon -d '{ "colors": 16,   "max_iterations":50, "width":30,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```

```
curl -X POST  http://localhost:3000/api/rayon -d '{ "colors": 256,   "max_iterations":1000, "width":3840,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```

```
curl -X POST  http://localhost:3000/api/rayon -d '{ "colors": 256,   "max_iterations":2000, "width":7680,"z1":{"a":-2.0,"b":1.5},"z2":{"a":0.8,"b":-1.5}}'   -H 'Content-Type: application/json'
```






## Download Link

Download json and put it in the root folder of the project

https://www.ditig.com/downloads/256-colors.json
