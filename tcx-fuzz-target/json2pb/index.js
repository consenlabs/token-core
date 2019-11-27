var protobuf = require("protobufjs");
var fs = require('fs');
var path = require('path');

var outDir = "../hex_in";
var jsonIn = "../json_in";

function walkDir(dir, callback) {
    fs.readdirSync(dir).forEach( f => {
      let dirPath = path.join(dir, f);
      let isDirectory = fs.statSync(dirPath).isDirectory();
      isDirectory ? 
        walkDir(dirPath, callback) : callback(dir, f);
    });
  };



protobuf.load("api.proto", function(err, root) {
    if (err)
        throw err;
        
    var payload = {
        "type": "api.InitTokenCoreXParam",
        "fileDir": "./test-data",
        "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
        "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
    };

    const generate_pb = (dir, f) => {
        var filePath = path.join(dir, f);
        var fileContent = fs.readFileSync(filePath);
        console.log(fileContent.toString());
        var payload = JSON.parse(fileContent.toString());
        console.log(payload.type);
        var Type = root.lookupType(payload.type);
        var errMsg = Type.verify(payload);
        if (errMsg)
            throw Error(errMsg);
        var message = Type.create(payload); // or use .fromObject if conversion is necessary

        // Encode a message to an Uint8Array (browser) or Buffer (node)
        var buffer = Type.encode(message).finish();
        // ... do something with buffer
        var hexStr = buffer.toString('hex');
        console.log(hexStr);
        fs.writeFileSync(path.join(outDir, f), hexStr);
    }

    walkDir(jsonIn, generate_pb);
    console.log("Generate Success");
    var payloads = [
        {
            "type": "api.InitTokenCoreXParam",
            "fileDir": "./test-data",
            "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
            "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
        },
        {
            "type": "api.InitTokenCoreXParamCopy",
            "fileDir1": "./test-data",
            "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
            "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
        }
    ]
for (var idx in payloads) {
    var payload = payloads[idx];
    var Type = root.lookupType(payload.type);
    var errMsg = Type.verify(payload);
    if (errMsg)
        throw Error(errMsg);
    var message = Type.create(payload); // or use .fromObject if conversion is necessary

    // Encode a message to an Uint8Array (browser) or Buffer (node)
    var buffer = Type.encode(message).finish();
    // ... do something with buffer
    var hexStr = buffer.toString('hex');
    console.log(payload.type, hexStr);
}
    

    // Obtain a message type
    

    // Exemplary payload
    

    // Verify the payload if necessary (i.e. when possibly incomplete or invalid)
    

    // Create a new message
    

    // // Decode an Uint8Array (browser) or Buffer (node) to a message
    // var message = AwesomeMessage.decode(buffer);
    // // ... do something with message

    // // If the application uses length-delimited buffers, there is also encodeDelimited and decodeDelimited.

    // // Maybe convert the message back to a plain object
    // var object = AwesomeMessage.toObject(message, {
    //     longs: String,
    //     enums: String,
    //     bytes: String,
    //     // see ConversionOptions
    // });
});