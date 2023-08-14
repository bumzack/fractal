curl -X POST http://localhost:8081/api/singlethreaded        -d '{"z1":{"a":-1.0,"b":1.5},"z2":{"a":1.0,"b":-1.5},"width":20,"max_iterations":100,"colors":256,"x_tiles":5,"y_tiles":19}'   --header "Content-Type: application/json"



```
  mvn clean verify
```

```
 java --enable-preview -jar target/Fractal-Thingi-0.0.1-SNAPSHOT.jar
```