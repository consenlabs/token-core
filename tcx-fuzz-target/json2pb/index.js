var protobuf = require("protobufjs");
var fs = require('fs');
var path = require('path');

var outDir = "../hex_in";
var jsonIn = "../json_in";

function clearAllFiles(directory) {
    let files = fs.readdirSync(directory);
    for (const file of files) {
    fs.unlinkSync(path.join(directory, file));
    }
}

function walkDir(dir, callback) {
    fs.readdirSync(dir).forEach(f => {
        let dirPath = path.join(dir, f);
        let isDirectory = fs.statSync(dirPath).isDirectory();
        isDirectory ?
            walkDir(dirPath, callback) : callback(dir, f);
    });
};

protobuf.load('google/protobuf/any.proto', function (err, root) {
    if (err) {
        throw err;
    }

    var Any = root.lookup('Any');

    protobuf.load('api.proto', function (err, root) {
        if (err) {
            throw err;
        }

        var TcxAction = root.lookup('TcxAction');
        const generate_pb = (dir, f) => {
            var filePath = path.join(dir, f);
            var fileContent = fs.readFileSync(filePath);
            console.log(fileContent.toString());
            var payload = JSON.parse(fileContent.toString());

            var param = payload.param;

            // var paramBytes = encode(param);
            var ParamType = root.lookupType(param.type);
            var encodedParam = ParamType.create(param);
            var any = Any.create({
                type_url: param.type,
                value: ParamType.encode(encodedParam).finish(),
            })
            payload.param = any;

            // console.log(param.type, paramBytes.toString('hex'));
            // payload.param = paramBytes;
            // var ApiType = root.lookupType('api.TcxAction');
            var message = TcxAction.create({
                "method": payload.method,
                "param": any
            });
            var buffer = TcxAction.encode(message).finish();
            // ... do something with buffer
            var hexStr = buffer.toString('hex');
            console.log(payload.method, hexStr);
            fs.writeFileSync(path.join(outDir, f), hexStr);
        }


        clearAllFiles(outDir);
        walkDir(jsonIn, generate_pb);
        console.log("Generate Success");
    });
});


