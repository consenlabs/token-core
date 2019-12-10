var protobuf = require("protobufjs");
var fs = require('fs');
var path = require('path');
const { spawnSync } = require( 'child_process' );

var outDir = "../test_result";
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

        protobuf.load("api_params.proto", function (err, root) {
            if (err)
                throw err;
            const generate_pb = (dir, f) => {
                var filePath = path.join(dir, f);
                var fileContent = fs.readFileSync(filePath);
                console.log(fileContent.toString());
                var payload = JSON.parse(fileContent.toString());

                var param = payload.param;

                var ParamType = root.lookupType(param.type);
                var encodedParam = ParamType.create(param);
                var any = Any.create({
                    type_url: param.type,
                    value: ParamType.encode(encodedParam).finish(),
                })
                payload.param = any;

                var message = TcxAction.create({
                    "method": payload.method,
                    "param": any
                });
                var buffer = TcxAction.encode(message).finish();
                var hexStr = buffer.toString('hex');
                const ls = spawnSync( '../../target/debug/tcx-tester', [hexStr] );

                fs.writeFileSync(path.join(outDir, f), ls.stdout.toString());
            }

            
            clearAllFiles(outDir);
            walkDir(jsonIn, generate_pb);
            console.log("Generate Success");
        });
    });

});


