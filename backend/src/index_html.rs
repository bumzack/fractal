pub static INDEX_HTML: &str = r###"<!doctype html>
<html lang="en">
<head>
    <!-- Required meta tags -->
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <!-- Bootstrap CSS -->
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet"
          integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous">

    <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>

    <title>Fractal Thingi !</title>

    <script>
        $(document).ready(function () {

            $("#renderscene").click(function () {

                let request = {
                    "width": 1200,
                    "z1": {
                        "a": -2.0,
                        "b": 1.5
                    },
                    "z2": {
                         "a": 1.8,
                        "b": -1.5
                    },
                    "max_iterations": 50,
                    "colors": 256,
                    "x_tiles": 10,
                    "y_tiles": 10
                };

                const ctx = $("#canvas")[0].getContext('2d');
                ctx.canvas.width = 1200;     // parseInt($("#widthInput").val());
                ctx.canvas.height = 1400;     // parseInt($("#heightInput").val());

                $("#renderscene").prop("disabled", true);
                const uri = 'ws://localhost:3000/api/crossbeamtiles';
                const ws = new WebSocket(uri);

                ws.onopen = function () {
                    $("#statustext").html("Running!");

                    console.log("sending request ", JSON.stringify(request));
                    let request_string = JSON.stringify(request);
                    ws.send(request_string);
                };

                ws.onmessage = function (msg) {
                    prcoess_message(msg.data);
                };

                ws.onclose = function () {
                    console.log("websocket closed");
                    $("#statustext").html("finished!");
                    $("#renderscene").prop("disabled", false);
                };
            });

            function prcoess_message(msg) {
                const ctx = $("#canvas")[0].getContext('2d');
                const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height)

                var tile = JSON.parse(msg);
                var it = 0;
                console.log(" tile idx ", tile.idx);
                for (let i = 0; i < tile.points.length; i++) {
                    const point = tile.points[i];
                    var idx = parseInt(point.y * canvas.width * 4 + point.x * 4);

                    var r = parseInt(point.c.r  );
                    var g = parseInt(point.c.g  );
                    var b = parseInt(point.c.b  );

                    imageData.data[idx] = r;
                    imageData.data[idx + 1] = g;
                    imageData.data[idx + 2] = b;
                    imageData.data[idx + 3] = 255;
                }
                console.log("putting data back");
                ctx.putImageData(imageData, 0, 0, 0, 0, canvas.width, canvas.height);
            }
        });
    </script>
</head>
<body>
<div class="container-fluid">
    <div class="row">
        <div class="col-4">
            <h1>Hello, fractal thingi!!</h1>
        </div>
    </div>

    <div class="row">
        <div class="col-3">
            <form class="row g-3">
                <button id="renderscene" type="button" class="btn btn-primary">Render scene</button>
            </form>
            <br/>
            <div><p id="statustext"></p>
            </div>
        </div>
        <div class="col-9">
            <canvas id="canvas" width="800" height="600"></canvas>
        </div>
    </div>
</div>
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM"
        crossorigin="anonymous"></script>

</body>
</html>

"###;
