var protobuf = require("protobufjs");
var fs = require('fs');
var path = require('path');

var outDir = "../hex_in";
var jsonIn = "../json_in";

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

    protobuf.load("api.proto", function (err, root) {
        if (err)
            throw err;

        // var payload = {
        //     "type": "api.InitTokenCoreXParam",
        //     "fileDir": "./test-data",
        //     "xpubCommonKey128": "B888D25EC8C12BD5043777B1AC49F872",
        //     "xpubCommonIv": "9C0C30889CBCC5E01AB5B2BB88715799"
        // };

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

        // walkDir(jsonIn, generate_pb);
        // console.log("Generate Success");
        var payload =
        {
            "type": "api.TcxAction",
            "method": "import_wallet_from_mnemonic",
            "param": {
                "type": "api.ImportWalletFromMnemonicParam",
                "chainType": "LITECOIN",
                "mnemonic": "salute slush now script nest law admit achieve voice soda fruit field",
                "name": "LTC-Wallet-1",
                "network": "MAINNET",
                "overwrite": true,
                "password": "Insecure Password",
                "passwordHint": "",
                "path": "m/44'/1'/0'/0/0",
                "segWit": "NONE",
                "source": "MNEMONIC"
            }
        };

        var param = payload.param;

        // var paramBytes = encode(param);
        var ParamType = root.lookupType("api.ImportWalletFromMnemonicParam");
        var encodedParam = ParamType.create(param);
        var any = Any.create({
            type_url: 'imToken.ImportWalletFromMnemonicParam',
            value: ParamType.encode(encodedParam).finish(),
        })
        payload.param = any;

        // console.log(param.type, paramBytes.toString('hex'));
        // payload.param = paramBytes;
        var ApiType = root.lookupType('api.TcxAction');
        var message = ApiType.create({
            "method": "import_wallet_from_mnemonic",
            "param": any
        });
        var buffer = ApiType.encode(message).finish();
        // ... do something with buffer
        var hexStr = buffer.toString('hex');
        console.log(payload.type, hexStr);

        var result = "0a2432383862613939372d633336322d343332382d393236652d336134316561646432333462120c4c54432d57616c6c65742d311a084c495445434f494e22224c524235336d7a38506d425044424838484670336635625648784a394271783850482a084d4e454d4f4e494330d9c6fcee053a260a24696d546f6b656e2e6170692e496d706f727457616c6c657446726f6d4d6e656d6f6e6963";
        var retBuf = Buffer.from(result, 'hex');
        var WalletType = root.lookup("api.WalletResult");
        var wallet = WalletType.decode(retBuf);
        console.log("Wallet: ", JSON.stringify(wallet));

    });

});


