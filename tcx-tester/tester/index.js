var protobuf = require("protobufjs");
var fs = require('fs');
var path = require('path');
const { spawnSync } = require( 'child_process' );

var log4js = require('log4js');
var logger = log4js.getLogger();
logger.level = 'info';
logger.info("begin tcx tester:");

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

async function test() {
    let protoDir = "../../tcx-proto/src";
    let protoFiles = fs.readdirSync(protoDir)
    .filter(f => { return f.endsWith(".proto") }).map(f => {
            return path.join(protoDir, f);
    });
    protoFiles.push('google/protobuf/any.proto');
    let root = await protobuf.load(protoFiles);
    var Any = root.lookup('Any');
    var TcxAction = root.lookup('TcxAction');

    const generate_pb = (dir, f) => {
        var filePath = path.join(dir, f);
        var fileContent = fs.readFileSync(filePath);
        var payload = JSON.parse(fileContent.toString());

        var param = payload.param;

        if (payload.method.startsWith("sign")) {
            var ParamType = root.lookupType(param.input.type);
            var encodedParam = ParamType.create(param.input);
            var any = Any.create({
                type_url: param.input.type,
                value: ParamType.encode(encodedParam).finish(),
            })
            param.input = any;
        }
        
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
        logger.info(`test input: ${f}, method: ${payload.method}`);
        logger.debug(`param: `, JSON.stringify(payload, "", 2));
        logger.debug(`encoded hex: ${hexStr}`);
        const cmd = spawnSync( '../../target/debug/tcx-tester', [hexStr] );
        

        if (cmd.stderr && cmd.stderr.toString()) {
            logger.error(`test failed: ${cmd.stderr.toString()}`);
            fs.writeFileSync(path.join(outDir, f), cmd.stderr.toString());
        } else {
            logger.info("test result: ", cmd.stdout.toString());
            fs.writeFileSync(path.join(outDir, f), cmd.stdout.toString());
        }
        
    }

    clearAllFiles(outDir);
    walkDir(jsonIn, generate_pb);
    console.log("All test finished");

}


test();