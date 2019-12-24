/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
"use strict";

var $protobuf = require("protobufjs/minimal");

// Common aliases
var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

$root.api = (function() {

    /**
     * Namespace api.
     * @exports api
     * @namespace
     */
    var api = {};

    api.TcxAction = (function() {

        /**
         * Properties of a TcxAction.
         * @memberof api
         * @interface ITcxAction
         * @property {string|null} [method] TcxAction method
         * @property {google.protobuf.IAny|null} [param] TcxAction param
         */

        /**
         * Constructs a new TcxAction.
         * @memberof api
         * @classdesc Represents a TcxAction.
         * @implements ITcxAction
         * @constructor
         * @param {api.ITcxAction=} [properties] Properties to set
         */
        function TcxAction(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * TcxAction method.
         * @member {string} method
         * @memberof api.TcxAction
         * @instance
         */
        TcxAction.prototype.method = "";

        /**
         * TcxAction param.
         * @member {google.protobuf.IAny|null|undefined} param
         * @memberof api.TcxAction
         * @instance
         */
        TcxAction.prototype.param = null;

        /**
         * Creates a new TcxAction instance using the specified properties.
         * @function create
         * @memberof api.TcxAction
         * @static
         * @param {api.ITcxAction=} [properties] Properties to set
         * @returns {api.TcxAction} TcxAction instance
         */
        TcxAction.create = function create(properties) {
            return new TcxAction(properties);
        };

        /**
         * Encodes the specified TcxAction message. Does not implicitly {@link api.TcxAction.verify|verify} messages.
         * @function encode
         * @memberof api.TcxAction
         * @static
         * @param {api.ITcxAction} message TcxAction message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TcxAction.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.method != null && message.hasOwnProperty("method"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.method);
            if (message.param != null && message.hasOwnProperty("param"))
                $root.google.protobuf.Any.encode(message.param, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified TcxAction message, length delimited. Does not implicitly {@link api.TcxAction.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.TcxAction
         * @static
         * @param {api.ITcxAction} message TcxAction message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TcxAction.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a TcxAction message from the specified reader or buffer.
         * @function decode
         * @memberof api.TcxAction
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.TcxAction} TcxAction
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TcxAction.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.TcxAction();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.method = reader.string();
                    break;
                case 2:
                    message.param = $root.google.protobuf.Any.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a TcxAction message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.TcxAction
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.TcxAction} TcxAction
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TcxAction.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a TcxAction message.
         * @function verify
         * @memberof api.TcxAction
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        TcxAction.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.method != null && message.hasOwnProperty("method"))
                if (!$util.isString(message.method))
                    return "method: string expected";
            if (message.param != null && message.hasOwnProperty("param")) {
                var error = $root.google.protobuf.Any.verify(message.param);
                if (error)
                    return "param." + error;
            }
            return null;
        };

        /**
         * Creates a TcxAction message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.TcxAction
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.TcxAction} TcxAction
         */
        TcxAction.fromObject = function fromObject(object) {
            if (object instanceof $root.api.TcxAction)
                return object;
            var message = new $root.api.TcxAction();
            if (object.method != null)
                message.method = String(object.method);
            if (object.param != null) {
                if (typeof object.param !== "object")
                    throw TypeError(".api.TcxAction.param: object expected");
                message.param = $root.google.protobuf.Any.fromObject(object.param);
            }
            return message;
        };

        /**
         * Creates a plain object from a TcxAction message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.TcxAction
         * @static
         * @param {api.TcxAction} message TcxAction
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        TcxAction.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.method = "";
                object.param = null;
            }
            if (message.method != null && message.hasOwnProperty("method"))
                object.method = message.method;
            if (message.param != null && message.hasOwnProperty("param"))
                object.param = $root.google.protobuf.Any.toObject(message.param, options);
            return object;
        };

        /**
         * Converts this TcxAction to JSON.
         * @function toJSON
         * @memberof api.TcxAction
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        TcxAction.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return TcxAction;
    })();

    api.Response = (function() {

        /**
         * Properties of a Response.
         * @memberof api
         * @interface IResponse
         * @property {boolean|null} [isSuccess] Response isSuccess
         * @property {string|null} [error] Response error
         */

        /**
         * Constructs a new Response.
         * @memberof api
         * @classdesc Represents a Response.
         * @implements IResponse
         * @constructor
         * @param {api.IResponse=} [properties] Properties to set
         */
        function Response(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Response isSuccess.
         * @member {boolean} isSuccess
         * @memberof api.Response
         * @instance
         */
        Response.prototype.isSuccess = false;

        /**
         * Response error.
         * @member {string} error
         * @memberof api.Response
         * @instance
         */
        Response.prototype.error = "";

        /**
         * Creates a new Response instance using the specified properties.
         * @function create
         * @memberof api.Response
         * @static
         * @param {api.IResponse=} [properties] Properties to set
         * @returns {api.Response} Response instance
         */
        Response.create = function create(properties) {
            return new Response(properties);
        };

        /**
         * Encodes the specified Response message. Does not implicitly {@link api.Response.verify|verify} messages.
         * @function encode
         * @memberof api.Response
         * @static
         * @param {api.IResponse} message Response message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Response.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.isSuccess != null && message.hasOwnProperty("isSuccess"))
                writer.uint32(/* id 1, wireType 0 =*/8).bool(message.isSuccess);
            if (message.error != null && message.hasOwnProperty("error"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.error);
            return writer;
        };

        /**
         * Encodes the specified Response message, length delimited. Does not implicitly {@link api.Response.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.Response
         * @static
         * @param {api.IResponse} message Response message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Response.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Response message from the specified reader or buffer.
         * @function decode
         * @memberof api.Response
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.Response} Response
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Response.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.Response();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.isSuccess = reader.bool();
                    break;
                case 2:
                    message.error = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Response message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.Response
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.Response} Response
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Response.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Response message.
         * @function verify
         * @memberof api.Response
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Response.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.isSuccess != null && message.hasOwnProperty("isSuccess"))
                if (typeof message.isSuccess !== "boolean")
                    return "isSuccess: boolean expected";
            if (message.error != null && message.hasOwnProperty("error"))
                if (!$util.isString(message.error))
                    return "error: string expected";
            return null;
        };

        /**
         * Creates a Response message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.Response
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.Response} Response
         */
        Response.fromObject = function fromObject(object) {
            if (object instanceof $root.api.Response)
                return object;
            var message = new $root.api.Response();
            if (object.isSuccess != null)
                message.isSuccess = Boolean(object.isSuccess);
            if (object.error != null)
                message.error = String(object.error);
            return message;
        };

        /**
         * Creates a plain object from a Response message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.Response
         * @static
         * @param {api.Response} message Response
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Response.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.isSuccess = false;
                object.error = "";
            }
            if (message.isSuccess != null && message.hasOwnProperty("isSuccess"))
                object.isSuccess = message.isSuccess;
            if (message.error != null && message.hasOwnProperty("error"))
                object.error = message.error;
            return object;
        };

        /**
         * Converts this Response to JSON.
         * @function toJSON
         * @memberof api.Response
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Response.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Response;
    })();

    api.InitTokenCoreXParam = (function() {

        /**
         * Properties of an InitTokenCoreXParam.
         * @memberof api
         * @interface IInitTokenCoreXParam
         * @property {string|null} [fileDir] InitTokenCoreXParam fileDir
         * @property {string|null} [xpubCommonKey] InitTokenCoreXParam xpubCommonKey
         * @property {string|null} [xpubCommonIv] InitTokenCoreXParam xpubCommonIv
         */

        /**
         * Constructs a new InitTokenCoreXParam.
         * @memberof api
         * @classdesc Represents an InitTokenCoreXParam.
         * @implements IInitTokenCoreXParam
         * @constructor
         * @param {api.IInitTokenCoreXParam=} [properties] Properties to set
         */
        function InitTokenCoreXParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * InitTokenCoreXParam fileDir.
         * @member {string} fileDir
         * @memberof api.InitTokenCoreXParam
         * @instance
         */
        InitTokenCoreXParam.prototype.fileDir = "";

        /**
         * InitTokenCoreXParam xpubCommonKey.
         * @member {string} xpubCommonKey
         * @memberof api.InitTokenCoreXParam
         * @instance
         */
        InitTokenCoreXParam.prototype.xpubCommonKey = "";

        /**
         * InitTokenCoreXParam xpubCommonIv.
         * @member {string} xpubCommonIv
         * @memberof api.InitTokenCoreXParam
         * @instance
         */
        InitTokenCoreXParam.prototype.xpubCommonIv = "";

        /**
         * Creates a new InitTokenCoreXParam instance using the specified properties.
         * @function create
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {api.IInitTokenCoreXParam=} [properties] Properties to set
         * @returns {api.InitTokenCoreXParam} InitTokenCoreXParam instance
         */
        InitTokenCoreXParam.create = function create(properties) {
            return new InitTokenCoreXParam(properties);
        };

        /**
         * Encodes the specified InitTokenCoreXParam message. Does not implicitly {@link api.InitTokenCoreXParam.verify|verify} messages.
         * @function encode
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {api.IInitTokenCoreXParam} message InitTokenCoreXParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        InitTokenCoreXParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.fileDir != null && message.hasOwnProperty("fileDir"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.fileDir);
            if (message.xpubCommonKey != null && message.hasOwnProperty("xpubCommonKey"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.xpubCommonKey);
            if (message.xpubCommonIv != null && message.hasOwnProperty("xpubCommonIv"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.xpubCommonIv);
            return writer;
        };

        /**
         * Encodes the specified InitTokenCoreXParam message, length delimited. Does not implicitly {@link api.InitTokenCoreXParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {api.IInitTokenCoreXParam} message InitTokenCoreXParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        InitTokenCoreXParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an InitTokenCoreXParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.InitTokenCoreXParam} InitTokenCoreXParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        InitTokenCoreXParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.InitTokenCoreXParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.fileDir = reader.string();
                    break;
                case 2:
                    message.xpubCommonKey = reader.string();
                    break;
                case 3:
                    message.xpubCommonIv = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an InitTokenCoreXParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.InitTokenCoreXParam} InitTokenCoreXParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        InitTokenCoreXParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an InitTokenCoreXParam message.
         * @function verify
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        InitTokenCoreXParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.fileDir != null && message.hasOwnProperty("fileDir"))
                if (!$util.isString(message.fileDir))
                    return "fileDir: string expected";
            if (message.xpubCommonKey != null && message.hasOwnProperty("xpubCommonKey"))
                if (!$util.isString(message.xpubCommonKey))
                    return "xpubCommonKey: string expected";
            if (message.xpubCommonIv != null && message.hasOwnProperty("xpubCommonIv"))
                if (!$util.isString(message.xpubCommonIv))
                    return "xpubCommonIv: string expected";
            return null;
        };

        /**
         * Creates an InitTokenCoreXParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.InitTokenCoreXParam} InitTokenCoreXParam
         */
        InitTokenCoreXParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.InitTokenCoreXParam)
                return object;
            var message = new $root.api.InitTokenCoreXParam();
            if (object.fileDir != null)
                message.fileDir = String(object.fileDir);
            if (object.xpubCommonKey != null)
                message.xpubCommonKey = String(object.xpubCommonKey);
            if (object.xpubCommonIv != null)
                message.xpubCommonIv = String(object.xpubCommonIv);
            return message;
        };

        /**
         * Creates a plain object from an InitTokenCoreXParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.InitTokenCoreXParam
         * @static
         * @param {api.InitTokenCoreXParam} message InitTokenCoreXParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        InitTokenCoreXParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.fileDir = "";
                object.xpubCommonKey = "";
                object.xpubCommonIv = "";
            }
            if (message.fileDir != null && message.hasOwnProperty("fileDir"))
                object.fileDir = message.fileDir;
            if (message.xpubCommonKey != null && message.hasOwnProperty("xpubCommonKey"))
                object.xpubCommonKey = message.xpubCommonKey;
            if (message.xpubCommonIv != null && message.hasOwnProperty("xpubCommonIv"))
                object.xpubCommonIv = message.xpubCommonIv;
            return object;
        };

        /**
         * Converts this InitTokenCoreXParam to JSON.
         * @function toJSON
         * @memberof api.InitTokenCoreXParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        InitTokenCoreXParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return InitTokenCoreXParam;
    })();

    api.HdStoreCreateParam = (function() {

        /**
         * Properties of a HdStoreCreateParam.
         * @memberof api
         * @interface IHdStoreCreateParam
         * @property {string|null} [password] HdStoreCreateParam password
         * @property {string|null} [passwordHint] HdStoreCreateParam passwordHint
         * @property {string|null} [name] HdStoreCreateParam name
         */

        /**
         * Constructs a new HdStoreCreateParam.
         * @memberof api
         * @classdesc Represents a HdStoreCreateParam.
         * @implements IHdStoreCreateParam
         * @constructor
         * @param {api.IHdStoreCreateParam=} [properties] Properties to set
         */
        function HdStoreCreateParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * HdStoreCreateParam password.
         * @member {string} password
         * @memberof api.HdStoreCreateParam
         * @instance
         */
        HdStoreCreateParam.prototype.password = "";

        /**
         * HdStoreCreateParam passwordHint.
         * @member {string} passwordHint
         * @memberof api.HdStoreCreateParam
         * @instance
         */
        HdStoreCreateParam.prototype.passwordHint = "";

        /**
         * HdStoreCreateParam name.
         * @member {string} name
         * @memberof api.HdStoreCreateParam
         * @instance
         */
        HdStoreCreateParam.prototype.name = "";

        /**
         * Creates a new HdStoreCreateParam instance using the specified properties.
         * @function create
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {api.IHdStoreCreateParam=} [properties] Properties to set
         * @returns {api.HdStoreCreateParam} HdStoreCreateParam instance
         */
        HdStoreCreateParam.create = function create(properties) {
            return new HdStoreCreateParam(properties);
        };

        /**
         * Encodes the specified HdStoreCreateParam message. Does not implicitly {@link api.HdStoreCreateParam.verify|verify} messages.
         * @function encode
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {api.IHdStoreCreateParam} message HdStoreCreateParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreCreateParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.password);
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.passwordHint);
            if (message.name != null && message.hasOwnProperty("name"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.name);
            return writer;
        };

        /**
         * Encodes the specified HdStoreCreateParam message, length delimited. Does not implicitly {@link api.HdStoreCreateParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {api.IHdStoreCreateParam} message HdStoreCreateParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreCreateParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a HdStoreCreateParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.HdStoreCreateParam} HdStoreCreateParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreCreateParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.HdStoreCreateParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.password = reader.string();
                    break;
                case 2:
                    message.passwordHint = reader.string();
                    break;
                case 3:
                    message.name = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a HdStoreCreateParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.HdStoreCreateParam} HdStoreCreateParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreCreateParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a HdStoreCreateParam message.
         * @function verify
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        HdStoreCreateParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                if (!$util.isString(message.passwordHint))
                    return "passwordHint: string expected";
            if (message.name != null && message.hasOwnProperty("name"))
                if (!$util.isString(message.name))
                    return "name: string expected";
            return null;
        };

        /**
         * Creates a HdStoreCreateParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.HdStoreCreateParam} HdStoreCreateParam
         */
        HdStoreCreateParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.HdStoreCreateParam)
                return object;
            var message = new $root.api.HdStoreCreateParam();
            if (object.password != null)
                message.password = String(object.password);
            if (object.passwordHint != null)
                message.passwordHint = String(object.passwordHint);
            if (object.name != null)
                message.name = String(object.name);
            return message;
        };

        /**
         * Creates a plain object from a HdStoreCreateParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.HdStoreCreateParam
         * @static
         * @param {api.HdStoreCreateParam} message HdStoreCreateParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        HdStoreCreateParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.password = "";
                object.passwordHint = "";
                object.name = "";
            }
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                object.passwordHint = message.passwordHint;
            if (message.name != null && message.hasOwnProperty("name"))
                object.name = message.name;
            return object;
        };

        /**
         * Converts this HdStoreCreateParam to JSON.
         * @function toJSON
         * @memberof api.HdStoreCreateParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        HdStoreCreateParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return HdStoreCreateParam;
    })();

    api.WalletResult = (function() {

        /**
         * Properties of a WalletResult.
         * @memberof api
         * @interface IWalletResult
         * @property {string|null} [id] WalletResult id
         * @property {string|null} [name] WalletResult name
         * @property {string|null} [source] WalletResult source
         * @property {Array.<api.IAccountResponse>|null} [accounts] WalletResult accounts
         * @property {number|Long|null} [createdAt] WalletResult createdAt
         */

        /**
         * Constructs a new WalletResult.
         * @memberof api
         * @classdesc Represents a WalletResult.
         * @implements IWalletResult
         * @constructor
         * @param {api.IWalletResult=} [properties] Properties to set
         */
        function WalletResult(properties) {
            this.accounts = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * WalletResult id.
         * @member {string} id
         * @memberof api.WalletResult
         * @instance
         */
        WalletResult.prototype.id = "";

        /**
         * WalletResult name.
         * @member {string} name
         * @memberof api.WalletResult
         * @instance
         */
        WalletResult.prototype.name = "";

        /**
         * WalletResult source.
         * @member {string} source
         * @memberof api.WalletResult
         * @instance
         */
        WalletResult.prototype.source = "";

        /**
         * WalletResult accounts.
         * @member {Array.<api.IAccountResponse>} accounts
         * @memberof api.WalletResult
         * @instance
         */
        WalletResult.prototype.accounts = $util.emptyArray;

        /**
         * WalletResult createdAt.
         * @member {number|Long} createdAt
         * @memberof api.WalletResult
         * @instance
         */
        WalletResult.prototype.createdAt = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Creates a new WalletResult instance using the specified properties.
         * @function create
         * @memberof api.WalletResult
         * @static
         * @param {api.IWalletResult=} [properties] Properties to set
         * @returns {api.WalletResult} WalletResult instance
         */
        WalletResult.create = function create(properties) {
            return new WalletResult(properties);
        };

        /**
         * Encodes the specified WalletResult message. Does not implicitly {@link api.WalletResult.verify|verify} messages.
         * @function encode
         * @memberof api.WalletResult
         * @static
         * @param {api.IWalletResult} message WalletResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        WalletResult.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.name != null && message.hasOwnProperty("name"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.name);
            if (message.source != null && message.hasOwnProperty("source"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.source);
            if (message.accounts != null && message.accounts.length)
                for (var i = 0; i < message.accounts.length; ++i)
                    $root.api.AccountResponse.encode(message.accounts[i], writer.uint32(/* id 4, wireType 2 =*/34).fork()).ldelim();
            if (message.createdAt != null && message.hasOwnProperty("createdAt"))
                writer.uint32(/* id 5, wireType 0 =*/40).int64(message.createdAt);
            return writer;
        };

        /**
         * Encodes the specified WalletResult message, length delimited. Does not implicitly {@link api.WalletResult.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.WalletResult
         * @static
         * @param {api.IWalletResult} message WalletResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        WalletResult.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a WalletResult message from the specified reader or buffer.
         * @function decode
         * @memberof api.WalletResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.WalletResult} WalletResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        WalletResult.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.WalletResult();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.name = reader.string();
                    break;
                case 3:
                    message.source = reader.string();
                    break;
                case 4:
                    if (!(message.accounts && message.accounts.length))
                        message.accounts = [];
                    message.accounts.push($root.api.AccountResponse.decode(reader, reader.uint32()));
                    break;
                case 5:
                    message.createdAt = reader.int64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a WalletResult message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.WalletResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.WalletResult} WalletResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        WalletResult.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a WalletResult message.
         * @function verify
         * @memberof api.WalletResult
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        WalletResult.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.name != null && message.hasOwnProperty("name"))
                if (!$util.isString(message.name))
                    return "name: string expected";
            if (message.source != null && message.hasOwnProperty("source"))
                if (!$util.isString(message.source))
                    return "source: string expected";
            if (message.accounts != null && message.hasOwnProperty("accounts")) {
                if (!Array.isArray(message.accounts))
                    return "accounts: array expected";
                for (var i = 0; i < message.accounts.length; ++i) {
                    var error = $root.api.AccountResponse.verify(message.accounts[i]);
                    if (error)
                        return "accounts." + error;
                }
            }
            if (message.createdAt != null && message.hasOwnProperty("createdAt"))
                if (!$util.isInteger(message.createdAt) && !(message.createdAt && $util.isInteger(message.createdAt.low) && $util.isInteger(message.createdAt.high)))
                    return "createdAt: integer|Long expected";
            return null;
        };

        /**
         * Creates a WalletResult message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.WalletResult
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.WalletResult} WalletResult
         */
        WalletResult.fromObject = function fromObject(object) {
            if (object instanceof $root.api.WalletResult)
                return object;
            var message = new $root.api.WalletResult();
            if (object.id != null)
                message.id = String(object.id);
            if (object.name != null)
                message.name = String(object.name);
            if (object.source != null)
                message.source = String(object.source);
            if (object.accounts) {
                if (!Array.isArray(object.accounts))
                    throw TypeError(".api.WalletResult.accounts: array expected");
                message.accounts = [];
                for (var i = 0; i < object.accounts.length; ++i) {
                    if (typeof object.accounts[i] !== "object")
                        throw TypeError(".api.WalletResult.accounts: object expected");
                    message.accounts[i] = $root.api.AccountResponse.fromObject(object.accounts[i]);
                }
            }
            if (object.createdAt != null)
                if ($util.Long)
                    (message.createdAt = $util.Long.fromValue(object.createdAt)).unsigned = false;
                else if (typeof object.createdAt === "string")
                    message.createdAt = parseInt(object.createdAt, 10);
                else if (typeof object.createdAt === "number")
                    message.createdAt = object.createdAt;
                else if (typeof object.createdAt === "object")
                    message.createdAt = new $util.LongBits(object.createdAt.low >>> 0, object.createdAt.high >>> 0).toNumber();
            return message;
        };

        /**
         * Creates a plain object from a WalletResult message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.WalletResult
         * @static
         * @param {api.WalletResult} message WalletResult
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        WalletResult.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.accounts = [];
            if (options.defaults) {
                object.id = "";
                object.name = "";
                object.source = "";
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.createdAt = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.createdAt = options.longs === String ? "0" : 0;
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.name != null && message.hasOwnProperty("name"))
                object.name = message.name;
            if (message.source != null && message.hasOwnProperty("source"))
                object.source = message.source;
            if (message.accounts && message.accounts.length) {
                object.accounts = [];
                for (var j = 0; j < message.accounts.length; ++j)
                    object.accounts[j] = $root.api.AccountResponse.toObject(message.accounts[j], options);
            }
            if (message.createdAt != null && message.hasOwnProperty("createdAt"))
                if (typeof message.createdAt === "number")
                    object.createdAt = options.longs === String ? String(message.createdAt) : message.createdAt;
                else
                    object.createdAt = options.longs === String ? $util.Long.prototype.toString.call(message.createdAt) : options.longs === Number ? new $util.LongBits(message.createdAt.low >>> 0, message.createdAt.high >>> 0).toNumber() : message.createdAt;
            return object;
        };

        /**
         * Converts this WalletResult to JSON.
         * @function toJSON
         * @memberof api.WalletResult
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        WalletResult.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return WalletResult;
    })();

    api.HdStoreImportParam = (function() {

        /**
         * Properties of a HdStoreImportParam.
         * @memberof api
         * @interface IHdStoreImportParam
         * @property {string|null} [mnemonic] HdStoreImportParam mnemonic
         * @property {string|null} [password] HdStoreImportParam password
         * @property {string|null} [source] HdStoreImportParam source
         * @property {string|null} [name] HdStoreImportParam name
         * @property {string|null} [passwordHint] HdStoreImportParam passwordHint
         * @property {boolean|null} [overwrite] HdStoreImportParam overwrite
         */

        /**
         * Constructs a new HdStoreImportParam.
         * @memberof api
         * @classdesc Represents a HdStoreImportParam.
         * @implements IHdStoreImportParam
         * @constructor
         * @param {api.IHdStoreImportParam=} [properties] Properties to set
         */
        function HdStoreImportParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * HdStoreImportParam mnemonic.
         * @member {string} mnemonic
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.mnemonic = "";

        /**
         * HdStoreImportParam password.
         * @member {string} password
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.password = "";

        /**
         * HdStoreImportParam source.
         * @member {string} source
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.source = "";

        /**
         * HdStoreImportParam name.
         * @member {string} name
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.name = "";

        /**
         * HdStoreImportParam passwordHint.
         * @member {string} passwordHint
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.passwordHint = "";

        /**
         * HdStoreImportParam overwrite.
         * @member {boolean} overwrite
         * @memberof api.HdStoreImportParam
         * @instance
         */
        HdStoreImportParam.prototype.overwrite = false;

        /**
         * Creates a new HdStoreImportParam instance using the specified properties.
         * @function create
         * @memberof api.HdStoreImportParam
         * @static
         * @param {api.IHdStoreImportParam=} [properties] Properties to set
         * @returns {api.HdStoreImportParam} HdStoreImportParam instance
         */
        HdStoreImportParam.create = function create(properties) {
            return new HdStoreImportParam(properties);
        };

        /**
         * Encodes the specified HdStoreImportParam message. Does not implicitly {@link api.HdStoreImportParam.verify|verify} messages.
         * @function encode
         * @memberof api.HdStoreImportParam
         * @static
         * @param {api.IHdStoreImportParam} message HdStoreImportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreImportParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.mnemonic != null && message.hasOwnProperty("mnemonic"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.mnemonic);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.source != null && message.hasOwnProperty("source"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.source);
            if (message.name != null && message.hasOwnProperty("name"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.name);
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                writer.uint32(/* id 5, wireType 2 =*/42).string(message.passwordHint);
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                writer.uint32(/* id 6, wireType 0 =*/48).bool(message.overwrite);
            return writer;
        };

        /**
         * Encodes the specified HdStoreImportParam message, length delimited. Does not implicitly {@link api.HdStoreImportParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.HdStoreImportParam
         * @static
         * @param {api.IHdStoreImportParam} message HdStoreImportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreImportParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a HdStoreImportParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.HdStoreImportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.HdStoreImportParam} HdStoreImportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreImportParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.HdStoreImportParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.mnemonic = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    message.source = reader.string();
                    break;
                case 4:
                    message.name = reader.string();
                    break;
                case 5:
                    message.passwordHint = reader.string();
                    break;
                case 6:
                    message.overwrite = reader.bool();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a HdStoreImportParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.HdStoreImportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.HdStoreImportParam} HdStoreImportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreImportParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a HdStoreImportParam message.
         * @function verify
         * @memberof api.HdStoreImportParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        HdStoreImportParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.mnemonic != null && message.hasOwnProperty("mnemonic"))
                if (!$util.isString(message.mnemonic))
                    return "mnemonic: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.source != null && message.hasOwnProperty("source"))
                if (!$util.isString(message.source))
                    return "source: string expected";
            if (message.name != null && message.hasOwnProperty("name"))
                if (!$util.isString(message.name))
                    return "name: string expected";
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                if (!$util.isString(message.passwordHint))
                    return "passwordHint: string expected";
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                if (typeof message.overwrite !== "boolean")
                    return "overwrite: boolean expected";
            return null;
        };

        /**
         * Creates a HdStoreImportParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.HdStoreImportParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.HdStoreImportParam} HdStoreImportParam
         */
        HdStoreImportParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.HdStoreImportParam)
                return object;
            var message = new $root.api.HdStoreImportParam();
            if (object.mnemonic != null)
                message.mnemonic = String(object.mnemonic);
            if (object.password != null)
                message.password = String(object.password);
            if (object.source != null)
                message.source = String(object.source);
            if (object.name != null)
                message.name = String(object.name);
            if (object.passwordHint != null)
                message.passwordHint = String(object.passwordHint);
            if (object.overwrite != null)
                message.overwrite = Boolean(object.overwrite);
            return message;
        };

        /**
         * Creates a plain object from a HdStoreImportParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.HdStoreImportParam
         * @static
         * @param {api.HdStoreImportParam} message HdStoreImportParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        HdStoreImportParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.mnemonic = "";
                object.password = "";
                object.source = "";
                object.name = "";
                object.passwordHint = "";
                object.overwrite = false;
            }
            if (message.mnemonic != null && message.hasOwnProperty("mnemonic"))
                object.mnemonic = message.mnemonic;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.source != null && message.hasOwnProperty("source"))
                object.source = message.source;
            if (message.name != null && message.hasOwnProperty("name"))
                object.name = message.name;
            if (message.passwordHint != null && message.hasOwnProperty("passwordHint"))
                object.passwordHint = message.passwordHint;
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                object.overwrite = message.overwrite;
            return object;
        };

        /**
         * Converts this HdStoreImportParam to JSON.
         * @function toJSON
         * @memberof api.HdStoreImportParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        HdStoreImportParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return HdStoreImportParam;
    })();

    api.KeystoreCommonDeriveParam = (function() {

        /**
         * Properties of a KeystoreCommonDeriveParam.
         * @memberof api
         * @interface IKeystoreCommonDeriveParam
         * @property {string|null} [id] KeystoreCommonDeriveParam id
         * @property {string|null} [password] KeystoreCommonDeriveParam password
         * @property {Array.<api.KeystoreCommonDeriveParam.IDerivation>|null} [derivations] KeystoreCommonDeriveParam derivations
         */

        /**
         * Constructs a new KeystoreCommonDeriveParam.
         * @memberof api
         * @classdesc Represents a KeystoreCommonDeriveParam.
         * @implements IKeystoreCommonDeriveParam
         * @constructor
         * @param {api.IKeystoreCommonDeriveParam=} [properties] Properties to set
         */
        function KeystoreCommonDeriveParam(properties) {
            this.derivations = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * KeystoreCommonDeriveParam id.
         * @member {string} id
         * @memberof api.KeystoreCommonDeriveParam
         * @instance
         */
        KeystoreCommonDeriveParam.prototype.id = "";

        /**
         * KeystoreCommonDeriveParam password.
         * @member {string} password
         * @memberof api.KeystoreCommonDeriveParam
         * @instance
         */
        KeystoreCommonDeriveParam.prototype.password = "";

        /**
         * KeystoreCommonDeriveParam derivations.
         * @member {Array.<api.KeystoreCommonDeriveParam.IDerivation>} derivations
         * @memberof api.KeystoreCommonDeriveParam
         * @instance
         */
        KeystoreCommonDeriveParam.prototype.derivations = $util.emptyArray;

        /**
         * Creates a new KeystoreCommonDeriveParam instance using the specified properties.
         * @function create
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {api.IKeystoreCommonDeriveParam=} [properties] Properties to set
         * @returns {api.KeystoreCommonDeriveParam} KeystoreCommonDeriveParam instance
         */
        KeystoreCommonDeriveParam.create = function create(properties) {
            return new KeystoreCommonDeriveParam(properties);
        };

        /**
         * Encodes the specified KeystoreCommonDeriveParam message. Does not implicitly {@link api.KeystoreCommonDeriveParam.verify|verify} messages.
         * @function encode
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {api.IKeystoreCommonDeriveParam} message KeystoreCommonDeriveParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonDeriveParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.derivations != null && message.derivations.length)
                for (var i = 0; i < message.derivations.length; ++i)
                    $root.api.KeystoreCommonDeriveParam.Derivation.encode(message.derivations[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified KeystoreCommonDeriveParam message, length delimited. Does not implicitly {@link api.KeystoreCommonDeriveParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {api.IKeystoreCommonDeriveParam} message KeystoreCommonDeriveParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonDeriveParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a KeystoreCommonDeriveParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.KeystoreCommonDeriveParam} KeystoreCommonDeriveParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonDeriveParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonDeriveParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    if (!(message.derivations && message.derivations.length))
                        message.derivations = [];
                    message.derivations.push($root.api.KeystoreCommonDeriveParam.Derivation.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a KeystoreCommonDeriveParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.KeystoreCommonDeriveParam} KeystoreCommonDeriveParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonDeriveParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a KeystoreCommonDeriveParam message.
         * @function verify
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        KeystoreCommonDeriveParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.derivations != null && message.hasOwnProperty("derivations")) {
                if (!Array.isArray(message.derivations))
                    return "derivations: array expected";
                for (var i = 0; i < message.derivations.length; ++i) {
                    var error = $root.api.KeystoreCommonDeriveParam.Derivation.verify(message.derivations[i]);
                    if (error)
                        return "derivations." + error;
                }
            }
            return null;
        };

        /**
         * Creates a KeystoreCommonDeriveParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.KeystoreCommonDeriveParam} KeystoreCommonDeriveParam
         */
        KeystoreCommonDeriveParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.KeystoreCommonDeriveParam)
                return object;
            var message = new $root.api.KeystoreCommonDeriveParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.password != null)
                message.password = String(object.password);
            if (object.derivations) {
                if (!Array.isArray(object.derivations))
                    throw TypeError(".api.KeystoreCommonDeriveParam.derivations: array expected");
                message.derivations = [];
                for (var i = 0; i < object.derivations.length; ++i) {
                    if (typeof object.derivations[i] !== "object")
                        throw TypeError(".api.KeystoreCommonDeriveParam.derivations: object expected");
                    message.derivations[i] = $root.api.KeystoreCommonDeriveParam.Derivation.fromObject(object.derivations[i]);
                }
            }
            return message;
        };

        /**
         * Creates a plain object from a KeystoreCommonDeriveParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.KeystoreCommonDeriveParam
         * @static
         * @param {api.KeystoreCommonDeriveParam} message KeystoreCommonDeriveParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        KeystoreCommonDeriveParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.derivations = [];
            if (options.defaults) {
                object.id = "";
                object.password = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.derivations && message.derivations.length) {
                object.derivations = [];
                for (var j = 0; j < message.derivations.length; ++j)
                    object.derivations[j] = $root.api.KeystoreCommonDeriveParam.Derivation.toObject(message.derivations[j], options);
            }
            return object;
        };

        /**
         * Converts this KeystoreCommonDeriveParam to JSON.
         * @function toJSON
         * @memberof api.KeystoreCommonDeriveParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        KeystoreCommonDeriveParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        KeystoreCommonDeriveParam.Derivation = (function() {

            /**
             * Properties of a Derivation.
             * @memberof api.KeystoreCommonDeriveParam
             * @interface IDerivation
             * @property {string|null} [chainType] Derivation chainType
             * @property {string|null} [path] Derivation path
             * @property {string|null} [network] Derivation network
             * @property {string|null} [segWit] Derivation segWit
             * @property {string|null} [chainId] Derivation chainId
             */

            /**
             * Constructs a new Derivation.
             * @memberof api.KeystoreCommonDeriveParam
             * @classdesc Represents a Derivation.
             * @implements IDerivation
             * @constructor
             * @param {api.KeystoreCommonDeriveParam.IDerivation=} [properties] Properties to set
             */
            function Derivation(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Derivation chainType.
             * @member {string} chainType
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             */
            Derivation.prototype.chainType = "";

            /**
             * Derivation path.
             * @member {string} path
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             */
            Derivation.prototype.path = "";

            /**
             * Derivation network.
             * @member {string} network
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             */
            Derivation.prototype.network = "";

            /**
             * Derivation segWit.
             * @member {string} segWit
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             */
            Derivation.prototype.segWit = "";

            /**
             * Derivation chainId.
             * @member {string} chainId
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             */
            Derivation.prototype.chainId = "";

            /**
             * Creates a new Derivation instance using the specified properties.
             * @function create
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {api.KeystoreCommonDeriveParam.IDerivation=} [properties] Properties to set
             * @returns {api.KeystoreCommonDeriveParam.Derivation} Derivation instance
             */
            Derivation.create = function create(properties) {
                return new Derivation(properties);
            };

            /**
             * Encodes the specified Derivation message. Does not implicitly {@link api.KeystoreCommonDeriveParam.Derivation.verify|verify} messages.
             * @function encode
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {api.KeystoreCommonDeriveParam.IDerivation} message Derivation message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Derivation.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.chainType != null && message.hasOwnProperty("chainType"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.chainType);
                if (message.path != null && message.hasOwnProperty("path"))
                    writer.uint32(/* id 2, wireType 2 =*/18).string(message.path);
                if (message.network != null && message.hasOwnProperty("network"))
                    writer.uint32(/* id 3, wireType 2 =*/26).string(message.network);
                if (message.segWit != null && message.hasOwnProperty("segWit"))
                    writer.uint32(/* id 4, wireType 2 =*/34).string(message.segWit);
                if (message.chainId != null && message.hasOwnProperty("chainId"))
                    writer.uint32(/* id 5, wireType 2 =*/42).string(message.chainId);
                return writer;
            };

            /**
             * Encodes the specified Derivation message, length delimited. Does not implicitly {@link api.KeystoreCommonDeriveParam.Derivation.verify|verify} messages.
             * @function encodeDelimited
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {api.KeystoreCommonDeriveParam.IDerivation} message Derivation message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Derivation.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes a Derivation message from the specified reader or buffer.
             * @function decode
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {api.KeystoreCommonDeriveParam.Derivation} Derivation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Derivation.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonDeriveParam.Derivation();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.chainType = reader.string();
                        break;
                    case 2:
                        message.path = reader.string();
                        break;
                    case 3:
                        message.network = reader.string();
                        break;
                    case 4:
                        message.segWit = reader.string();
                        break;
                    case 5:
                        message.chainId = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes a Derivation message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {api.KeystoreCommonDeriveParam.Derivation} Derivation
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Derivation.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies a Derivation message.
             * @function verify
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Derivation.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.chainType != null && message.hasOwnProperty("chainType"))
                    if (!$util.isString(message.chainType))
                        return "chainType: string expected";
                if (message.path != null && message.hasOwnProperty("path"))
                    if (!$util.isString(message.path))
                        return "path: string expected";
                if (message.network != null && message.hasOwnProperty("network"))
                    if (!$util.isString(message.network))
                        return "network: string expected";
                if (message.segWit != null && message.hasOwnProperty("segWit"))
                    if (!$util.isString(message.segWit))
                        return "segWit: string expected";
                if (message.chainId != null && message.hasOwnProperty("chainId"))
                    if (!$util.isString(message.chainId))
                        return "chainId: string expected";
                return null;
            };

            /**
             * Creates a Derivation message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {api.KeystoreCommonDeriveParam.Derivation} Derivation
             */
            Derivation.fromObject = function fromObject(object) {
                if (object instanceof $root.api.KeystoreCommonDeriveParam.Derivation)
                    return object;
                var message = new $root.api.KeystoreCommonDeriveParam.Derivation();
                if (object.chainType != null)
                    message.chainType = String(object.chainType);
                if (object.path != null)
                    message.path = String(object.path);
                if (object.network != null)
                    message.network = String(object.network);
                if (object.segWit != null)
                    message.segWit = String(object.segWit);
                if (object.chainId != null)
                    message.chainId = String(object.chainId);
                return message;
            };

            /**
             * Creates a plain object from a Derivation message. Also converts values to other types if specified.
             * @function toObject
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @static
             * @param {api.KeystoreCommonDeriveParam.Derivation} message Derivation
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Derivation.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.chainType = "";
                    object.path = "";
                    object.network = "";
                    object.segWit = "";
                    object.chainId = "";
                }
                if (message.chainType != null && message.hasOwnProperty("chainType"))
                    object.chainType = message.chainType;
                if (message.path != null && message.hasOwnProperty("path"))
                    object.path = message.path;
                if (message.network != null && message.hasOwnProperty("network"))
                    object.network = message.network;
                if (message.segWit != null && message.hasOwnProperty("segWit"))
                    object.segWit = message.segWit;
                if (message.chainId != null && message.hasOwnProperty("chainId"))
                    object.chainId = message.chainId;
                return object;
            };

            /**
             * Converts this Derivation to JSON.
             * @function toJSON
             * @memberof api.KeystoreCommonDeriveParam.Derivation
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Derivation.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            return Derivation;
        })();

        return KeystoreCommonDeriveParam;
    })();

    api.AccountResponse = (function() {

        /**
         * Properties of an AccountResponse.
         * @memberof api
         * @interface IAccountResponse
         * @property {string|null} [chainType] AccountResponse chainType
         * @property {string|null} [address] AccountResponse address
         * @property {string|null} [path] AccountResponse path
         * @property {string|null} [extendedXpubKey] AccountResponse extendedXpubKey
         */

        /**
         * Constructs a new AccountResponse.
         * @memberof api
         * @classdesc Represents an AccountResponse.
         * @implements IAccountResponse
         * @constructor
         * @param {api.IAccountResponse=} [properties] Properties to set
         */
        function AccountResponse(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * AccountResponse chainType.
         * @member {string} chainType
         * @memberof api.AccountResponse
         * @instance
         */
        AccountResponse.prototype.chainType = "";

        /**
         * AccountResponse address.
         * @member {string} address
         * @memberof api.AccountResponse
         * @instance
         */
        AccountResponse.prototype.address = "";

        /**
         * AccountResponse path.
         * @member {string} path
         * @memberof api.AccountResponse
         * @instance
         */
        AccountResponse.prototype.path = "";

        /**
         * AccountResponse extendedXpubKey.
         * @member {string} extendedXpubKey
         * @memberof api.AccountResponse
         * @instance
         */
        AccountResponse.prototype.extendedXpubKey = "";

        /**
         * Creates a new AccountResponse instance using the specified properties.
         * @function create
         * @memberof api.AccountResponse
         * @static
         * @param {api.IAccountResponse=} [properties] Properties to set
         * @returns {api.AccountResponse} AccountResponse instance
         */
        AccountResponse.create = function create(properties) {
            return new AccountResponse(properties);
        };

        /**
         * Encodes the specified AccountResponse message. Does not implicitly {@link api.AccountResponse.verify|verify} messages.
         * @function encode
         * @memberof api.AccountResponse
         * @static
         * @param {api.IAccountResponse} message AccountResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        AccountResponse.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.chainType);
            if (message.address != null && message.hasOwnProperty("address"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.address);
            if (message.path != null && message.hasOwnProperty("path"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.path);
            if (message.extendedXpubKey != null && message.hasOwnProperty("extendedXpubKey"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.extendedXpubKey);
            return writer;
        };

        /**
         * Encodes the specified AccountResponse message, length delimited. Does not implicitly {@link api.AccountResponse.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.AccountResponse
         * @static
         * @param {api.IAccountResponse} message AccountResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        AccountResponse.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an AccountResponse message from the specified reader or buffer.
         * @function decode
         * @memberof api.AccountResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.AccountResponse} AccountResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        AccountResponse.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.AccountResponse();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.chainType = reader.string();
                    break;
                case 2:
                    message.address = reader.string();
                    break;
                case 3:
                    message.path = reader.string();
                    break;
                case 4:
                    message.extendedXpubKey = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an AccountResponse message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.AccountResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.AccountResponse} AccountResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        AccountResponse.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an AccountResponse message.
         * @function verify
         * @memberof api.AccountResponse
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        AccountResponse.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                if (!$util.isString(message.chainType))
                    return "chainType: string expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            if (message.path != null && message.hasOwnProperty("path"))
                if (!$util.isString(message.path))
                    return "path: string expected";
            if (message.extendedXpubKey != null && message.hasOwnProperty("extendedXpubKey"))
                if (!$util.isString(message.extendedXpubKey))
                    return "extendedXpubKey: string expected";
            return null;
        };

        /**
         * Creates an AccountResponse message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.AccountResponse
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.AccountResponse} AccountResponse
         */
        AccountResponse.fromObject = function fromObject(object) {
            if (object instanceof $root.api.AccountResponse)
                return object;
            var message = new $root.api.AccountResponse();
            if (object.chainType != null)
                message.chainType = String(object.chainType);
            if (object.address != null)
                message.address = String(object.address);
            if (object.path != null)
                message.path = String(object.path);
            if (object.extendedXpubKey != null)
                message.extendedXpubKey = String(object.extendedXpubKey);
            return message;
        };

        /**
         * Creates a plain object from an AccountResponse message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.AccountResponse
         * @static
         * @param {api.AccountResponse} message AccountResponse
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        AccountResponse.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.chainType = "";
                object.address = "";
                object.path = "";
                object.extendedXpubKey = "";
            }
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                object.chainType = message.chainType;
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            if (message.path != null && message.hasOwnProperty("path"))
                object.path = message.path;
            if (message.extendedXpubKey != null && message.hasOwnProperty("extendedXpubKey"))
                object.extendedXpubKey = message.extendedXpubKey;
            return object;
        };

        /**
         * Converts this AccountResponse to JSON.
         * @function toJSON
         * @memberof api.AccountResponse
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        AccountResponse.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return AccountResponse;
    })();

    api.AccountsResponse = (function() {

        /**
         * Properties of an AccountsResponse.
         * @memberof api
         * @interface IAccountsResponse
         * @property {Array.<api.IAccountResponse>|null} [accounts] AccountsResponse accounts
         */

        /**
         * Constructs a new AccountsResponse.
         * @memberof api
         * @classdesc Represents an AccountsResponse.
         * @implements IAccountsResponse
         * @constructor
         * @param {api.IAccountsResponse=} [properties] Properties to set
         */
        function AccountsResponse(properties) {
            this.accounts = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * AccountsResponse accounts.
         * @member {Array.<api.IAccountResponse>} accounts
         * @memberof api.AccountsResponse
         * @instance
         */
        AccountsResponse.prototype.accounts = $util.emptyArray;

        /**
         * Creates a new AccountsResponse instance using the specified properties.
         * @function create
         * @memberof api.AccountsResponse
         * @static
         * @param {api.IAccountsResponse=} [properties] Properties to set
         * @returns {api.AccountsResponse} AccountsResponse instance
         */
        AccountsResponse.create = function create(properties) {
            return new AccountsResponse(properties);
        };

        /**
         * Encodes the specified AccountsResponse message. Does not implicitly {@link api.AccountsResponse.verify|verify} messages.
         * @function encode
         * @memberof api.AccountsResponse
         * @static
         * @param {api.IAccountsResponse} message AccountsResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        AccountsResponse.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.accounts != null && message.accounts.length)
                for (var i = 0; i < message.accounts.length; ++i)
                    $root.api.AccountResponse.encode(message.accounts[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified AccountsResponse message, length delimited. Does not implicitly {@link api.AccountsResponse.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.AccountsResponse
         * @static
         * @param {api.IAccountsResponse} message AccountsResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        AccountsResponse.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an AccountsResponse message from the specified reader or buffer.
         * @function decode
         * @memberof api.AccountsResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.AccountsResponse} AccountsResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        AccountsResponse.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.AccountsResponse();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    if (!(message.accounts && message.accounts.length))
                        message.accounts = [];
                    message.accounts.push($root.api.AccountResponse.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an AccountsResponse message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.AccountsResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.AccountsResponse} AccountsResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        AccountsResponse.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an AccountsResponse message.
         * @function verify
         * @memberof api.AccountsResponse
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        AccountsResponse.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.accounts != null && message.hasOwnProperty("accounts")) {
                if (!Array.isArray(message.accounts))
                    return "accounts: array expected";
                for (var i = 0; i < message.accounts.length; ++i) {
                    var error = $root.api.AccountResponse.verify(message.accounts[i]);
                    if (error)
                        return "accounts." + error;
                }
            }
            return null;
        };

        /**
         * Creates an AccountsResponse message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.AccountsResponse
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.AccountsResponse} AccountsResponse
         */
        AccountsResponse.fromObject = function fromObject(object) {
            if (object instanceof $root.api.AccountsResponse)
                return object;
            var message = new $root.api.AccountsResponse();
            if (object.accounts) {
                if (!Array.isArray(object.accounts))
                    throw TypeError(".api.AccountsResponse.accounts: array expected");
                message.accounts = [];
                for (var i = 0; i < object.accounts.length; ++i) {
                    if (typeof object.accounts[i] !== "object")
                        throw TypeError(".api.AccountsResponse.accounts: object expected");
                    message.accounts[i] = $root.api.AccountResponse.fromObject(object.accounts[i]);
                }
            }
            return message;
        };

        /**
         * Creates a plain object from an AccountsResponse message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.AccountsResponse
         * @static
         * @param {api.AccountsResponse} message AccountsResponse
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        AccountsResponse.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.accounts = [];
            if (message.accounts && message.accounts.length) {
                object.accounts = [];
                for (var j = 0; j < message.accounts.length; ++j)
                    object.accounts[j] = $root.api.AccountResponse.toObject(message.accounts[j], options);
            }
            return object;
        };

        /**
         * Converts this AccountsResponse to JSON.
         * @function toJSON
         * @memberof api.AccountsResponse
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        AccountsResponse.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return AccountsResponse;
    })();

    api.KeystoreCommonExportResult = (function() {

        /**
         * Properties of a KeystoreCommonExportResult.
         * @memberof api
         * @interface IKeystoreCommonExportResult
         * @property {string|null} [id] KeystoreCommonExportResult id
         * @property {api.KeyType|null} [type] KeystoreCommonExportResult type
         * @property {string|null} [value] KeystoreCommonExportResult value
         */

        /**
         * Constructs a new KeystoreCommonExportResult.
         * @memberof api
         * @classdesc Represents a KeystoreCommonExportResult.
         * @implements IKeystoreCommonExportResult
         * @constructor
         * @param {api.IKeystoreCommonExportResult=} [properties] Properties to set
         */
        function KeystoreCommonExportResult(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * KeystoreCommonExportResult id.
         * @member {string} id
         * @memberof api.KeystoreCommonExportResult
         * @instance
         */
        KeystoreCommonExportResult.prototype.id = "";

        /**
         * KeystoreCommonExportResult type.
         * @member {api.KeyType} type
         * @memberof api.KeystoreCommonExportResult
         * @instance
         */
        KeystoreCommonExportResult.prototype.type = 0;

        /**
         * KeystoreCommonExportResult value.
         * @member {string} value
         * @memberof api.KeystoreCommonExportResult
         * @instance
         */
        KeystoreCommonExportResult.prototype.value = "";

        /**
         * Creates a new KeystoreCommonExportResult instance using the specified properties.
         * @function create
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {api.IKeystoreCommonExportResult=} [properties] Properties to set
         * @returns {api.KeystoreCommonExportResult} KeystoreCommonExportResult instance
         */
        KeystoreCommonExportResult.create = function create(properties) {
            return new KeystoreCommonExportResult(properties);
        };

        /**
         * Encodes the specified KeystoreCommonExportResult message. Does not implicitly {@link api.KeystoreCommonExportResult.verify|verify} messages.
         * @function encode
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {api.IKeystoreCommonExportResult} message KeystoreCommonExportResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExportResult.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.type != null && message.hasOwnProperty("type"))
                writer.uint32(/* id 2, wireType 0 =*/16).int32(message.type);
            if (message.value != null && message.hasOwnProperty("value"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.value);
            return writer;
        };

        /**
         * Encodes the specified KeystoreCommonExportResult message, length delimited. Does not implicitly {@link api.KeystoreCommonExportResult.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {api.IKeystoreCommonExportResult} message KeystoreCommonExportResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExportResult.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a KeystoreCommonExportResult message from the specified reader or buffer.
         * @function decode
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.KeystoreCommonExportResult} KeystoreCommonExportResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExportResult.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonExportResult();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.type = reader.int32();
                    break;
                case 3:
                    message.value = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a KeystoreCommonExportResult message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.KeystoreCommonExportResult} KeystoreCommonExportResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExportResult.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a KeystoreCommonExportResult message.
         * @function verify
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        KeystoreCommonExportResult.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.type != null && message.hasOwnProperty("type"))
                switch (message.type) {
                default:
                    return "type: enum value expected";
                case 0:
                case 1:
                    break;
                }
            if (message.value != null && message.hasOwnProperty("value"))
                if (!$util.isString(message.value))
                    return "value: string expected";
            return null;
        };

        /**
         * Creates a KeystoreCommonExportResult message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.KeystoreCommonExportResult} KeystoreCommonExportResult
         */
        KeystoreCommonExportResult.fromObject = function fromObject(object) {
            if (object instanceof $root.api.KeystoreCommonExportResult)
                return object;
            var message = new $root.api.KeystoreCommonExportResult();
            if (object.id != null)
                message.id = String(object.id);
            switch (object.type) {
            case "MNEMONIC":
            case 0:
                message.type = 0;
                break;
            case "PRIVATE_KEY":
            case 1:
                message.type = 1;
                break;
            }
            if (object.value != null)
                message.value = String(object.value);
            return message;
        };

        /**
         * Creates a plain object from a KeystoreCommonExportResult message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.KeystoreCommonExportResult
         * @static
         * @param {api.KeystoreCommonExportResult} message KeystoreCommonExportResult
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        KeystoreCommonExportResult.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.type = options.enums === String ? "MNEMONIC" : 0;
                object.value = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.type != null && message.hasOwnProperty("type"))
                object.type = options.enums === String ? $root.api.KeyType[message.type] : message.type;
            if (message.value != null && message.hasOwnProperty("value"))
                object.value = message.value;
            return object;
        };

        /**
         * Converts this KeystoreCommonExportResult to JSON.
         * @function toJSON
         * @memberof api.KeystoreCommonExportResult
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        KeystoreCommonExportResult.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return KeystoreCommonExportResult;
    })();

    /**
     * KeyType enum.
     * @name api.KeyType
     * @enum {string}
     * @property {number} MNEMONIC=0 MNEMONIC value
     * @property {number} PRIVATE_KEY=1 PRIVATE_KEY value
     */
    api.KeyType = (function() {
        var valuesById = {}, values = Object.create(valuesById);
        values[valuesById[0] = "MNEMONIC"] = 0;
        values[valuesById[1] = "PRIVATE_KEY"] = 1;
        return values;
    })();

    api.PrivateKeyStoreImportParam = (function() {

        /**
         * Properties of a PrivateKeyStoreImportParam.
         * @memberof api
         * @interface IPrivateKeyStoreImportParam
         * @property {string|null} [privateKey] PrivateKeyStoreImportParam privateKey
         * @property {string|null} [password] PrivateKeyStoreImportParam password
         * @property {boolean|null} [overwrite] PrivateKeyStoreImportParam overwrite
         */

        /**
         * Constructs a new PrivateKeyStoreImportParam.
         * @memberof api
         * @classdesc Represents a PrivateKeyStoreImportParam.
         * @implements IPrivateKeyStoreImportParam
         * @constructor
         * @param {api.IPrivateKeyStoreImportParam=} [properties] Properties to set
         */
        function PrivateKeyStoreImportParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * PrivateKeyStoreImportParam privateKey.
         * @member {string} privateKey
         * @memberof api.PrivateKeyStoreImportParam
         * @instance
         */
        PrivateKeyStoreImportParam.prototype.privateKey = "";

        /**
         * PrivateKeyStoreImportParam password.
         * @member {string} password
         * @memberof api.PrivateKeyStoreImportParam
         * @instance
         */
        PrivateKeyStoreImportParam.prototype.password = "";

        /**
         * PrivateKeyStoreImportParam overwrite.
         * @member {boolean} overwrite
         * @memberof api.PrivateKeyStoreImportParam
         * @instance
         */
        PrivateKeyStoreImportParam.prototype.overwrite = false;

        /**
         * Creates a new PrivateKeyStoreImportParam instance using the specified properties.
         * @function create
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {api.IPrivateKeyStoreImportParam=} [properties] Properties to set
         * @returns {api.PrivateKeyStoreImportParam} PrivateKeyStoreImportParam instance
         */
        PrivateKeyStoreImportParam.create = function create(properties) {
            return new PrivateKeyStoreImportParam(properties);
        };

        /**
         * Encodes the specified PrivateKeyStoreImportParam message. Does not implicitly {@link api.PrivateKeyStoreImportParam.verify|verify} messages.
         * @function encode
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {api.IPrivateKeyStoreImportParam} message PrivateKeyStoreImportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PrivateKeyStoreImportParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.privateKey != null && message.hasOwnProperty("privateKey"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.privateKey);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                writer.uint32(/* id 3, wireType 0 =*/24).bool(message.overwrite);
            return writer;
        };

        /**
         * Encodes the specified PrivateKeyStoreImportParam message, length delimited. Does not implicitly {@link api.PrivateKeyStoreImportParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {api.IPrivateKeyStoreImportParam} message PrivateKeyStoreImportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PrivateKeyStoreImportParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a PrivateKeyStoreImportParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.PrivateKeyStoreImportParam} PrivateKeyStoreImportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PrivateKeyStoreImportParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.PrivateKeyStoreImportParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.privateKey = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    message.overwrite = reader.bool();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a PrivateKeyStoreImportParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.PrivateKeyStoreImportParam} PrivateKeyStoreImportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PrivateKeyStoreImportParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a PrivateKeyStoreImportParam message.
         * @function verify
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        PrivateKeyStoreImportParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.privateKey != null && message.hasOwnProperty("privateKey"))
                if (!$util.isString(message.privateKey))
                    return "privateKey: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                if (typeof message.overwrite !== "boolean")
                    return "overwrite: boolean expected";
            return null;
        };

        /**
         * Creates a PrivateKeyStoreImportParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.PrivateKeyStoreImportParam} PrivateKeyStoreImportParam
         */
        PrivateKeyStoreImportParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.PrivateKeyStoreImportParam)
                return object;
            var message = new $root.api.PrivateKeyStoreImportParam();
            if (object.privateKey != null)
                message.privateKey = String(object.privateKey);
            if (object.password != null)
                message.password = String(object.password);
            if (object.overwrite != null)
                message.overwrite = Boolean(object.overwrite);
            return message;
        };

        /**
         * Creates a plain object from a PrivateKeyStoreImportParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.PrivateKeyStoreImportParam
         * @static
         * @param {api.PrivateKeyStoreImportParam} message PrivateKeyStoreImportParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        PrivateKeyStoreImportParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.privateKey = "";
                object.password = "";
                object.overwrite = false;
            }
            if (message.privateKey != null && message.hasOwnProperty("privateKey"))
                object.privateKey = message.privateKey;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.overwrite != null && message.hasOwnProperty("overwrite"))
                object.overwrite = message.overwrite;
            return object;
        };

        /**
         * Converts this PrivateKeyStoreImportParam to JSON.
         * @function toJSON
         * @memberof api.PrivateKeyStoreImportParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        PrivateKeyStoreImportParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return PrivateKeyStoreImportParam;
    })();

    api.PrivateKeyStoreExportParam = (function() {

        /**
         * Properties of a PrivateKeyStoreExportParam.
         * @memberof api
         * @interface IPrivateKeyStoreExportParam
         * @property {string|null} [id] PrivateKeyStoreExportParam id
         * @property {string|null} [password] PrivateKeyStoreExportParam password
         * @property {string|null} [chainType] PrivateKeyStoreExportParam chainType
         * @property {string|null} [network] PrivateKeyStoreExportParam network
         */

        /**
         * Constructs a new PrivateKeyStoreExportParam.
         * @memberof api
         * @classdesc Represents a PrivateKeyStoreExportParam.
         * @implements IPrivateKeyStoreExportParam
         * @constructor
         * @param {api.IPrivateKeyStoreExportParam=} [properties] Properties to set
         */
        function PrivateKeyStoreExportParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * PrivateKeyStoreExportParam id.
         * @member {string} id
         * @memberof api.PrivateKeyStoreExportParam
         * @instance
         */
        PrivateKeyStoreExportParam.prototype.id = "";

        /**
         * PrivateKeyStoreExportParam password.
         * @member {string} password
         * @memberof api.PrivateKeyStoreExportParam
         * @instance
         */
        PrivateKeyStoreExportParam.prototype.password = "";

        /**
         * PrivateKeyStoreExportParam chainType.
         * @member {string} chainType
         * @memberof api.PrivateKeyStoreExportParam
         * @instance
         */
        PrivateKeyStoreExportParam.prototype.chainType = "";

        /**
         * PrivateKeyStoreExportParam network.
         * @member {string} network
         * @memberof api.PrivateKeyStoreExportParam
         * @instance
         */
        PrivateKeyStoreExportParam.prototype.network = "";

        /**
         * Creates a new PrivateKeyStoreExportParam instance using the specified properties.
         * @function create
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {api.IPrivateKeyStoreExportParam=} [properties] Properties to set
         * @returns {api.PrivateKeyStoreExportParam} PrivateKeyStoreExportParam instance
         */
        PrivateKeyStoreExportParam.create = function create(properties) {
            return new PrivateKeyStoreExportParam(properties);
        };

        /**
         * Encodes the specified PrivateKeyStoreExportParam message. Does not implicitly {@link api.PrivateKeyStoreExportParam.verify|verify} messages.
         * @function encode
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {api.IPrivateKeyStoreExportParam} message PrivateKeyStoreExportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PrivateKeyStoreExportParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.chainType);
            if (message.network != null && message.hasOwnProperty("network"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.network);
            return writer;
        };

        /**
         * Encodes the specified PrivateKeyStoreExportParam message, length delimited. Does not implicitly {@link api.PrivateKeyStoreExportParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {api.IPrivateKeyStoreExportParam} message PrivateKeyStoreExportParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        PrivateKeyStoreExportParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a PrivateKeyStoreExportParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.PrivateKeyStoreExportParam} PrivateKeyStoreExportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PrivateKeyStoreExportParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.PrivateKeyStoreExportParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    message.chainType = reader.string();
                    break;
                case 4:
                    message.network = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a PrivateKeyStoreExportParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.PrivateKeyStoreExportParam} PrivateKeyStoreExportParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        PrivateKeyStoreExportParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a PrivateKeyStoreExportParam message.
         * @function verify
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        PrivateKeyStoreExportParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                if (!$util.isString(message.chainType))
                    return "chainType: string expected";
            if (message.network != null && message.hasOwnProperty("network"))
                if (!$util.isString(message.network))
                    return "network: string expected";
            return null;
        };

        /**
         * Creates a PrivateKeyStoreExportParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.PrivateKeyStoreExportParam} PrivateKeyStoreExportParam
         */
        PrivateKeyStoreExportParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.PrivateKeyStoreExportParam)
                return object;
            var message = new $root.api.PrivateKeyStoreExportParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.password != null)
                message.password = String(object.password);
            if (object.chainType != null)
                message.chainType = String(object.chainType);
            if (object.network != null)
                message.network = String(object.network);
            return message;
        };

        /**
         * Creates a plain object from a PrivateKeyStoreExportParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.PrivateKeyStoreExportParam
         * @static
         * @param {api.PrivateKeyStoreExportParam} message PrivateKeyStoreExportParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        PrivateKeyStoreExportParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.password = "";
                object.chainType = "";
                object.network = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                object.chainType = message.chainType;
            if (message.network != null && message.hasOwnProperty("network"))
                object.network = message.network;
            return object;
        };

        /**
         * Converts this PrivateKeyStoreExportParam to JSON.
         * @function toJSON
         * @memberof api.PrivateKeyStoreExportParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        PrivateKeyStoreExportParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return PrivateKeyStoreExportParam;
    })();

    api.WalletKeyParam = (function() {

        /**
         * Properties of a WalletKeyParam.
         * @memberof api
         * @interface IWalletKeyParam
         * @property {string|null} [id] WalletKeyParam id
         * @property {string|null} [password] WalletKeyParam password
         */

        /**
         * Constructs a new WalletKeyParam.
         * @memberof api
         * @classdesc Represents a WalletKeyParam.
         * @implements IWalletKeyParam
         * @constructor
         * @param {api.IWalletKeyParam=} [properties] Properties to set
         */
        function WalletKeyParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * WalletKeyParam id.
         * @member {string} id
         * @memberof api.WalletKeyParam
         * @instance
         */
        WalletKeyParam.prototype.id = "";

        /**
         * WalletKeyParam password.
         * @member {string} password
         * @memberof api.WalletKeyParam
         * @instance
         */
        WalletKeyParam.prototype.password = "";

        /**
         * Creates a new WalletKeyParam instance using the specified properties.
         * @function create
         * @memberof api.WalletKeyParam
         * @static
         * @param {api.IWalletKeyParam=} [properties] Properties to set
         * @returns {api.WalletKeyParam} WalletKeyParam instance
         */
        WalletKeyParam.create = function create(properties) {
            return new WalletKeyParam(properties);
        };

        /**
         * Encodes the specified WalletKeyParam message. Does not implicitly {@link api.WalletKeyParam.verify|verify} messages.
         * @function encode
         * @memberof api.WalletKeyParam
         * @static
         * @param {api.IWalletKeyParam} message WalletKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        WalletKeyParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            return writer;
        };

        /**
         * Encodes the specified WalletKeyParam message, length delimited. Does not implicitly {@link api.WalletKeyParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.WalletKeyParam
         * @static
         * @param {api.IWalletKeyParam} message WalletKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        WalletKeyParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a WalletKeyParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.WalletKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.WalletKeyParam} WalletKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        WalletKeyParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.WalletKeyParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a WalletKeyParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.WalletKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.WalletKeyParam} WalletKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        WalletKeyParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a WalletKeyParam message.
         * @function verify
         * @memberof api.WalletKeyParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        WalletKeyParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            return null;
        };

        /**
         * Creates a WalletKeyParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.WalletKeyParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.WalletKeyParam} WalletKeyParam
         */
        WalletKeyParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.WalletKeyParam)
                return object;
            var message = new $root.api.WalletKeyParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.password != null)
                message.password = String(object.password);
            return message;
        };

        /**
         * Creates a plain object from a WalletKeyParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.WalletKeyParam
         * @static
         * @param {api.WalletKeyParam} message WalletKeyParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        WalletKeyParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.password = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            return object;
        };

        /**
         * Converts this WalletKeyParam to JSON.
         * @function toJSON
         * @memberof api.WalletKeyParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        WalletKeyParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return WalletKeyParam;
    })();

    api.KeystoreCommonExistsParam = (function() {

        /**
         * Properties of a KeystoreCommonExistsParam.
         * @memberof api
         * @interface IKeystoreCommonExistsParam
         * @property {api.KeyType|null} [type] KeystoreCommonExistsParam type
         * @property {string|null} [value] KeystoreCommonExistsParam value
         */

        /**
         * Constructs a new KeystoreCommonExistsParam.
         * @memberof api
         * @classdesc Represents a KeystoreCommonExistsParam.
         * @implements IKeystoreCommonExistsParam
         * @constructor
         * @param {api.IKeystoreCommonExistsParam=} [properties] Properties to set
         */
        function KeystoreCommonExistsParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * KeystoreCommonExistsParam type.
         * @member {api.KeyType} type
         * @memberof api.KeystoreCommonExistsParam
         * @instance
         */
        KeystoreCommonExistsParam.prototype.type = 0;

        /**
         * KeystoreCommonExistsParam value.
         * @member {string} value
         * @memberof api.KeystoreCommonExistsParam
         * @instance
         */
        KeystoreCommonExistsParam.prototype.value = "";

        /**
         * Creates a new KeystoreCommonExistsParam instance using the specified properties.
         * @function create
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {api.IKeystoreCommonExistsParam=} [properties] Properties to set
         * @returns {api.KeystoreCommonExistsParam} KeystoreCommonExistsParam instance
         */
        KeystoreCommonExistsParam.create = function create(properties) {
            return new KeystoreCommonExistsParam(properties);
        };

        /**
         * Encodes the specified KeystoreCommonExistsParam message. Does not implicitly {@link api.KeystoreCommonExistsParam.verify|verify} messages.
         * @function encode
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {api.IKeystoreCommonExistsParam} message KeystoreCommonExistsParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExistsParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.type != null && message.hasOwnProperty("type"))
                writer.uint32(/* id 1, wireType 0 =*/8).int32(message.type);
            if (message.value != null && message.hasOwnProperty("value"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.value);
            return writer;
        };

        /**
         * Encodes the specified KeystoreCommonExistsParam message, length delimited. Does not implicitly {@link api.KeystoreCommonExistsParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {api.IKeystoreCommonExistsParam} message KeystoreCommonExistsParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExistsParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a KeystoreCommonExistsParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.KeystoreCommonExistsParam} KeystoreCommonExistsParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExistsParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonExistsParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.type = reader.int32();
                    break;
                case 2:
                    message.value = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a KeystoreCommonExistsParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.KeystoreCommonExistsParam} KeystoreCommonExistsParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExistsParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a KeystoreCommonExistsParam message.
         * @function verify
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        KeystoreCommonExistsParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.type != null && message.hasOwnProperty("type"))
                switch (message.type) {
                default:
                    return "type: enum value expected";
                case 0:
                case 1:
                    break;
                }
            if (message.value != null && message.hasOwnProperty("value"))
                if (!$util.isString(message.value))
                    return "value: string expected";
            return null;
        };

        /**
         * Creates a KeystoreCommonExistsParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.KeystoreCommonExistsParam} KeystoreCommonExistsParam
         */
        KeystoreCommonExistsParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.KeystoreCommonExistsParam)
                return object;
            var message = new $root.api.KeystoreCommonExistsParam();
            switch (object.type) {
            case "MNEMONIC":
            case 0:
                message.type = 0;
                break;
            case "PRIVATE_KEY":
            case 1:
                message.type = 1;
                break;
            }
            if (object.value != null)
                message.value = String(object.value);
            return message;
        };

        /**
         * Creates a plain object from a KeystoreCommonExistsParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.KeystoreCommonExistsParam
         * @static
         * @param {api.KeystoreCommonExistsParam} message KeystoreCommonExistsParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        KeystoreCommonExistsParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.type = options.enums === String ? "MNEMONIC" : 0;
                object.value = "";
            }
            if (message.type != null && message.hasOwnProperty("type"))
                object.type = options.enums === String ? $root.api.KeyType[message.type] : message.type;
            if (message.value != null && message.hasOwnProperty("value"))
                object.value = message.value;
            return object;
        };

        /**
         * Converts this KeystoreCommonExistsParam to JSON.
         * @function toJSON
         * @memberof api.KeystoreCommonExistsParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        KeystoreCommonExistsParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return KeystoreCommonExistsParam;
    })();

    api.KeystoreCommonExistsResult = (function() {

        /**
         * Properties of a KeystoreCommonExistsResult.
         * @memberof api
         * @interface IKeystoreCommonExistsResult
         * @property {boolean|null} [isExists] KeystoreCommonExistsResult isExists
         * @property {string|null} [id] KeystoreCommonExistsResult id
         */

        /**
         * Constructs a new KeystoreCommonExistsResult.
         * @memberof api
         * @classdesc Represents a KeystoreCommonExistsResult.
         * @implements IKeystoreCommonExistsResult
         * @constructor
         * @param {api.IKeystoreCommonExistsResult=} [properties] Properties to set
         */
        function KeystoreCommonExistsResult(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * KeystoreCommonExistsResult isExists.
         * @member {boolean} isExists
         * @memberof api.KeystoreCommonExistsResult
         * @instance
         */
        KeystoreCommonExistsResult.prototype.isExists = false;

        /**
         * KeystoreCommonExistsResult id.
         * @member {string} id
         * @memberof api.KeystoreCommonExistsResult
         * @instance
         */
        KeystoreCommonExistsResult.prototype.id = "";

        /**
         * Creates a new KeystoreCommonExistsResult instance using the specified properties.
         * @function create
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {api.IKeystoreCommonExistsResult=} [properties] Properties to set
         * @returns {api.KeystoreCommonExistsResult} KeystoreCommonExistsResult instance
         */
        KeystoreCommonExistsResult.create = function create(properties) {
            return new KeystoreCommonExistsResult(properties);
        };

        /**
         * Encodes the specified KeystoreCommonExistsResult message. Does not implicitly {@link api.KeystoreCommonExistsResult.verify|verify} messages.
         * @function encode
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {api.IKeystoreCommonExistsResult} message KeystoreCommonExistsResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExistsResult.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.isExists != null && message.hasOwnProperty("isExists"))
                writer.uint32(/* id 1, wireType 0 =*/8).bool(message.isExists);
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.id);
            return writer;
        };

        /**
         * Encodes the specified KeystoreCommonExistsResult message, length delimited. Does not implicitly {@link api.KeystoreCommonExistsResult.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {api.IKeystoreCommonExistsResult} message KeystoreCommonExistsResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonExistsResult.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a KeystoreCommonExistsResult message from the specified reader or buffer.
         * @function decode
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.KeystoreCommonExistsResult} KeystoreCommonExistsResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExistsResult.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonExistsResult();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.isExists = reader.bool();
                    break;
                case 2:
                    message.id = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a KeystoreCommonExistsResult message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.KeystoreCommonExistsResult} KeystoreCommonExistsResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonExistsResult.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a KeystoreCommonExistsResult message.
         * @function verify
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        KeystoreCommonExistsResult.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.isExists != null && message.hasOwnProperty("isExists"))
                if (typeof message.isExists !== "boolean")
                    return "isExists: boolean expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            return null;
        };

        /**
         * Creates a KeystoreCommonExistsResult message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.KeystoreCommonExistsResult} KeystoreCommonExistsResult
         */
        KeystoreCommonExistsResult.fromObject = function fromObject(object) {
            if (object instanceof $root.api.KeystoreCommonExistsResult)
                return object;
            var message = new $root.api.KeystoreCommonExistsResult();
            if (object.isExists != null)
                message.isExists = Boolean(object.isExists);
            if (object.id != null)
                message.id = String(object.id);
            return message;
        };

        /**
         * Creates a plain object from a KeystoreCommonExistsResult message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.KeystoreCommonExistsResult
         * @static
         * @param {api.KeystoreCommonExistsResult} message KeystoreCommonExistsResult
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        KeystoreCommonExistsResult.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.isExists = false;
                object.id = "";
            }
            if (message.isExists != null && message.hasOwnProperty("isExists"))
                object.isExists = message.isExists;
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            return object;
        };

        /**
         * Converts this KeystoreCommonExistsResult to JSON.
         * @function toJSON
         * @memberof api.KeystoreCommonExistsResult
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        KeystoreCommonExistsResult.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return KeystoreCommonExistsResult;
    })();

    api.KeystoreCommonAccountsParam = (function() {

        /**
         * Properties of a KeystoreCommonAccountsParam.
         * @memberof api
         * @interface IKeystoreCommonAccountsParam
         * @property {string|null} [id] KeystoreCommonAccountsParam id
         */

        /**
         * Constructs a new KeystoreCommonAccountsParam.
         * @memberof api
         * @classdesc Represents a KeystoreCommonAccountsParam.
         * @implements IKeystoreCommonAccountsParam
         * @constructor
         * @param {api.IKeystoreCommonAccountsParam=} [properties] Properties to set
         */
        function KeystoreCommonAccountsParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * KeystoreCommonAccountsParam id.
         * @member {string} id
         * @memberof api.KeystoreCommonAccountsParam
         * @instance
         */
        KeystoreCommonAccountsParam.prototype.id = "";

        /**
         * Creates a new KeystoreCommonAccountsParam instance using the specified properties.
         * @function create
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {api.IKeystoreCommonAccountsParam=} [properties] Properties to set
         * @returns {api.KeystoreCommonAccountsParam} KeystoreCommonAccountsParam instance
         */
        KeystoreCommonAccountsParam.create = function create(properties) {
            return new KeystoreCommonAccountsParam(properties);
        };

        /**
         * Encodes the specified KeystoreCommonAccountsParam message. Does not implicitly {@link api.KeystoreCommonAccountsParam.verify|verify} messages.
         * @function encode
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {api.IKeystoreCommonAccountsParam} message KeystoreCommonAccountsParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonAccountsParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            return writer;
        };

        /**
         * Encodes the specified KeystoreCommonAccountsParam message, length delimited. Does not implicitly {@link api.KeystoreCommonAccountsParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {api.IKeystoreCommonAccountsParam} message KeystoreCommonAccountsParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        KeystoreCommonAccountsParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a KeystoreCommonAccountsParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.KeystoreCommonAccountsParam} KeystoreCommonAccountsParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonAccountsParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.KeystoreCommonAccountsParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a KeystoreCommonAccountsParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.KeystoreCommonAccountsParam} KeystoreCommonAccountsParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        KeystoreCommonAccountsParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a KeystoreCommonAccountsParam message.
         * @function verify
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        KeystoreCommonAccountsParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            return null;
        };

        /**
         * Creates a KeystoreCommonAccountsParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.KeystoreCommonAccountsParam} KeystoreCommonAccountsParam
         */
        KeystoreCommonAccountsParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.KeystoreCommonAccountsParam)
                return object;
            var message = new $root.api.KeystoreCommonAccountsParam();
            if (object.id != null)
                message.id = String(object.id);
            return message;
        };

        /**
         * Creates a plain object from a KeystoreCommonAccountsParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.KeystoreCommonAccountsParam
         * @static
         * @param {api.KeystoreCommonAccountsParam} message KeystoreCommonAccountsParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        KeystoreCommonAccountsParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults)
                object.id = "";
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            return object;
        };

        /**
         * Converts this KeystoreCommonAccountsParam to JSON.
         * @function toJSON
         * @memberof api.KeystoreCommonAccountsParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        KeystoreCommonAccountsParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return KeystoreCommonAccountsParam;
    })();

    api.SignParam = (function() {

        /**
         * Properties of a SignParam.
         * @memberof api
         * @interface ISignParam
         * @property {string|null} [id] SignParam id
         * @property {string|null} [password] SignParam password
         * @property {string|null} [chainType] SignParam chainType
         * @property {string|null} [address] SignParam address
         * @property {google.protobuf.IAny|null} [input] SignParam input
         */

        /**
         * Constructs a new SignParam.
         * @memberof api
         * @classdesc Represents a SignParam.
         * @implements ISignParam
         * @constructor
         * @param {api.ISignParam=} [properties] Properties to set
         */
        function SignParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * SignParam id.
         * @member {string} id
         * @memberof api.SignParam
         * @instance
         */
        SignParam.prototype.id = "";

        /**
         * SignParam password.
         * @member {string} password
         * @memberof api.SignParam
         * @instance
         */
        SignParam.prototype.password = "";

        /**
         * SignParam chainType.
         * @member {string} chainType
         * @memberof api.SignParam
         * @instance
         */
        SignParam.prototype.chainType = "";

        /**
         * SignParam address.
         * @member {string} address
         * @memberof api.SignParam
         * @instance
         */
        SignParam.prototype.address = "";

        /**
         * SignParam input.
         * @member {google.protobuf.IAny|null|undefined} input
         * @memberof api.SignParam
         * @instance
         */
        SignParam.prototype.input = null;

        /**
         * Creates a new SignParam instance using the specified properties.
         * @function create
         * @memberof api.SignParam
         * @static
         * @param {api.ISignParam=} [properties] Properties to set
         * @returns {api.SignParam} SignParam instance
         */
        SignParam.create = function create(properties) {
            return new SignParam(properties);
        };

        /**
         * Encodes the specified SignParam message. Does not implicitly {@link api.SignParam.verify|verify} messages.
         * @function encode
         * @memberof api.SignParam
         * @static
         * @param {api.ISignParam} message SignParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        SignParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.chainType);
            if (message.address != null && message.hasOwnProperty("address"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.address);
            if (message.input != null && message.hasOwnProperty("input"))
                $root.google.protobuf.Any.encode(message.input, writer.uint32(/* id 5, wireType 2 =*/42).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified SignParam message, length delimited. Does not implicitly {@link api.SignParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.SignParam
         * @static
         * @param {api.ISignParam} message SignParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        SignParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a SignParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.SignParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.SignParam} SignParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        SignParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.SignParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    message.chainType = reader.string();
                    break;
                case 4:
                    message.address = reader.string();
                    break;
                case 5:
                    message.input = $root.google.protobuf.Any.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a SignParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.SignParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.SignParam} SignParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        SignParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a SignParam message.
         * @function verify
         * @memberof api.SignParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        SignParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                if (!$util.isString(message.chainType))
                    return "chainType: string expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            if (message.input != null && message.hasOwnProperty("input")) {
                var error = $root.google.protobuf.Any.verify(message.input);
                if (error)
                    return "input." + error;
            }
            return null;
        };

        /**
         * Creates a SignParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.SignParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.SignParam} SignParam
         */
        SignParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.SignParam)
                return object;
            var message = new $root.api.SignParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.password != null)
                message.password = String(object.password);
            if (object.chainType != null)
                message.chainType = String(object.chainType);
            if (object.address != null)
                message.address = String(object.address);
            if (object.input != null) {
                if (typeof object.input !== "object")
                    throw TypeError(".api.SignParam.input: object expected");
                message.input = $root.google.protobuf.Any.fromObject(object.input);
            }
            return message;
        };

        /**
         * Creates a plain object from a SignParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.SignParam
         * @static
         * @param {api.SignParam} message SignParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        SignParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.password = "";
                object.chainType = "";
                object.address = "";
                object.input = null;
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                object.chainType = message.chainType;
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            if (message.input != null && message.hasOwnProperty("input"))
                object.input = $root.google.protobuf.Any.toObject(message.input, options);
            return object;
        };

        /**
         * Converts this SignParam to JSON.
         * @function toJSON
         * @memberof api.SignParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        SignParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return SignParam;
    })();

    api.ExternalAddressParam = (function() {

        /**
         * Properties of an ExternalAddressParam.
         * @memberof api
         * @interface IExternalAddressParam
         * @property {string|null} [id] ExternalAddressParam id
         * @property {string|null} [chainType] ExternalAddressParam chainType
         * @property {number|null} [externalIdx] ExternalAddressParam externalIdx
         */

        /**
         * Constructs a new ExternalAddressParam.
         * @memberof api
         * @classdesc Represents an ExternalAddressParam.
         * @implements IExternalAddressParam
         * @constructor
         * @param {api.IExternalAddressParam=} [properties] Properties to set
         */
        function ExternalAddressParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * ExternalAddressParam id.
         * @member {string} id
         * @memberof api.ExternalAddressParam
         * @instance
         */
        ExternalAddressParam.prototype.id = "";

        /**
         * ExternalAddressParam chainType.
         * @member {string} chainType
         * @memberof api.ExternalAddressParam
         * @instance
         */
        ExternalAddressParam.prototype.chainType = "";

        /**
         * ExternalAddressParam externalIdx.
         * @member {number} externalIdx
         * @memberof api.ExternalAddressParam
         * @instance
         */
        ExternalAddressParam.prototype.externalIdx = 0;

        /**
         * Creates a new ExternalAddressParam instance using the specified properties.
         * @function create
         * @memberof api.ExternalAddressParam
         * @static
         * @param {api.IExternalAddressParam=} [properties] Properties to set
         * @returns {api.ExternalAddressParam} ExternalAddressParam instance
         */
        ExternalAddressParam.create = function create(properties) {
            return new ExternalAddressParam(properties);
        };

        /**
         * Encodes the specified ExternalAddressParam message. Does not implicitly {@link api.ExternalAddressParam.verify|verify} messages.
         * @function encode
         * @memberof api.ExternalAddressParam
         * @static
         * @param {api.IExternalAddressParam} message ExternalAddressParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.chainType);
            if (message.externalIdx != null && message.hasOwnProperty("externalIdx"))
                writer.uint32(/* id 3, wireType 0 =*/24).uint32(message.externalIdx);
            return writer;
        };

        /**
         * Encodes the specified ExternalAddressParam message, length delimited. Does not implicitly {@link api.ExternalAddressParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.ExternalAddressParam
         * @static
         * @param {api.IExternalAddressParam} message ExternalAddressParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an ExternalAddressParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.ExternalAddressParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.ExternalAddressParam} ExternalAddressParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.ExternalAddressParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.chainType = reader.string();
                    break;
                case 3:
                    message.externalIdx = reader.uint32();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an ExternalAddressParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.ExternalAddressParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.ExternalAddressParam} ExternalAddressParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an ExternalAddressParam message.
         * @function verify
         * @memberof api.ExternalAddressParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        ExternalAddressParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                if (!$util.isString(message.chainType))
                    return "chainType: string expected";
            if (message.externalIdx != null && message.hasOwnProperty("externalIdx"))
                if (!$util.isInteger(message.externalIdx))
                    return "externalIdx: integer expected";
            return null;
        };

        /**
         * Creates an ExternalAddressParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.ExternalAddressParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.ExternalAddressParam} ExternalAddressParam
         */
        ExternalAddressParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.ExternalAddressParam)
                return object;
            var message = new $root.api.ExternalAddressParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.chainType != null)
                message.chainType = String(object.chainType);
            if (object.externalIdx != null)
                message.externalIdx = object.externalIdx >>> 0;
            return message;
        };

        /**
         * Creates a plain object from an ExternalAddressParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.ExternalAddressParam
         * @static
         * @param {api.ExternalAddressParam} message ExternalAddressParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        ExternalAddressParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.chainType = "";
                object.externalIdx = 0;
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                object.chainType = message.chainType;
            if (message.externalIdx != null && message.hasOwnProperty("externalIdx"))
                object.externalIdx = message.externalIdx;
            return object;
        };

        /**
         * Converts this ExternalAddressParam to JSON.
         * @function toJSON
         * @memberof api.ExternalAddressParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        ExternalAddressParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return ExternalAddressParam;
    })();

    api.ExternalAddressResult = (function() {

        /**
         * Properties of an ExternalAddressResult.
         * @memberof api
         * @interface IExternalAddressResult
         * @property {string|null} [address] ExternalAddressResult address
         * @property {string|null} [derivedPath] ExternalAddressResult derivedPath
         * @property {string|null} [type] ExternalAddressResult type
         */

        /**
         * Constructs a new ExternalAddressResult.
         * @memberof api
         * @classdesc Represents an ExternalAddressResult.
         * @implements IExternalAddressResult
         * @constructor
         * @param {api.IExternalAddressResult=} [properties] Properties to set
         */
        function ExternalAddressResult(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * ExternalAddressResult address.
         * @member {string} address
         * @memberof api.ExternalAddressResult
         * @instance
         */
        ExternalAddressResult.prototype.address = "";

        /**
         * ExternalAddressResult derivedPath.
         * @member {string} derivedPath
         * @memberof api.ExternalAddressResult
         * @instance
         */
        ExternalAddressResult.prototype.derivedPath = "";

        /**
         * ExternalAddressResult type.
         * @member {string} type
         * @memberof api.ExternalAddressResult
         * @instance
         */
        ExternalAddressResult.prototype.type = "";

        /**
         * Creates a new ExternalAddressResult instance using the specified properties.
         * @function create
         * @memberof api.ExternalAddressResult
         * @static
         * @param {api.IExternalAddressResult=} [properties] Properties to set
         * @returns {api.ExternalAddressResult} ExternalAddressResult instance
         */
        ExternalAddressResult.create = function create(properties) {
            return new ExternalAddressResult(properties);
        };

        /**
         * Encodes the specified ExternalAddressResult message. Does not implicitly {@link api.ExternalAddressResult.verify|verify} messages.
         * @function encode
         * @memberof api.ExternalAddressResult
         * @static
         * @param {api.IExternalAddressResult} message ExternalAddressResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressResult.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.address != null && message.hasOwnProperty("address"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.address);
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.derivedPath);
            if (message.type != null && message.hasOwnProperty("type"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.type);
            return writer;
        };

        /**
         * Encodes the specified ExternalAddressResult message, length delimited. Does not implicitly {@link api.ExternalAddressResult.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.ExternalAddressResult
         * @static
         * @param {api.IExternalAddressResult} message ExternalAddressResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressResult.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an ExternalAddressResult message from the specified reader or buffer.
         * @function decode
         * @memberof api.ExternalAddressResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.ExternalAddressResult} ExternalAddressResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressResult.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.ExternalAddressResult();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.address = reader.string();
                    break;
                case 2:
                    message.derivedPath = reader.string();
                    break;
                case 3:
                    message.type = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an ExternalAddressResult message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.ExternalAddressResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.ExternalAddressResult} ExternalAddressResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressResult.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an ExternalAddressResult message.
         * @function verify
         * @memberof api.ExternalAddressResult
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        ExternalAddressResult.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                if (!$util.isString(message.derivedPath))
                    return "derivedPath: string expected";
            if (message.type != null && message.hasOwnProperty("type"))
                if (!$util.isString(message.type))
                    return "type: string expected";
            return null;
        };

        /**
         * Creates an ExternalAddressResult message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.ExternalAddressResult
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.ExternalAddressResult} ExternalAddressResult
         */
        ExternalAddressResult.fromObject = function fromObject(object) {
            if (object instanceof $root.api.ExternalAddressResult)
                return object;
            var message = new $root.api.ExternalAddressResult();
            if (object.address != null)
                message.address = String(object.address);
            if (object.derivedPath != null)
                message.derivedPath = String(object.derivedPath);
            if (object.type != null)
                message.type = String(object.type);
            return message;
        };

        /**
         * Creates a plain object from an ExternalAddressResult message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.ExternalAddressResult
         * @static
         * @param {api.ExternalAddressResult} message ExternalAddressResult
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        ExternalAddressResult.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.address = "";
                object.derivedPath = "";
                object.type = "";
            }
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                object.derivedPath = message.derivedPath;
            if (message.type != null && message.hasOwnProperty("type"))
                object.type = message.type;
            return object;
        };

        /**
         * Converts this ExternalAddressResult to JSON.
         * @function toJSON
         * @memberof api.ExternalAddressResult
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        ExternalAddressResult.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return ExternalAddressResult;
    })();

    api.ExternalAddressExtra = (function() {

        /**
         * Properties of an ExternalAddressExtra.
         * @memberof api
         * @interface IExternalAddressExtra
         * @property {string|null} [encXpub] ExternalAddressExtra encXpub
         * @property {api.ExternalAddressExtra.IExternalAddress|null} [externalAddress] ExternalAddressExtra externalAddress
         */

        /**
         * Constructs a new ExternalAddressExtra.
         * @memberof api
         * @classdesc Represents an ExternalAddressExtra.
         * @implements IExternalAddressExtra
         * @constructor
         * @param {api.IExternalAddressExtra=} [properties] Properties to set
         */
        function ExternalAddressExtra(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * ExternalAddressExtra encXpub.
         * @member {string} encXpub
         * @memberof api.ExternalAddressExtra
         * @instance
         */
        ExternalAddressExtra.prototype.encXpub = "";

        /**
         * ExternalAddressExtra externalAddress.
         * @member {api.ExternalAddressExtra.IExternalAddress|null|undefined} externalAddress
         * @memberof api.ExternalAddressExtra
         * @instance
         */
        ExternalAddressExtra.prototype.externalAddress = null;

        /**
         * Creates a new ExternalAddressExtra instance using the specified properties.
         * @function create
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {api.IExternalAddressExtra=} [properties] Properties to set
         * @returns {api.ExternalAddressExtra} ExternalAddressExtra instance
         */
        ExternalAddressExtra.create = function create(properties) {
            return new ExternalAddressExtra(properties);
        };

        /**
         * Encodes the specified ExternalAddressExtra message. Does not implicitly {@link api.ExternalAddressExtra.verify|verify} messages.
         * @function encode
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {api.IExternalAddressExtra} message ExternalAddressExtra message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressExtra.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.encXpub != null && message.hasOwnProperty("encXpub"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.encXpub);
            if (message.externalAddress != null && message.hasOwnProperty("externalAddress"))
                $root.api.ExternalAddressExtra.ExternalAddress.encode(message.externalAddress, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
            return writer;
        };

        /**
         * Encodes the specified ExternalAddressExtra message, length delimited. Does not implicitly {@link api.ExternalAddressExtra.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {api.IExternalAddressExtra} message ExternalAddressExtra message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        ExternalAddressExtra.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an ExternalAddressExtra message from the specified reader or buffer.
         * @function decode
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.ExternalAddressExtra} ExternalAddressExtra
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressExtra.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.ExternalAddressExtra();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.encXpub = reader.string();
                    break;
                case 2:
                    message.externalAddress = $root.api.ExternalAddressExtra.ExternalAddress.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an ExternalAddressExtra message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.ExternalAddressExtra} ExternalAddressExtra
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        ExternalAddressExtra.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an ExternalAddressExtra message.
         * @function verify
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        ExternalAddressExtra.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.encXpub != null && message.hasOwnProperty("encXpub"))
                if (!$util.isString(message.encXpub))
                    return "encXpub: string expected";
            if (message.externalAddress != null && message.hasOwnProperty("externalAddress")) {
                var error = $root.api.ExternalAddressExtra.ExternalAddress.verify(message.externalAddress);
                if (error)
                    return "externalAddress." + error;
            }
            return null;
        };

        /**
         * Creates an ExternalAddressExtra message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.ExternalAddressExtra} ExternalAddressExtra
         */
        ExternalAddressExtra.fromObject = function fromObject(object) {
            if (object instanceof $root.api.ExternalAddressExtra)
                return object;
            var message = new $root.api.ExternalAddressExtra();
            if (object.encXpub != null)
                message.encXpub = String(object.encXpub);
            if (object.externalAddress != null) {
                if (typeof object.externalAddress !== "object")
                    throw TypeError(".api.ExternalAddressExtra.externalAddress: object expected");
                message.externalAddress = $root.api.ExternalAddressExtra.ExternalAddress.fromObject(object.externalAddress);
            }
            return message;
        };

        /**
         * Creates a plain object from an ExternalAddressExtra message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.ExternalAddressExtra
         * @static
         * @param {api.ExternalAddressExtra} message ExternalAddressExtra
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        ExternalAddressExtra.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.encXpub = "";
                object.externalAddress = null;
            }
            if (message.encXpub != null && message.hasOwnProperty("encXpub"))
                object.encXpub = message.encXpub;
            if (message.externalAddress != null && message.hasOwnProperty("externalAddress"))
                object.externalAddress = $root.api.ExternalAddressExtra.ExternalAddress.toObject(message.externalAddress, options);
            return object;
        };

        /**
         * Converts this ExternalAddressExtra to JSON.
         * @function toJSON
         * @memberof api.ExternalAddressExtra
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        ExternalAddressExtra.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        ExternalAddressExtra.ExternalAddress = (function() {

            /**
             * Properties of an ExternalAddress.
             * @memberof api.ExternalAddressExtra
             * @interface IExternalAddress
             * @property {string|null} [address] ExternalAddress address
             * @property {string|null} [derivedPath] ExternalAddress derivedPath
             * @property {string|null} [type] ExternalAddress type
             */

            /**
             * Constructs a new ExternalAddress.
             * @memberof api.ExternalAddressExtra
             * @classdesc Represents an ExternalAddress.
             * @implements IExternalAddress
             * @constructor
             * @param {api.ExternalAddressExtra.IExternalAddress=} [properties] Properties to set
             */
            function ExternalAddress(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * ExternalAddress address.
             * @member {string} address
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @instance
             */
            ExternalAddress.prototype.address = "";

            /**
             * ExternalAddress derivedPath.
             * @member {string} derivedPath
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @instance
             */
            ExternalAddress.prototype.derivedPath = "";

            /**
             * ExternalAddress type.
             * @member {string} type
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @instance
             */
            ExternalAddress.prototype.type = "";

            /**
             * Creates a new ExternalAddress instance using the specified properties.
             * @function create
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {api.ExternalAddressExtra.IExternalAddress=} [properties] Properties to set
             * @returns {api.ExternalAddressExtra.ExternalAddress} ExternalAddress instance
             */
            ExternalAddress.create = function create(properties) {
                return new ExternalAddress(properties);
            };

            /**
             * Encodes the specified ExternalAddress message. Does not implicitly {@link api.ExternalAddressExtra.ExternalAddress.verify|verify} messages.
             * @function encode
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {api.ExternalAddressExtra.IExternalAddress} message ExternalAddress message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ExternalAddress.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.address != null && message.hasOwnProperty("address"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.address);
                if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                    writer.uint32(/* id 2, wireType 2 =*/18).string(message.derivedPath);
                if (message.type != null && message.hasOwnProperty("type"))
                    writer.uint32(/* id 3, wireType 2 =*/26).string(message.type);
                return writer;
            };

            /**
             * Encodes the specified ExternalAddress message, length delimited. Does not implicitly {@link api.ExternalAddressExtra.ExternalAddress.verify|verify} messages.
             * @function encodeDelimited
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {api.ExternalAddressExtra.IExternalAddress} message ExternalAddress message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ExternalAddress.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an ExternalAddress message from the specified reader or buffer.
             * @function decode
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {api.ExternalAddressExtra.ExternalAddress} ExternalAddress
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ExternalAddress.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.ExternalAddressExtra.ExternalAddress();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.address = reader.string();
                        break;
                    case 2:
                        message.derivedPath = reader.string();
                        break;
                    case 3:
                        message.type = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an ExternalAddress message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {api.ExternalAddressExtra.ExternalAddress} ExternalAddress
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ExternalAddress.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an ExternalAddress message.
             * @function verify
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ExternalAddress.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.address != null && message.hasOwnProperty("address"))
                    if (!$util.isString(message.address))
                        return "address: string expected";
                if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                    if (!$util.isString(message.derivedPath))
                        return "derivedPath: string expected";
                if (message.type != null && message.hasOwnProperty("type"))
                    if (!$util.isString(message.type))
                        return "type: string expected";
                return null;
            };

            /**
             * Creates an ExternalAddress message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {api.ExternalAddressExtra.ExternalAddress} ExternalAddress
             */
            ExternalAddress.fromObject = function fromObject(object) {
                if (object instanceof $root.api.ExternalAddressExtra.ExternalAddress)
                    return object;
                var message = new $root.api.ExternalAddressExtra.ExternalAddress();
                if (object.address != null)
                    message.address = String(object.address);
                if (object.derivedPath != null)
                    message.derivedPath = String(object.derivedPath);
                if (object.type != null)
                    message.type = String(object.type);
                return message;
            };

            /**
             * Creates a plain object from an ExternalAddress message. Also converts values to other types if specified.
             * @function toObject
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @static
             * @param {api.ExternalAddressExtra.ExternalAddress} message ExternalAddress
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ExternalAddress.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.address = "";
                    object.derivedPath = "";
                    object.type = "";
                }
                if (message.address != null && message.hasOwnProperty("address"))
                    object.address = message.address;
                if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                    object.derivedPath = message.derivedPath;
                if (message.type != null && message.hasOwnProperty("type"))
                    object.type = message.type;
                return object;
            };

            /**
             * Converts this ExternalAddress to JSON.
             * @function toJSON
             * @memberof api.ExternalAddressExtra.ExternalAddress
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ExternalAddress.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            return ExternalAddress;
        })();

        return ExternalAddressExtra;
    })();

    api.BtcForkDeriveExtraParam = (function() {

        /**
         * Properties of a BtcForkDeriveExtraParam.
         * @memberof api
         * @interface IBtcForkDeriveExtraParam
         * @property {string|null} [network] BtcForkDeriveExtraParam network
         * @property {string|null} [segWit] BtcForkDeriveExtraParam segWit
         */

        /**
         * Constructs a new BtcForkDeriveExtraParam.
         * @memberof api
         * @classdesc Represents a BtcForkDeriveExtraParam.
         * @implements IBtcForkDeriveExtraParam
         * @constructor
         * @param {api.IBtcForkDeriveExtraParam=} [properties] Properties to set
         */
        function BtcForkDeriveExtraParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BtcForkDeriveExtraParam network.
         * @member {string} network
         * @memberof api.BtcForkDeriveExtraParam
         * @instance
         */
        BtcForkDeriveExtraParam.prototype.network = "";

        /**
         * BtcForkDeriveExtraParam segWit.
         * @member {string} segWit
         * @memberof api.BtcForkDeriveExtraParam
         * @instance
         */
        BtcForkDeriveExtraParam.prototype.segWit = "";

        /**
         * Creates a new BtcForkDeriveExtraParam instance using the specified properties.
         * @function create
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {api.IBtcForkDeriveExtraParam=} [properties] Properties to set
         * @returns {api.BtcForkDeriveExtraParam} BtcForkDeriveExtraParam instance
         */
        BtcForkDeriveExtraParam.create = function create(properties) {
            return new BtcForkDeriveExtraParam(properties);
        };

        /**
         * Encodes the specified BtcForkDeriveExtraParam message. Does not implicitly {@link api.BtcForkDeriveExtraParam.verify|verify} messages.
         * @function encode
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {api.IBtcForkDeriveExtraParam} message BtcForkDeriveExtraParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkDeriveExtraParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.network != null && message.hasOwnProperty("network"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.network);
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.segWit);
            return writer;
        };

        /**
         * Encodes the specified BtcForkDeriveExtraParam message, length delimited. Does not implicitly {@link api.BtcForkDeriveExtraParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {api.IBtcForkDeriveExtraParam} message BtcForkDeriveExtraParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkDeriveExtraParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BtcForkDeriveExtraParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.BtcForkDeriveExtraParam} BtcForkDeriveExtraParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkDeriveExtraParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.BtcForkDeriveExtraParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.network = reader.string();
                    break;
                case 2:
                    message.segWit = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BtcForkDeriveExtraParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.BtcForkDeriveExtraParam} BtcForkDeriveExtraParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkDeriveExtraParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BtcForkDeriveExtraParam message.
         * @function verify
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BtcForkDeriveExtraParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.network != null && message.hasOwnProperty("network"))
                if (!$util.isString(message.network))
                    return "network: string expected";
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                if (!$util.isString(message.segWit))
                    return "segWit: string expected";
            return null;
        };

        /**
         * Creates a BtcForkDeriveExtraParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.BtcForkDeriveExtraParam} BtcForkDeriveExtraParam
         */
        BtcForkDeriveExtraParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.BtcForkDeriveExtraParam)
                return object;
            var message = new $root.api.BtcForkDeriveExtraParam();
            if (object.network != null)
                message.network = String(object.network);
            if (object.segWit != null)
                message.segWit = String(object.segWit);
            return message;
        };

        /**
         * Creates a plain object from a BtcForkDeriveExtraParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.BtcForkDeriveExtraParam
         * @static
         * @param {api.BtcForkDeriveExtraParam} message BtcForkDeriveExtraParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BtcForkDeriveExtraParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.network = "";
                object.segWit = "";
            }
            if (message.network != null && message.hasOwnProperty("network"))
                object.network = message.network;
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                object.segWit = message.segWit;
            return object;
        };

        /**
         * Converts this BtcForkDeriveExtraParam to JSON.
         * @function toJSON
         * @memberof api.BtcForkDeriveExtraParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BtcForkDeriveExtraParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return BtcForkDeriveExtraParam;
    })();

    api.HdStoreExtendedPublicKeyParam = (function() {

        /**
         * Properties of a HdStoreExtendedPublicKeyParam.
         * @memberof api
         * @interface IHdStoreExtendedPublicKeyParam
         * @property {string|null} [id] HdStoreExtendedPublicKeyParam id
         * @property {string|null} [password] HdStoreExtendedPublicKeyParam password
         * @property {string|null} [chainType] HdStoreExtendedPublicKeyParam chainType
         * @property {string|null} [address] HdStoreExtendedPublicKeyParam address
         */

        /**
         * Constructs a new HdStoreExtendedPublicKeyParam.
         * @memberof api
         * @classdesc Represents a HdStoreExtendedPublicKeyParam.
         * @implements IHdStoreExtendedPublicKeyParam
         * @constructor
         * @param {api.IHdStoreExtendedPublicKeyParam=} [properties] Properties to set
         */
        function HdStoreExtendedPublicKeyParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * HdStoreExtendedPublicKeyParam id.
         * @member {string} id
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @instance
         */
        HdStoreExtendedPublicKeyParam.prototype.id = "";

        /**
         * HdStoreExtendedPublicKeyParam password.
         * @member {string} password
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @instance
         */
        HdStoreExtendedPublicKeyParam.prototype.password = "";

        /**
         * HdStoreExtendedPublicKeyParam chainType.
         * @member {string} chainType
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @instance
         */
        HdStoreExtendedPublicKeyParam.prototype.chainType = "";

        /**
         * HdStoreExtendedPublicKeyParam address.
         * @member {string} address
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @instance
         */
        HdStoreExtendedPublicKeyParam.prototype.address = "";

        /**
         * Creates a new HdStoreExtendedPublicKeyParam instance using the specified properties.
         * @function create
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {api.IHdStoreExtendedPublicKeyParam=} [properties] Properties to set
         * @returns {api.HdStoreExtendedPublicKeyParam} HdStoreExtendedPublicKeyParam instance
         */
        HdStoreExtendedPublicKeyParam.create = function create(properties) {
            return new HdStoreExtendedPublicKeyParam(properties);
        };

        /**
         * Encodes the specified HdStoreExtendedPublicKeyParam message. Does not implicitly {@link api.HdStoreExtendedPublicKeyParam.verify|verify} messages.
         * @function encode
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {api.IHdStoreExtendedPublicKeyParam} message HdStoreExtendedPublicKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreExtendedPublicKeyParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.password != null && message.hasOwnProperty("password"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.password);
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.chainType);
            if (message.address != null && message.hasOwnProperty("address"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.address);
            return writer;
        };

        /**
         * Encodes the specified HdStoreExtendedPublicKeyParam message, length delimited. Does not implicitly {@link api.HdStoreExtendedPublicKeyParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {api.IHdStoreExtendedPublicKeyParam} message HdStoreExtendedPublicKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreExtendedPublicKeyParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a HdStoreExtendedPublicKeyParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.HdStoreExtendedPublicKeyParam} HdStoreExtendedPublicKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreExtendedPublicKeyParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.HdStoreExtendedPublicKeyParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.password = reader.string();
                    break;
                case 3:
                    message.chainType = reader.string();
                    break;
                case 4:
                    message.address = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a HdStoreExtendedPublicKeyParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.HdStoreExtendedPublicKeyParam} HdStoreExtendedPublicKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreExtendedPublicKeyParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a HdStoreExtendedPublicKeyParam message.
         * @function verify
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        HdStoreExtendedPublicKeyParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.password != null && message.hasOwnProperty("password"))
                if (!$util.isString(message.password))
                    return "password: string expected";
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                if (!$util.isString(message.chainType))
                    return "chainType: string expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            return null;
        };

        /**
         * Creates a HdStoreExtendedPublicKeyParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.HdStoreExtendedPublicKeyParam} HdStoreExtendedPublicKeyParam
         */
        HdStoreExtendedPublicKeyParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.HdStoreExtendedPublicKeyParam)
                return object;
            var message = new $root.api.HdStoreExtendedPublicKeyParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.password != null)
                message.password = String(object.password);
            if (object.chainType != null)
                message.chainType = String(object.chainType);
            if (object.address != null)
                message.address = String(object.address);
            return message;
        };

        /**
         * Creates a plain object from a HdStoreExtendedPublicKeyParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @static
         * @param {api.HdStoreExtendedPublicKeyParam} message HdStoreExtendedPublicKeyParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        HdStoreExtendedPublicKeyParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.password = "";
                object.chainType = "";
                object.address = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.password != null && message.hasOwnProperty("password"))
                object.password = message.password;
            if (message.chainType != null && message.hasOwnProperty("chainType"))
                object.chainType = message.chainType;
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            return object;
        };

        /**
         * Converts this HdStoreExtendedPublicKeyParam to JSON.
         * @function toJSON
         * @memberof api.HdStoreExtendedPublicKeyParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        HdStoreExtendedPublicKeyParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return HdStoreExtendedPublicKeyParam;
    })();

    api.HdStoreExtendedPublicKeyResponse = (function() {

        /**
         * Properties of a HdStoreExtendedPublicKeyResponse.
         * @memberof api
         * @interface IHdStoreExtendedPublicKeyResponse
         * @property {string|null} [extendedPublicKey] HdStoreExtendedPublicKeyResponse extendedPublicKey
         */

        /**
         * Constructs a new HdStoreExtendedPublicKeyResponse.
         * @memberof api
         * @classdesc Represents a HdStoreExtendedPublicKeyResponse.
         * @implements IHdStoreExtendedPublicKeyResponse
         * @constructor
         * @param {api.IHdStoreExtendedPublicKeyResponse=} [properties] Properties to set
         */
        function HdStoreExtendedPublicKeyResponse(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * HdStoreExtendedPublicKeyResponse extendedPublicKey.
         * @member {string} extendedPublicKey
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @instance
         */
        HdStoreExtendedPublicKeyResponse.prototype.extendedPublicKey = "";

        /**
         * Creates a new HdStoreExtendedPublicKeyResponse instance using the specified properties.
         * @function create
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {api.IHdStoreExtendedPublicKeyResponse=} [properties] Properties to set
         * @returns {api.HdStoreExtendedPublicKeyResponse} HdStoreExtendedPublicKeyResponse instance
         */
        HdStoreExtendedPublicKeyResponse.create = function create(properties) {
            return new HdStoreExtendedPublicKeyResponse(properties);
        };

        /**
         * Encodes the specified HdStoreExtendedPublicKeyResponse message. Does not implicitly {@link api.HdStoreExtendedPublicKeyResponse.verify|verify} messages.
         * @function encode
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {api.IHdStoreExtendedPublicKeyResponse} message HdStoreExtendedPublicKeyResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreExtendedPublicKeyResponse.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.extendedPublicKey != null && message.hasOwnProperty("extendedPublicKey"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.extendedPublicKey);
            return writer;
        };

        /**
         * Encodes the specified HdStoreExtendedPublicKeyResponse message, length delimited. Does not implicitly {@link api.HdStoreExtendedPublicKeyResponse.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {api.IHdStoreExtendedPublicKeyResponse} message HdStoreExtendedPublicKeyResponse message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        HdStoreExtendedPublicKeyResponse.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a HdStoreExtendedPublicKeyResponse message from the specified reader or buffer.
         * @function decode
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.HdStoreExtendedPublicKeyResponse} HdStoreExtendedPublicKeyResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreExtendedPublicKeyResponse.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.HdStoreExtendedPublicKeyResponse();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.extendedPublicKey = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a HdStoreExtendedPublicKeyResponse message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.HdStoreExtendedPublicKeyResponse} HdStoreExtendedPublicKeyResponse
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        HdStoreExtendedPublicKeyResponse.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a HdStoreExtendedPublicKeyResponse message.
         * @function verify
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        HdStoreExtendedPublicKeyResponse.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.extendedPublicKey != null && message.hasOwnProperty("extendedPublicKey"))
                if (!$util.isString(message.extendedPublicKey))
                    return "extendedPublicKey: string expected";
            return null;
        };

        /**
         * Creates a HdStoreExtendedPublicKeyResponse message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.HdStoreExtendedPublicKeyResponse} HdStoreExtendedPublicKeyResponse
         */
        HdStoreExtendedPublicKeyResponse.fromObject = function fromObject(object) {
            if (object instanceof $root.api.HdStoreExtendedPublicKeyResponse)
                return object;
            var message = new $root.api.HdStoreExtendedPublicKeyResponse();
            if (object.extendedPublicKey != null)
                message.extendedPublicKey = String(object.extendedPublicKey);
            return message;
        };

        /**
         * Creates a plain object from a HdStoreExtendedPublicKeyResponse message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @static
         * @param {api.HdStoreExtendedPublicKeyResponse} message HdStoreExtendedPublicKeyResponse
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        HdStoreExtendedPublicKeyResponse.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults)
                object.extendedPublicKey = "";
            if (message.extendedPublicKey != null && message.hasOwnProperty("extendedPublicKey"))
                object.extendedPublicKey = message.extendedPublicKey;
            return object;
        };

        /**
         * Converts this HdStoreExtendedPublicKeyResponse to JSON.
         * @function toJSON
         * @memberof api.HdStoreExtendedPublicKeyResponse
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        HdStoreExtendedPublicKeyResponse.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return HdStoreExtendedPublicKeyResponse;
    })();

    api.CacheDerivedKeyParam = (function() {

        /**
         * Properties of a CacheDerivedKeyParam.
         * @memberof api
         * @interface ICacheDerivedKeyParam
         * @property {string|null} [id] CacheDerivedKeyParam id
         * @property {string|null} [derivedKey] CacheDerivedKeyParam derivedKey
         * @property {string|null} [tempPassword] CacheDerivedKeyParam tempPassword
         */

        /**
         * Constructs a new CacheDerivedKeyParam.
         * @memberof api
         * @classdesc Represents a CacheDerivedKeyParam.
         * @implements ICacheDerivedKeyParam
         * @constructor
         * @param {api.ICacheDerivedKeyParam=} [properties] Properties to set
         */
        function CacheDerivedKeyParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * CacheDerivedKeyParam id.
         * @member {string} id
         * @memberof api.CacheDerivedKeyParam
         * @instance
         */
        CacheDerivedKeyParam.prototype.id = "";

        /**
         * CacheDerivedKeyParam derivedKey.
         * @member {string} derivedKey
         * @memberof api.CacheDerivedKeyParam
         * @instance
         */
        CacheDerivedKeyParam.prototype.derivedKey = "";

        /**
         * CacheDerivedKeyParam tempPassword.
         * @member {string} tempPassword
         * @memberof api.CacheDerivedKeyParam
         * @instance
         */
        CacheDerivedKeyParam.prototype.tempPassword = "";

        /**
         * Creates a new CacheDerivedKeyParam instance using the specified properties.
         * @function create
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {api.ICacheDerivedKeyParam=} [properties] Properties to set
         * @returns {api.CacheDerivedKeyParam} CacheDerivedKeyParam instance
         */
        CacheDerivedKeyParam.create = function create(properties) {
            return new CacheDerivedKeyParam(properties);
        };

        /**
         * Encodes the specified CacheDerivedKeyParam message. Does not implicitly {@link api.CacheDerivedKeyParam.verify|verify} messages.
         * @function encode
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {api.ICacheDerivedKeyParam} message CacheDerivedKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CacheDerivedKeyParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.derivedKey);
            if (message.tempPassword != null && message.hasOwnProperty("tempPassword"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.tempPassword);
            return writer;
        };

        /**
         * Encodes the specified CacheDerivedKeyParam message, length delimited. Does not implicitly {@link api.CacheDerivedKeyParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {api.ICacheDerivedKeyParam} message CacheDerivedKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CacheDerivedKeyParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a CacheDerivedKeyParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.CacheDerivedKeyParam} CacheDerivedKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CacheDerivedKeyParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.CacheDerivedKeyParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.derivedKey = reader.string();
                    break;
                case 3:
                    message.tempPassword = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a CacheDerivedKeyParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.CacheDerivedKeyParam} CacheDerivedKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CacheDerivedKeyParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a CacheDerivedKeyParam message.
         * @function verify
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CacheDerivedKeyParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                if (!$util.isString(message.derivedKey))
                    return "derivedKey: string expected";
            if (message.tempPassword != null && message.hasOwnProperty("tempPassword"))
                if (!$util.isString(message.tempPassword))
                    return "tempPassword: string expected";
            return null;
        };

        /**
         * Creates a CacheDerivedKeyParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.CacheDerivedKeyParam} CacheDerivedKeyParam
         */
        CacheDerivedKeyParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.CacheDerivedKeyParam)
                return object;
            var message = new $root.api.CacheDerivedKeyParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.derivedKey != null)
                message.derivedKey = String(object.derivedKey);
            if (object.tempPassword != null)
                message.tempPassword = String(object.tempPassword);
            return message;
        };

        /**
         * Creates a plain object from a CacheDerivedKeyParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.CacheDerivedKeyParam
         * @static
         * @param {api.CacheDerivedKeyParam} message CacheDerivedKeyParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CacheDerivedKeyParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.derivedKey = "";
                object.tempPassword = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                object.derivedKey = message.derivedKey;
            if (message.tempPassword != null && message.hasOwnProperty("tempPassword"))
                object.tempPassword = message.tempPassword;
            return object;
        };

        /**
         * Converts this CacheDerivedKeyParam to JSON.
         * @function toJSON
         * @memberof api.CacheDerivedKeyParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CacheDerivedKeyParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return CacheDerivedKeyParam;
    })();

    api.VerifyDerivedKeyParam = (function() {

        /**
         * Properties of a VerifyDerivedKeyParam.
         * @memberof api
         * @interface IVerifyDerivedKeyParam
         * @property {string|null} [id] VerifyDerivedKeyParam id
         * @property {string|null} [derivedKey] VerifyDerivedKeyParam derivedKey
         */

        /**
         * Constructs a new VerifyDerivedKeyParam.
         * @memberof api
         * @classdesc Represents a VerifyDerivedKeyParam.
         * @implements IVerifyDerivedKeyParam
         * @constructor
         * @param {api.IVerifyDerivedKeyParam=} [properties] Properties to set
         */
        function VerifyDerivedKeyParam(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * VerifyDerivedKeyParam id.
         * @member {string} id
         * @memberof api.VerifyDerivedKeyParam
         * @instance
         */
        VerifyDerivedKeyParam.prototype.id = "";

        /**
         * VerifyDerivedKeyParam derivedKey.
         * @member {string} derivedKey
         * @memberof api.VerifyDerivedKeyParam
         * @instance
         */
        VerifyDerivedKeyParam.prototype.derivedKey = "";

        /**
         * Creates a new VerifyDerivedKeyParam instance using the specified properties.
         * @function create
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {api.IVerifyDerivedKeyParam=} [properties] Properties to set
         * @returns {api.VerifyDerivedKeyParam} VerifyDerivedKeyParam instance
         */
        VerifyDerivedKeyParam.create = function create(properties) {
            return new VerifyDerivedKeyParam(properties);
        };

        /**
         * Encodes the specified VerifyDerivedKeyParam message. Does not implicitly {@link api.VerifyDerivedKeyParam.verify|verify} messages.
         * @function encode
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {api.IVerifyDerivedKeyParam} message VerifyDerivedKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        VerifyDerivedKeyParam.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.derivedKey);
            return writer;
        };

        /**
         * Encodes the specified VerifyDerivedKeyParam message, length delimited. Does not implicitly {@link api.VerifyDerivedKeyParam.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {api.IVerifyDerivedKeyParam} message VerifyDerivedKeyParam message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        VerifyDerivedKeyParam.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a VerifyDerivedKeyParam message from the specified reader or buffer.
         * @function decode
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.VerifyDerivedKeyParam} VerifyDerivedKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        VerifyDerivedKeyParam.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.VerifyDerivedKeyParam();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.derivedKey = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a VerifyDerivedKeyParam message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.VerifyDerivedKeyParam} VerifyDerivedKeyParam
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        VerifyDerivedKeyParam.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a VerifyDerivedKeyParam message.
         * @function verify
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        VerifyDerivedKeyParam.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                if (!$util.isString(message.derivedKey))
                    return "derivedKey: string expected";
            return null;
        };

        /**
         * Creates a VerifyDerivedKeyParam message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.VerifyDerivedKeyParam} VerifyDerivedKeyParam
         */
        VerifyDerivedKeyParam.fromObject = function fromObject(object) {
            if (object instanceof $root.api.VerifyDerivedKeyParam)
                return object;
            var message = new $root.api.VerifyDerivedKeyParam();
            if (object.id != null)
                message.id = String(object.id);
            if (object.derivedKey != null)
                message.derivedKey = String(object.derivedKey);
            return message;
        };

        /**
         * Creates a plain object from a VerifyDerivedKeyParam message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.VerifyDerivedKeyParam
         * @static
         * @param {api.VerifyDerivedKeyParam} message VerifyDerivedKeyParam
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        VerifyDerivedKeyParam.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.derivedKey = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                object.derivedKey = message.derivedKey;
            return object;
        };

        /**
         * Converts this VerifyDerivedKeyParam to JSON.
         * @function toJSON
         * @memberof api.VerifyDerivedKeyParam
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        VerifyDerivedKeyParam.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return VerifyDerivedKeyParam;
    })();

    api.DerivedKeyResult = (function() {

        /**
         * Properties of a DerivedKeyResult.
         * @memberof api
         * @interface IDerivedKeyResult
         * @property {string|null} [id] DerivedKeyResult id
         * @property {string|null} [derivedKey] DerivedKeyResult derivedKey
         */

        /**
         * Constructs a new DerivedKeyResult.
         * @memberof api
         * @classdesc Represents a DerivedKeyResult.
         * @implements IDerivedKeyResult
         * @constructor
         * @param {api.IDerivedKeyResult=} [properties] Properties to set
         */
        function DerivedKeyResult(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * DerivedKeyResult id.
         * @member {string} id
         * @memberof api.DerivedKeyResult
         * @instance
         */
        DerivedKeyResult.prototype.id = "";

        /**
         * DerivedKeyResult derivedKey.
         * @member {string} derivedKey
         * @memberof api.DerivedKeyResult
         * @instance
         */
        DerivedKeyResult.prototype.derivedKey = "";

        /**
         * Creates a new DerivedKeyResult instance using the specified properties.
         * @function create
         * @memberof api.DerivedKeyResult
         * @static
         * @param {api.IDerivedKeyResult=} [properties] Properties to set
         * @returns {api.DerivedKeyResult} DerivedKeyResult instance
         */
        DerivedKeyResult.create = function create(properties) {
            return new DerivedKeyResult(properties);
        };

        /**
         * Encodes the specified DerivedKeyResult message. Does not implicitly {@link api.DerivedKeyResult.verify|verify} messages.
         * @function encode
         * @memberof api.DerivedKeyResult
         * @static
         * @param {api.IDerivedKeyResult} message DerivedKeyResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        DerivedKeyResult.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.id != null && message.hasOwnProperty("id"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.id);
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.derivedKey);
            return writer;
        };

        /**
         * Encodes the specified DerivedKeyResult message, length delimited. Does not implicitly {@link api.DerivedKeyResult.verify|verify} messages.
         * @function encodeDelimited
         * @memberof api.DerivedKeyResult
         * @static
         * @param {api.IDerivedKeyResult} message DerivedKeyResult message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        DerivedKeyResult.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a DerivedKeyResult message from the specified reader or buffer.
         * @function decode
         * @memberof api.DerivedKeyResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {api.DerivedKeyResult} DerivedKeyResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        DerivedKeyResult.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.api.DerivedKeyResult();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.id = reader.string();
                    break;
                case 2:
                    message.derivedKey = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a DerivedKeyResult message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof api.DerivedKeyResult
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {api.DerivedKeyResult} DerivedKeyResult
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        DerivedKeyResult.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a DerivedKeyResult message.
         * @function verify
         * @memberof api.DerivedKeyResult
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        DerivedKeyResult.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.id != null && message.hasOwnProperty("id"))
                if (!$util.isString(message.id))
                    return "id: string expected";
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                if (!$util.isString(message.derivedKey))
                    return "derivedKey: string expected";
            return null;
        };

        /**
         * Creates a DerivedKeyResult message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof api.DerivedKeyResult
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {api.DerivedKeyResult} DerivedKeyResult
         */
        DerivedKeyResult.fromObject = function fromObject(object) {
            if (object instanceof $root.api.DerivedKeyResult)
                return object;
            var message = new $root.api.DerivedKeyResult();
            if (object.id != null)
                message.id = String(object.id);
            if (object.derivedKey != null)
                message.derivedKey = String(object.derivedKey);
            return message;
        };

        /**
         * Creates a plain object from a DerivedKeyResult message. Also converts values to other types if specified.
         * @function toObject
         * @memberof api.DerivedKeyResult
         * @static
         * @param {api.DerivedKeyResult} message DerivedKeyResult
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        DerivedKeyResult.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.id = "";
                object.derivedKey = "";
            }
            if (message.id != null && message.hasOwnProperty("id"))
                object.id = message.id;
            if (message.derivedKey != null && message.hasOwnProperty("derivedKey"))
                object.derivedKey = message.derivedKey;
            return object;
        };

        /**
         * Converts this DerivedKeyResult to JSON.
         * @function toJSON
         * @memberof api.DerivedKeyResult
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        DerivedKeyResult.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return DerivedKeyResult;
    })();

    return api;
})();

$root.google = (function() {

    /**
     * Namespace google.
     * @exports google
     * @namespace
     */
    var google = {};

    google.protobuf = (function() {

        /**
         * Namespace protobuf.
         * @memberof google
         * @namespace
         */
        var protobuf = {};

        protobuf.Any = (function() {

            /**
             * Properties of an Any.
             * @memberof google.protobuf
             * @interface IAny
             * @property {string|null} [type_url] Any type_url
             * @property {Uint8Array|null} [value] Any value
             */

            /**
             * Constructs a new Any.
             * @memberof google.protobuf
             * @classdesc Represents an Any.
             * @implements IAny
             * @constructor
             * @param {google.protobuf.IAny=} [properties] Properties to set
             */
            function Any(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }

            /**
             * Any type_url.
             * @member {string} type_url
             * @memberof google.protobuf.Any
             * @instance
             */
            Any.prototype.type_url = "";

            /**
             * Any value.
             * @member {Uint8Array} value
             * @memberof google.protobuf.Any
             * @instance
             */
            Any.prototype.value = $util.newBuffer([]);

            /**
             * Creates a new Any instance using the specified properties.
             * @function create
             * @memberof google.protobuf.Any
             * @static
             * @param {google.protobuf.IAny=} [properties] Properties to set
             * @returns {google.protobuf.Any} Any instance
             */
            Any.create = function create(properties) {
                return new Any(properties);
            };

            /**
             * Encodes the specified Any message. Does not implicitly {@link google.protobuf.Any.verify|verify} messages.
             * @function encode
             * @memberof google.protobuf.Any
             * @static
             * @param {google.protobuf.IAny} message Any message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Any.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.type_url != null && message.hasOwnProperty("type_url"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.type_url);
                if (message.value != null && message.hasOwnProperty("value"))
                    writer.uint32(/* id 2, wireType 2 =*/18).bytes(message.value);
                return writer;
            };

            /**
             * Encodes the specified Any message, length delimited. Does not implicitly {@link google.protobuf.Any.verify|verify} messages.
             * @function encodeDelimited
             * @memberof google.protobuf.Any
             * @static
             * @param {google.protobuf.IAny} message Any message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            Any.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };

            /**
             * Decodes an Any message from the specified reader or buffer.
             * @function decode
             * @memberof google.protobuf.Any
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {google.protobuf.Any} Any
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Any.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.google.protobuf.Any();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.type_url = reader.string();
                        break;
                    case 2:
                        message.value = reader.bytes();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };

            /**
             * Decodes an Any message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof google.protobuf.Any
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {google.protobuf.Any} Any
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            Any.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };

            /**
             * Verifies an Any message.
             * @function verify
             * @memberof google.protobuf.Any
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            Any.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.type_url != null && message.hasOwnProperty("type_url"))
                    if (!$util.isString(message.type_url))
                        return "type_url: string expected";
                if (message.value != null && message.hasOwnProperty("value"))
                    if (!(message.value && typeof message.value.length === "number" || $util.isString(message.value)))
                        return "value: buffer expected";
                return null;
            };

            /**
             * Creates an Any message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof google.protobuf.Any
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {google.protobuf.Any} Any
             */
            Any.fromObject = function fromObject(object) {
                if (object instanceof $root.google.protobuf.Any)
                    return object;
                var message = new $root.google.protobuf.Any();
                if (object.type_url != null)
                    message.type_url = String(object.type_url);
                if (object.value != null)
                    if (typeof object.value === "string")
                        $util.base64.decode(object.value, message.value = $util.newBuffer($util.base64.length(object.value)), 0);
                    else if (object.value.length)
                        message.value = object.value;
                return message;
            };

            /**
             * Creates a plain object from an Any message. Also converts values to other types if specified.
             * @function toObject
             * @memberof google.protobuf.Any
             * @static
             * @param {google.protobuf.Any} message Any
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            Any.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults) {
                    object.type_url = "";
                    if (options.bytes === String)
                        object.value = "";
                    else {
                        object.value = [];
                        if (options.bytes !== Array)
                            object.value = $util.newBuffer(object.value);
                    }
                }
                if (message.type_url != null && message.hasOwnProperty("type_url"))
                    object.type_url = message.type_url;
                if (message.value != null && message.hasOwnProperty("value"))
                    object.value = options.bytes === String ? $util.base64.encode(message.value, 0, message.value.length) : options.bytes === Array ? Array.prototype.slice.call(message.value) : message.value;
                return object;
            };

            /**
             * Converts this Any to JSON.
             * @function toJSON
             * @memberof google.protobuf.Any
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            Any.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };

            return Any;
        })();

        return protobuf;
    })();

    return google;
})();

$root.transaction = (function() {

    /**
     * Namespace transaction.
     * @exports transaction
     * @namespace
     */
    var transaction = {};

    transaction.Utxo = (function() {

        /**
         * Properties of an Utxo.
         * @memberof transaction
         * @interface IUtxo
         * @property {string|null} [txHash] Utxo txHash
         * @property {number|null} [vout] Utxo vout
         * @property {number|Long|null} [amount] Utxo amount
         * @property {string|null} [address] Utxo address
         * @property {string|null} [scriptPubKey] Utxo scriptPubKey
         * @property {string|null} [derivedPath] Utxo derivedPath
         * @property {number|Long|null} [sequence] Utxo sequence
         */

        /**
         * Constructs a new Utxo.
         * @memberof transaction
         * @classdesc Represents an Utxo.
         * @implements IUtxo
         * @constructor
         * @param {transaction.IUtxo=} [properties] Properties to set
         */
        function Utxo(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Utxo txHash.
         * @member {string} txHash
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.txHash = "";

        /**
         * Utxo vout.
         * @member {number} vout
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.vout = 0;

        /**
         * Utxo amount.
         * @member {number|Long} amount
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.amount = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Utxo address.
         * @member {string} address
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.address = "";

        /**
         * Utxo scriptPubKey.
         * @member {string} scriptPubKey
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.scriptPubKey = "";

        /**
         * Utxo derivedPath.
         * @member {string} derivedPath
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.derivedPath = "";

        /**
         * Utxo sequence.
         * @member {number|Long} sequence
         * @memberof transaction.Utxo
         * @instance
         */
        Utxo.prototype.sequence = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * Creates a new Utxo instance using the specified properties.
         * @function create
         * @memberof transaction.Utxo
         * @static
         * @param {transaction.IUtxo=} [properties] Properties to set
         * @returns {transaction.Utxo} Utxo instance
         */
        Utxo.create = function create(properties) {
            return new Utxo(properties);
        };

        /**
         * Encodes the specified Utxo message. Does not implicitly {@link transaction.Utxo.verify|verify} messages.
         * @function encode
         * @memberof transaction.Utxo
         * @static
         * @param {transaction.IUtxo} message Utxo message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Utxo.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.txHash);
            if (message.vout != null && message.hasOwnProperty("vout"))
                writer.uint32(/* id 2, wireType 0 =*/16).int32(message.vout);
            if (message.amount != null && message.hasOwnProperty("amount"))
                writer.uint32(/* id 3, wireType 0 =*/24).int64(message.amount);
            if (message.address != null && message.hasOwnProperty("address"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.address);
            if (message.scriptPubKey != null && message.hasOwnProperty("scriptPubKey"))
                writer.uint32(/* id 5, wireType 2 =*/42).string(message.scriptPubKey);
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                writer.uint32(/* id 6, wireType 2 =*/50).string(message.derivedPath);
            if (message.sequence != null && message.hasOwnProperty("sequence"))
                writer.uint32(/* id 7, wireType 0 =*/56).int64(message.sequence);
            return writer;
        };

        /**
         * Encodes the specified Utxo message, length delimited. Does not implicitly {@link transaction.Utxo.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.Utxo
         * @static
         * @param {transaction.IUtxo} message Utxo message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Utxo.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an Utxo message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.Utxo
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.Utxo} Utxo
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Utxo.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.Utxo();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.txHash = reader.string();
                    break;
                case 2:
                    message.vout = reader.int32();
                    break;
                case 3:
                    message.amount = reader.int64();
                    break;
                case 4:
                    message.address = reader.string();
                    break;
                case 5:
                    message.scriptPubKey = reader.string();
                    break;
                case 6:
                    message.derivedPath = reader.string();
                    break;
                case 7:
                    message.sequence = reader.int64();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an Utxo message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.Utxo
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.Utxo} Utxo
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Utxo.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an Utxo message.
         * @function verify
         * @memberof transaction.Utxo
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Utxo.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                if (!$util.isString(message.txHash))
                    return "txHash: string expected";
            if (message.vout != null && message.hasOwnProperty("vout"))
                if (!$util.isInteger(message.vout))
                    return "vout: integer expected";
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (!$util.isInteger(message.amount) && !(message.amount && $util.isInteger(message.amount.low) && $util.isInteger(message.amount.high)))
                    return "amount: integer|Long expected";
            if (message.address != null && message.hasOwnProperty("address"))
                if (!$util.isString(message.address))
                    return "address: string expected";
            if (message.scriptPubKey != null && message.hasOwnProperty("scriptPubKey"))
                if (!$util.isString(message.scriptPubKey))
                    return "scriptPubKey: string expected";
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                if (!$util.isString(message.derivedPath))
                    return "derivedPath: string expected";
            if (message.sequence != null && message.hasOwnProperty("sequence"))
                if (!$util.isInteger(message.sequence) && !(message.sequence && $util.isInteger(message.sequence.low) && $util.isInteger(message.sequence.high)))
                    return "sequence: integer|Long expected";
            return null;
        };

        /**
         * Creates an Utxo message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.Utxo
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.Utxo} Utxo
         */
        Utxo.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.Utxo)
                return object;
            var message = new $root.transaction.Utxo();
            if (object.txHash != null)
                message.txHash = String(object.txHash);
            if (object.vout != null)
                message.vout = object.vout | 0;
            if (object.amount != null)
                if ($util.Long)
                    (message.amount = $util.Long.fromValue(object.amount)).unsigned = false;
                else if (typeof object.amount === "string")
                    message.amount = parseInt(object.amount, 10);
                else if (typeof object.amount === "number")
                    message.amount = object.amount;
                else if (typeof object.amount === "object")
                    message.amount = new $util.LongBits(object.amount.low >>> 0, object.amount.high >>> 0).toNumber();
            if (object.address != null)
                message.address = String(object.address);
            if (object.scriptPubKey != null)
                message.scriptPubKey = String(object.scriptPubKey);
            if (object.derivedPath != null)
                message.derivedPath = String(object.derivedPath);
            if (object.sequence != null)
                if ($util.Long)
                    (message.sequence = $util.Long.fromValue(object.sequence)).unsigned = false;
                else if (typeof object.sequence === "string")
                    message.sequence = parseInt(object.sequence, 10);
                else if (typeof object.sequence === "number")
                    message.sequence = object.sequence;
                else if (typeof object.sequence === "object")
                    message.sequence = new $util.LongBits(object.sequence.low >>> 0, object.sequence.high >>> 0).toNumber();
            return message;
        };

        /**
         * Creates a plain object from an Utxo message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.Utxo
         * @static
         * @param {transaction.Utxo} message Utxo
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Utxo.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.txHash = "";
                object.vout = 0;
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.amount = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.amount = options.longs === String ? "0" : 0;
                object.address = "";
                object.scriptPubKey = "";
                object.derivedPath = "";
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.sequence = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.sequence = options.longs === String ? "0" : 0;
            }
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                object.txHash = message.txHash;
            if (message.vout != null && message.hasOwnProperty("vout"))
                object.vout = message.vout;
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (typeof message.amount === "number")
                    object.amount = options.longs === String ? String(message.amount) : message.amount;
                else
                    object.amount = options.longs === String ? $util.Long.prototype.toString.call(message.amount) : options.longs === Number ? new $util.LongBits(message.amount.low >>> 0, message.amount.high >>> 0).toNumber() : message.amount;
            if (message.address != null && message.hasOwnProperty("address"))
                object.address = message.address;
            if (message.scriptPubKey != null && message.hasOwnProperty("scriptPubKey"))
                object.scriptPubKey = message.scriptPubKey;
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                object.derivedPath = message.derivedPath;
            if (message.sequence != null && message.hasOwnProperty("sequence"))
                if (typeof message.sequence === "number")
                    object.sequence = options.longs === String ? String(message.sequence) : message.sequence;
                else
                    object.sequence = options.longs === String ? $util.Long.prototype.toString.call(message.sequence) : options.longs === Number ? new $util.LongBits(message.sequence.low >>> 0, message.sequence.high >>> 0).toNumber() : message.sequence;
            return object;
        };

        /**
         * Converts this Utxo to JSON.
         * @function toJSON
         * @memberof transaction.Utxo
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Utxo.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Utxo;
    })();

    transaction.BtcForkTxInput = (function() {

        /**
         * Properties of a BtcForkTxInput.
         * @memberof transaction
         * @interface IBtcForkTxInput
         * @property {string|null} [to] BtcForkTxInput to
         * @property {number|Long|null} [amount] BtcForkTxInput amount
         * @property {Array.<transaction.IUtxo>|null} [unspents] BtcForkTxInput unspents
         * @property {number|Long|null} [fee] BtcForkTxInput fee
         * @property {number|null} [changeAddressIndex] BtcForkTxInput changeAddressIndex
         * @property {string|null} [changeAddress] BtcForkTxInput changeAddress
         * @property {string|null} [network] BtcForkTxInput network
         * @property {string|null} [segWit] BtcForkTxInput segWit
         */

        /**
         * Constructs a new BtcForkTxInput.
         * @memberof transaction
         * @classdesc Represents a BtcForkTxInput.
         * @implements IBtcForkTxInput
         * @constructor
         * @param {transaction.IBtcForkTxInput=} [properties] Properties to set
         */
        function BtcForkTxInput(properties) {
            this.unspents = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BtcForkTxInput to.
         * @member {string} to
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.to = "";

        /**
         * BtcForkTxInput amount.
         * @member {number|Long} amount
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.amount = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * BtcForkTxInput unspents.
         * @member {Array.<transaction.IUtxo>} unspents
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.unspents = $util.emptyArray;

        /**
         * BtcForkTxInput fee.
         * @member {number|Long} fee
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.fee = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * BtcForkTxInput changeAddressIndex.
         * @member {number} changeAddressIndex
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.changeAddressIndex = 0;

        /**
         * BtcForkTxInput changeAddress.
         * @member {string} changeAddress
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.changeAddress = "";

        /**
         * BtcForkTxInput network.
         * @member {string} network
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.network = "";

        /**
         * BtcForkTxInput segWit.
         * @member {string} segWit
         * @memberof transaction.BtcForkTxInput
         * @instance
         */
        BtcForkTxInput.prototype.segWit = "";

        /**
         * Creates a new BtcForkTxInput instance using the specified properties.
         * @function create
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {transaction.IBtcForkTxInput=} [properties] Properties to set
         * @returns {transaction.BtcForkTxInput} BtcForkTxInput instance
         */
        BtcForkTxInput.create = function create(properties) {
            return new BtcForkTxInput(properties);
        };

        /**
         * Encodes the specified BtcForkTxInput message. Does not implicitly {@link transaction.BtcForkTxInput.verify|verify} messages.
         * @function encode
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {transaction.IBtcForkTxInput} message BtcForkTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkTxInput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.to != null && message.hasOwnProperty("to"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.to);
            if (message.amount != null && message.hasOwnProperty("amount"))
                writer.uint32(/* id 2, wireType 0 =*/16).int64(message.amount);
            if (message.unspents != null && message.unspents.length)
                for (var i = 0; i < message.unspents.length; ++i)
                    $root.transaction.Utxo.encode(message.unspents[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
            if (message.fee != null && message.hasOwnProperty("fee"))
                writer.uint32(/* id 4, wireType 0 =*/32).int64(message.fee);
            if (message.changeAddressIndex != null && message.hasOwnProperty("changeAddressIndex"))
                writer.uint32(/* id 5, wireType 0 =*/40).uint32(message.changeAddressIndex);
            if (message.changeAddress != null && message.hasOwnProperty("changeAddress"))
                writer.uint32(/* id 6, wireType 2 =*/50).string(message.changeAddress);
            if (message.network != null && message.hasOwnProperty("network"))
                writer.uint32(/* id 7, wireType 2 =*/58).string(message.network);
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                writer.uint32(/* id 8, wireType 2 =*/66).string(message.segWit);
            return writer;
        };

        /**
         * Encodes the specified BtcForkTxInput message, length delimited. Does not implicitly {@link transaction.BtcForkTxInput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {transaction.IBtcForkTxInput} message BtcForkTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkTxInput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BtcForkTxInput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.BtcForkTxInput} BtcForkTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkTxInput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.BtcForkTxInput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.to = reader.string();
                    break;
                case 2:
                    message.amount = reader.int64();
                    break;
                case 3:
                    if (!(message.unspents && message.unspents.length))
                        message.unspents = [];
                    message.unspents.push($root.transaction.Utxo.decode(reader, reader.uint32()));
                    break;
                case 4:
                    message.fee = reader.int64();
                    break;
                case 5:
                    message.changeAddressIndex = reader.uint32();
                    break;
                case 6:
                    message.changeAddress = reader.string();
                    break;
                case 7:
                    message.network = reader.string();
                    break;
                case 8:
                    message.segWit = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BtcForkTxInput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.BtcForkTxInput} BtcForkTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkTxInput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BtcForkTxInput message.
         * @function verify
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BtcForkTxInput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.to != null && message.hasOwnProperty("to"))
                if (!$util.isString(message.to))
                    return "to: string expected";
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (!$util.isInteger(message.amount) && !(message.amount && $util.isInteger(message.amount.low) && $util.isInteger(message.amount.high)))
                    return "amount: integer|Long expected";
            if (message.unspents != null && message.hasOwnProperty("unspents")) {
                if (!Array.isArray(message.unspents))
                    return "unspents: array expected";
                for (var i = 0; i < message.unspents.length; ++i) {
                    var error = $root.transaction.Utxo.verify(message.unspents[i]);
                    if (error)
                        return "unspents." + error;
                }
            }
            if (message.fee != null && message.hasOwnProperty("fee"))
                if (!$util.isInteger(message.fee) && !(message.fee && $util.isInteger(message.fee.low) && $util.isInteger(message.fee.high)))
                    return "fee: integer|Long expected";
            if (message.changeAddressIndex != null && message.hasOwnProperty("changeAddressIndex"))
                if (!$util.isInteger(message.changeAddressIndex))
                    return "changeAddressIndex: integer expected";
            if (message.changeAddress != null && message.hasOwnProperty("changeAddress"))
                if (!$util.isString(message.changeAddress))
                    return "changeAddress: string expected";
            if (message.network != null && message.hasOwnProperty("network"))
                if (!$util.isString(message.network))
                    return "network: string expected";
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                if (!$util.isString(message.segWit))
                    return "segWit: string expected";
            return null;
        };

        /**
         * Creates a BtcForkTxInput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.BtcForkTxInput} BtcForkTxInput
         */
        BtcForkTxInput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.BtcForkTxInput)
                return object;
            var message = new $root.transaction.BtcForkTxInput();
            if (object.to != null)
                message.to = String(object.to);
            if (object.amount != null)
                if ($util.Long)
                    (message.amount = $util.Long.fromValue(object.amount)).unsigned = false;
                else if (typeof object.amount === "string")
                    message.amount = parseInt(object.amount, 10);
                else if (typeof object.amount === "number")
                    message.amount = object.amount;
                else if (typeof object.amount === "object")
                    message.amount = new $util.LongBits(object.amount.low >>> 0, object.amount.high >>> 0).toNumber();
            if (object.unspents) {
                if (!Array.isArray(object.unspents))
                    throw TypeError(".transaction.BtcForkTxInput.unspents: array expected");
                message.unspents = [];
                for (var i = 0; i < object.unspents.length; ++i) {
                    if (typeof object.unspents[i] !== "object")
                        throw TypeError(".transaction.BtcForkTxInput.unspents: object expected");
                    message.unspents[i] = $root.transaction.Utxo.fromObject(object.unspents[i]);
                }
            }
            if (object.fee != null)
                if ($util.Long)
                    (message.fee = $util.Long.fromValue(object.fee)).unsigned = false;
                else if (typeof object.fee === "string")
                    message.fee = parseInt(object.fee, 10);
                else if (typeof object.fee === "number")
                    message.fee = object.fee;
                else if (typeof object.fee === "object")
                    message.fee = new $util.LongBits(object.fee.low >>> 0, object.fee.high >>> 0).toNumber();
            if (object.changeAddressIndex != null)
                message.changeAddressIndex = object.changeAddressIndex >>> 0;
            if (object.changeAddress != null)
                message.changeAddress = String(object.changeAddress);
            if (object.network != null)
                message.network = String(object.network);
            if (object.segWit != null)
                message.segWit = String(object.segWit);
            return message;
        };

        /**
         * Creates a plain object from a BtcForkTxInput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.BtcForkTxInput
         * @static
         * @param {transaction.BtcForkTxInput} message BtcForkTxInput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BtcForkTxInput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.unspents = [];
            if (options.defaults) {
                object.to = "";
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.amount = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.amount = options.longs === String ? "0" : 0;
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.fee = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.fee = options.longs === String ? "0" : 0;
                object.changeAddressIndex = 0;
                object.changeAddress = "";
                object.network = "";
                object.segWit = "";
            }
            if (message.to != null && message.hasOwnProperty("to"))
                object.to = message.to;
            if (message.amount != null && message.hasOwnProperty("amount"))
                if (typeof message.amount === "number")
                    object.amount = options.longs === String ? String(message.amount) : message.amount;
                else
                    object.amount = options.longs === String ? $util.Long.prototype.toString.call(message.amount) : options.longs === Number ? new $util.LongBits(message.amount.low >>> 0, message.amount.high >>> 0).toNumber() : message.amount;
            if (message.unspents && message.unspents.length) {
                object.unspents = [];
                for (var j = 0; j < message.unspents.length; ++j)
                    object.unspents[j] = $root.transaction.Utxo.toObject(message.unspents[j], options);
            }
            if (message.fee != null && message.hasOwnProperty("fee"))
                if (typeof message.fee === "number")
                    object.fee = options.longs === String ? String(message.fee) : message.fee;
                else
                    object.fee = options.longs === String ? $util.Long.prototype.toString.call(message.fee) : options.longs === Number ? new $util.LongBits(message.fee.low >>> 0, message.fee.high >>> 0).toNumber() : message.fee;
            if (message.changeAddressIndex != null && message.hasOwnProperty("changeAddressIndex"))
                object.changeAddressIndex = message.changeAddressIndex;
            if (message.changeAddress != null && message.hasOwnProperty("changeAddress"))
                object.changeAddress = message.changeAddress;
            if (message.network != null && message.hasOwnProperty("network"))
                object.network = message.network;
            if (message.segWit != null && message.hasOwnProperty("segWit"))
                object.segWit = message.segWit;
            return object;
        };

        /**
         * Converts this BtcForkTxInput to JSON.
         * @function toJSON
         * @memberof transaction.BtcForkTxInput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BtcForkTxInput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return BtcForkTxInput;
    })();

    transaction.BtcForkSignedTxOutput = (function() {

        /**
         * Properties of a BtcForkSignedTxOutput.
         * @memberof transaction
         * @interface IBtcForkSignedTxOutput
         * @property {string|null} [signature] BtcForkSignedTxOutput signature
         * @property {string|null} [txHash] BtcForkSignedTxOutput txHash
         */

        /**
         * Constructs a new BtcForkSignedTxOutput.
         * @memberof transaction
         * @classdesc Represents a BtcForkSignedTxOutput.
         * @implements IBtcForkSignedTxOutput
         * @constructor
         * @param {transaction.IBtcForkSignedTxOutput=} [properties] Properties to set
         */
        function BtcForkSignedTxOutput(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * BtcForkSignedTxOutput signature.
         * @member {string} signature
         * @memberof transaction.BtcForkSignedTxOutput
         * @instance
         */
        BtcForkSignedTxOutput.prototype.signature = "";

        /**
         * BtcForkSignedTxOutput txHash.
         * @member {string} txHash
         * @memberof transaction.BtcForkSignedTxOutput
         * @instance
         */
        BtcForkSignedTxOutput.prototype.txHash = "";

        /**
         * Creates a new BtcForkSignedTxOutput instance using the specified properties.
         * @function create
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {transaction.IBtcForkSignedTxOutput=} [properties] Properties to set
         * @returns {transaction.BtcForkSignedTxOutput} BtcForkSignedTxOutput instance
         */
        BtcForkSignedTxOutput.create = function create(properties) {
            return new BtcForkSignedTxOutput(properties);
        };

        /**
         * Encodes the specified BtcForkSignedTxOutput message. Does not implicitly {@link transaction.BtcForkSignedTxOutput.verify|verify} messages.
         * @function encode
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {transaction.IBtcForkSignedTxOutput} message BtcForkSignedTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkSignedTxOutput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.signature != null && message.hasOwnProperty("signature"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.signature);
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.txHash);
            return writer;
        };

        /**
         * Encodes the specified BtcForkSignedTxOutput message, length delimited. Does not implicitly {@link transaction.BtcForkSignedTxOutput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {transaction.IBtcForkSignedTxOutput} message BtcForkSignedTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        BtcForkSignedTxOutput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a BtcForkSignedTxOutput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.BtcForkSignedTxOutput} BtcForkSignedTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkSignedTxOutput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.BtcForkSignedTxOutput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.signature = reader.string();
                    break;
                case 2:
                    message.txHash = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a BtcForkSignedTxOutput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.BtcForkSignedTxOutput} BtcForkSignedTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        BtcForkSignedTxOutput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a BtcForkSignedTxOutput message.
         * @function verify
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        BtcForkSignedTxOutput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.signature != null && message.hasOwnProperty("signature"))
                if (!$util.isString(message.signature))
                    return "signature: string expected";
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                if (!$util.isString(message.txHash))
                    return "txHash: string expected";
            return null;
        };

        /**
         * Creates a BtcForkSignedTxOutput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.BtcForkSignedTxOutput} BtcForkSignedTxOutput
         */
        BtcForkSignedTxOutput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.BtcForkSignedTxOutput)
                return object;
            var message = new $root.transaction.BtcForkSignedTxOutput();
            if (object.signature != null)
                message.signature = String(object.signature);
            if (object.txHash != null)
                message.txHash = String(object.txHash);
            return message;
        };

        /**
         * Creates a plain object from a BtcForkSignedTxOutput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.BtcForkSignedTxOutput
         * @static
         * @param {transaction.BtcForkSignedTxOutput} message BtcForkSignedTxOutput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        BtcForkSignedTxOutput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.signature = "";
                object.txHash = "";
            }
            if (message.signature != null && message.hasOwnProperty("signature"))
                object.signature = message.signature;
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                object.txHash = message.txHash;
            return object;
        };

        /**
         * Converts this BtcForkSignedTxOutput to JSON.
         * @function toJSON
         * @memberof transaction.BtcForkSignedTxOutput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        BtcForkSignedTxOutput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return BtcForkSignedTxOutput;
    })();

    transaction.OutPoint = (function() {

        /**
         * Properties of an OutPoint.
         * @memberof transaction
         * @interface IOutPoint
         * @property {string|null} [txHash] OutPoint txHash
         * @property {number|null} [index] OutPoint index
         */

        /**
         * Constructs a new OutPoint.
         * @memberof transaction
         * @classdesc Represents an OutPoint.
         * @implements IOutPoint
         * @constructor
         * @param {transaction.IOutPoint=} [properties] Properties to set
         */
        function OutPoint(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * OutPoint txHash.
         * @member {string} txHash
         * @memberof transaction.OutPoint
         * @instance
         */
        OutPoint.prototype.txHash = "";

        /**
         * OutPoint index.
         * @member {number} index
         * @memberof transaction.OutPoint
         * @instance
         */
        OutPoint.prototype.index = 0;

        /**
         * Creates a new OutPoint instance using the specified properties.
         * @function create
         * @memberof transaction.OutPoint
         * @static
         * @param {transaction.IOutPoint=} [properties] Properties to set
         * @returns {transaction.OutPoint} OutPoint instance
         */
        OutPoint.create = function create(properties) {
            return new OutPoint(properties);
        };

        /**
         * Encodes the specified OutPoint message. Does not implicitly {@link transaction.OutPoint.verify|verify} messages.
         * @function encode
         * @memberof transaction.OutPoint
         * @static
         * @param {transaction.IOutPoint} message OutPoint message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        OutPoint.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.txHash);
            if (message.index != null && message.hasOwnProperty("index"))
                writer.uint32(/* id 2, wireType 0 =*/16).int32(message.index);
            return writer;
        };

        /**
         * Encodes the specified OutPoint message, length delimited. Does not implicitly {@link transaction.OutPoint.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.OutPoint
         * @static
         * @param {transaction.IOutPoint} message OutPoint message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        OutPoint.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes an OutPoint message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.OutPoint
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.OutPoint} OutPoint
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        OutPoint.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.OutPoint();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.txHash = reader.string();
                    break;
                case 2:
                    message.index = reader.int32();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes an OutPoint message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.OutPoint
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.OutPoint} OutPoint
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        OutPoint.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies an OutPoint message.
         * @function verify
         * @memberof transaction.OutPoint
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        OutPoint.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                if (!$util.isString(message.txHash))
                    return "txHash: string expected";
            if (message.index != null && message.hasOwnProperty("index"))
                if (!$util.isInteger(message.index))
                    return "index: integer expected";
            return null;
        };

        /**
         * Creates an OutPoint message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.OutPoint
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.OutPoint} OutPoint
         */
        OutPoint.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.OutPoint)
                return object;
            var message = new $root.transaction.OutPoint();
            if (object.txHash != null)
                message.txHash = String(object.txHash);
            if (object.index != null)
                message.index = object.index | 0;
            return message;
        };

        /**
         * Creates a plain object from an OutPoint message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.OutPoint
         * @static
         * @param {transaction.OutPoint} message OutPoint
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        OutPoint.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.txHash = "";
                object.index = 0;
            }
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                object.txHash = message.txHash;
            if (message.index != null && message.hasOwnProperty("index"))
                object.index = message.index;
            return object;
        };

        /**
         * Converts this OutPoint to JSON.
         * @function toJSON
         * @memberof transaction.OutPoint
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        OutPoint.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return OutPoint;
    })();

    transaction.Witness = (function() {

        /**
         * Properties of a Witness.
         * @memberof transaction
         * @interface IWitness
         * @property {string|null} [lock] Witness lock
         * @property {string|null} [inputType] Witness inputType
         * @property {string|null} [outputType] Witness outputType
         */

        /**
         * Constructs a new Witness.
         * @memberof transaction
         * @classdesc Represents a Witness.
         * @implements IWitness
         * @constructor
         * @param {transaction.IWitness=} [properties] Properties to set
         */
        function Witness(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Witness lock.
         * @member {string} lock
         * @memberof transaction.Witness
         * @instance
         */
        Witness.prototype.lock = "";

        /**
         * Witness inputType.
         * @member {string} inputType
         * @memberof transaction.Witness
         * @instance
         */
        Witness.prototype.inputType = "";

        /**
         * Witness outputType.
         * @member {string} outputType
         * @memberof transaction.Witness
         * @instance
         */
        Witness.prototype.outputType = "";

        /**
         * Creates a new Witness instance using the specified properties.
         * @function create
         * @memberof transaction.Witness
         * @static
         * @param {transaction.IWitness=} [properties] Properties to set
         * @returns {transaction.Witness} Witness instance
         */
        Witness.create = function create(properties) {
            return new Witness(properties);
        };

        /**
         * Encodes the specified Witness message. Does not implicitly {@link transaction.Witness.verify|verify} messages.
         * @function encode
         * @memberof transaction.Witness
         * @static
         * @param {transaction.IWitness} message Witness message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Witness.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.lock != null && message.hasOwnProperty("lock"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.lock);
            if (message.inputType != null && message.hasOwnProperty("inputType"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.inputType);
            if (message.outputType != null && message.hasOwnProperty("outputType"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.outputType);
            return writer;
        };

        /**
         * Encodes the specified Witness message, length delimited. Does not implicitly {@link transaction.Witness.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.Witness
         * @static
         * @param {transaction.IWitness} message Witness message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Witness.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Witness message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.Witness
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.Witness} Witness
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Witness.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.Witness();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.lock = reader.string();
                    break;
                case 2:
                    message.inputType = reader.string();
                    break;
                case 3:
                    message.outputType = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Witness message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.Witness
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.Witness} Witness
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Witness.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Witness message.
         * @function verify
         * @memberof transaction.Witness
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Witness.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.lock != null && message.hasOwnProperty("lock"))
                if (!$util.isString(message.lock))
                    return "lock: string expected";
            if (message.inputType != null && message.hasOwnProperty("inputType"))
                if (!$util.isString(message.inputType))
                    return "inputType: string expected";
            if (message.outputType != null && message.hasOwnProperty("outputType"))
                if (!$util.isString(message.outputType))
                    return "outputType: string expected";
            return null;
        };

        /**
         * Creates a Witness message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.Witness
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.Witness} Witness
         */
        Witness.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.Witness)
                return object;
            var message = new $root.transaction.Witness();
            if (object.lock != null)
                message.lock = String(object.lock);
            if (object.inputType != null)
                message.inputType = String(object.inputType);
            if (object.outputType != null)
                message.outputType = String(object.outputType);
            return message;
        };

        /**
         * Creates a plain object from a Witness message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.Witness
         * @static
         * @param {transaction.Witness} message Witness
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Witness.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.lock = "";
                object.inputType = "";
                object.outputType = "";
            }
            if (message.lock != null && message.hasOwnProperty("lock"))
                object.lock = message.lock;
            if (message.inputType != null && message.hasOwnProperty("inputType"))
                object.inputType = message.inputType;
            if (message.outputType != null && message.hasOwnProperty("outputType"))
                object.outputType = message.outputType;
            return object;
        };

        /**
         * Converts this Witness to JSON.
         * @function toJSON
         * @memberof transaction.Witness
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Witness.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Witness;
    })();

    transaction.Script = (function() {

        /**
         * Properties of a Script.
         * @memberof transaction
         * @interface IScript
         * @property {string|null} [args] Script args
         * @property {string|null} [codeHash] Script codeHash
         * @property {string|null} [hashType] Script hashType
         */

        /**
         * Constructs a new Script.
         * @memberof transaction
         * @classdesc Represents a Script.
         * @implements IScript
         * @constructor
         * @param {transaction.IScript=} [properties] Properties to set
         */
        function Script(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * Script args.
         * @member {string} args
         * @memberof transaction.Script
         * @instance
         */
        Script.prototype.args = "";

        /**
         * Script codeHash.
         * @member {string} codeHash
         * @memberof transaction.Script
         * @instance
         */
        Script.prototype.codeHash = "";

        /**
         * Script hashType.
         * @member {string} hashType
         * @memberof transaction.Script
         * @instance
         */
        Script.prototype.hashType = "";

        /**
         * Creates a new Script instance using the specified properties.
         * @function create
         * @memberof transaction.Script
         * @static
         * @param {transaction.IScript=} [properties] Properties to set
         * @returns {transaction.Script} Script instance
         */
        Script.create = function create(properties) {
            return new Script(properties);
        };

        /**
         * Encodes the specified Script message. Does not implicitly {@link transaction.Script.verify|verify} messages.
         * @function encode
         * @memberof transaction.Script
         * @static
         * @param {transaction.IScript} message Script message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Script.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.args != null && message.hasOwnProperty("args"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.args);
            if (message.codeHash != null && message.hasOwnProperty("codeHash"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.codeHash);
            if (message.hashType != null && message.hasOwnProperty("hashType"))
                writer.uint32(/* id 3, wireType 2 =*/26).string(message.hashType);
            return writer;
        };

        /**
         * Encodes the specified Script message, length delimited. Does not implicitly {@link transaction.Script.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.Script
         * @static
         * @param {transaction.IScript} message Script message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        Script.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a Script message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.Script
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.Script} Script
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Script.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.Script();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.args = reader.string();
                    break;
                case 2:
                    message.codeHash = reader.string();
                    break;
                case 3:
                    message.hashType = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a Script message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.Script
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.Script} Script
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        Script.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a Script message.
         * @function verify
         * @memberof transaction.Script
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        Script.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.args != null && message.hasOwnProperty("args"))
                if (!$util.isString(message.args))
                    return "args: string expected";
            if (message.codeHash != null && message.hasOwnProperty("codeHash"))
                if (!$util.isString(message.codeHash))
                    return "codeHash: string expected";
            if (message.hashType != null && message.hasOwnProperty("hashType"))
                if (!$util.isString(message.hashType))
                    return "hashType: string expected";
            return null;
        };

        /**
         * Creates a Script message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.Script
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.Script} Script
         */
        Script.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.Script)
                return object;
            var message = new $root.transaction.Script();
            if (object.args != null)
                message.args = String(object.args);
            if (object.codeHash != null)
                message.codeHash = String(object.codeHash);
            if (object.hashType != null)
                message.hashType = String(object.hashType);
            return message;
        };

        /**
         * Creates a plain object from a Script message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.Script
         * @static
         * @param {transaction.Script} message Script
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        Script.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.args = "";
                object.codeHash = "";
                object.hashType = "";
            }
            if (message.args != null && message.hasOwnProperty("args"))
                object.args = message.args;
            if (message.codeHash != null && message.hasOwnProperty("codeHash"))
                object.codeHash = message.codeHash;
            if (message.hashType != null && message.hasOwnProperty("hashType"))
                object.hashType = message.hashType;
            return object;
        };

        /**
         * Converts this Script to JSON.
         * @function toJSON
         * @memberof transaction.Script
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        Script.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return Script;
    })();

    transaction.CellInput = (function() {

        /**
         * Properties of a CellInput.
         * @memberof transaction
         * @interface ICellInput
         * @property {transaction.IOutPoint|null} [previousOutput] CellInput previousOutput
         * @property {string|null} [since] CellInput since
         */

        /**
         * Constructs a new CellInput.
         * @memberof transaction
         * @classdesc Represents a CellInput.
         * @implements ICellInput
         * @constructor
         * @param {transaction.ICellInput=} [properties] Properties to set
         */
        function CellInput(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * CellInput previousOutput.
         * @member {transaction.IOutPoint|null|undefined} previousOutput
         * @memberof transaction.CellInput
         * @instance
         */
        CellInput.prototype.previousOutput = null;

        /**
         * CellInput since.
         * @member {string} since
         * @memberof transaction.CellInput
         * @instance
         */
        CellInput.prototype.since = "";

        /**
         * Creates a new CellInput instance using the specified properties.
         * @function create
         * @memberof transaction.CellInput
         * @static
         * @param {transaction.ICellInput=} [properties] Properties to set
         * @returns {transaction.CellInput} CellInput instance
         */
        CellInput.create = function create(properties) {
            return new CellInput(properties);
        };

        /**
         * Encodes the specified CellInput message. Does not implicitly {@link transaction.CellInput.verify|verify} messages.
         * @function encode
         * @memberof transaction.CellInput
         * @static
         * @param {transaction.ICellInput} message CellInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CellInput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.previousOutput != null && message.hasOwnProperty("previousOutput"))
                $root.transaction.OutPoint.encode(message.previousOutput, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
            if (message.since != null && message.hasOwnProperty("since"))
                writer.uint32(/* id 2, wireType 2 =*/18).string(message.since);
            return writer;
        };

        /**
         * Encodes the specified CellInput message, length delimited. Does not implicitly {@link transaction.CellInput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.CellInput
         * @static
         * @param {transaction.ICellInput} message CellInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CellInput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a CellInput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.CellInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.CellInput} CellInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CellInput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.CellInput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.previousOutput = $root.transaction.OutPoint.decode(reader, reader.uint32());
                    break;
                case 2:
                    message.since = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a CellInput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.CellInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.CellInput} CellInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CellInput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a CellInput message.
         * @function verify
         * @memberof transaction.CellInput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CellInput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.previousOutput != null && message.hasOwnProperty("previousOutput")) {
                var error = $root.transaction.OutPoint.verify(message.previousOutput);
                if (error)
                    return "previousOutput." + error;
            }
            if (message.since != null && message.hasOwnProperty("since"))
                if (!$util.isString(message.since))
                    return "since: string expected";
            return null;
        };

        /**
         * Creates a CellInput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.CellInput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.CellInput} CellInput
         */
        CellInput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.CellInput)
                return object;
            var message = new $root.transaction.CellInput();
            if (object.previousOutput != null) {
                if (typeof object.previousOutput !== "object")
                    throw TypeError(".transaction.CellInput.previousOutput: object expected");
                message.previousOutput = $root.transaction.OutPoint.fromObject(object.previousOutput);
            }
            if (object.since != null)
                message.since = String(object.since);
            return message;
        };

        /**
         * Creates a plain object from a CellInput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.CellInput
         * @static
         * @param {transaction.CellInput} message CellInput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CellInput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.previousOutput = null;
                object.since = "";
            }
            if (message.previousOutput != null && message.hasOwnProperty("previousOutput"))
                object.previousOutput = $root.transaction.OutPoint.toObject(message.previousOutput, options);
            if (message.since != null && message.hasOwnProperty("since"))
                object.since = message.since;
            return object;
        };

        /**
         * Converts this CellInput to JSON.
         * @function toJSON
         * @memberof transaction.CellInput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CellInput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return CellInput;
    })();

    transaction.CachedCell = (function() {

        /**
         * Properties of a CachedCell.
         * @memberof transaction
         * @interface ICachedCell
         * @property {number|Long|null} [capacity] CachedCell capacity
         * @property {transaction.IScript|null} [lock] CachedCell lock
         * @property {transaction.IOutPoint|null} [outPoint] CachedCell outPoint
         * @property {string|null} [derivedPath] CachedCell derivedPath
         */

        /**
         * Constructs a new CachedCell.
         * @memberof transaction
         * @classdesc Represents a CachedCell.
         * @implements ICachedCell
         * @constructor
         * @param {transaction.ICachedCell=} [properties] Properties to set
         */
        function CachedCell(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * CachedCell capacity.
         * @member {number|Long} capacity
         * @memberof transaction.CachedCell
         * @instance
         */
        CachedCell.prototype.capacity = $util.Long ? $util.Long.fromBits(0,0,false) : 0;

        /**
         * CachedCell lock.
         * @member {transaction.IScript|null|undefined} lock
         * @memberof transaction.CachedCell
         * @instance
         */
        CachedCell.prototype.lock = null;

        /**
         * CachedCell outPoint.
         * @member {transaction.IOutPoint|null|undefined} outPoint
         * @memberof transaction.CachedCell
         * @instance
         */
        CachedCell.prototype.outPoint = null;

        /**
         * CachedCell derivedPath.
         * @member {string} derivedPath
         * @memberof transaction.CachedCell
         * @instance
         */
        CachedCell.prototype.derivedPath = "";

        /**
         * Creates a new CachedCell instance using the specified properties.
         * @function create
         * @memberof transaction.CachedCell
         * @static
         * @param {transaction.ICachedCell=} [properties] Properties to set
         * @returns {transaction.CachedCell} CachedCell instance
         */
        CachedCell.create = function create(properties) {
            return new CachedCell(properties);
        };

        /**
         * Encodes the specified CachedCell message. Does not implicitly {@link transaction.CachedCell.verify|verify} messages.
         * @function encode
         * @memberof transaction.CachedCell
         * @static
         * @param {transaction.ICachedCell} message CachedCell message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CachedCell.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.capacity != null && message.hasOwnProperty("capacity"))
                writer.uint32(/* id 1, wireType 0 =*/8).int64(message.capacity);
            if (message.lock != null && message.hasOwnProperty("lock"))
                $root.transaction.Script.encode(message.lock, writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
            if (message.outPoint != null && message.hasOwnProperty("outPoint"))
                $root.transaction.OutPoint.encode(message.outPoint, writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.derivedPath);
            return writer;
        };

        /**
         * Encodes the specified CachedCell message, length delimited. Does not implicitly {@link transaction.CachedCell.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.CachedCell
         * @static
         * @param {transaction.ICachedCell} message CachedCell message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CachedCell.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a CachedCell message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.CachedCell
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.CachedCell} CachedCell
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CachedCell.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.CachedCell();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.capacity = reader.int64();
                    break;
                case 2:
                    message.lock = $root.transaction.Script.decode(reader, reader.uint32());
                    break;
                case 3:
                    message.outPoint = $root.transaction.OutPoint.decode(reader, reader.uint32());
                    break;
                case 4:
                    message.derivedPath = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a CachedCell message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.CachedCell
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.CachedCell} CachedCell
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CachedCell.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a CachedCell message.
         * @function verify
         * @memberof transaction.CachedCell
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CachedCell.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.capacity != null && message.hasOwnProperty("capacity"))
                if (!$util.isInteger(message.capacity) && !(message.capacity && $util.isInteger(message.capacity.low) && $util.isInteger(message.capacity.high)))
                    return "capacity: integer|Long expected";
            if (message.lock != null && message.hasOwnProperty("lock")) {
                var error = $root.transaction.Script.verify(message.lock);
                if (error)
                    return "lock." + error;
            }
            if (message.outPoint != null && message.hasOwnProperty("outPoint")) {
                var error = $root.transaction.OutPoint.verify(message.outPoint);
                if (error)
                    return "outPoint." + error;
            }
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                if (!$util.isString(message.derivedPath))
                    return "derivedPath: string expected";
            return null;
        };

        /**
         * Creates a CachedCell message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.CachedCell
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.CachedCell} CachedCell
         */
        CachedCell.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.CachedCell)
                return object;
            var message = new $root.transaction.CachedCell();
            if (object.capacity != null)
                if ($util.Long)
                    (message.capacity = $util.Long.fromValue(object.capacity)).unsigned = false;
                else if (typeof object.capacity === "string")
                    message.capacity = parseInt(object.capacity, 10);
                else if (typeof object.capacity === "number")
                    message.capacity = object.capacity;
                else if (typeof object.capacity === "object")
                    message.capacity = new $util.LongBits(object.capacity.low >>> 0, object.capacity.high >>> 0).toNumber();
            if (object.lock != null) {
                if (typeof object.lock !== "object")
                    throw TypeError(".transaction.CachedCell.lock: object expected");
                message.lock = $root.transaction.Script.fromObject(object.lock);
            }
            if (object.outPoint != null) {
                if (typeof object.outPoint !== "object")
                    throw TypeError(".transaction.CachedCell.outPoint: object expected");
                message.outPoint = $root.transaction.OutPoint.fromObject(object.outPoint);
            }
            if (object.derivedPath != null)
                message.derivedPath = String(object.derivedPath);
            return message;
        };

        /**
         * Creates a plain object from a CachedCell message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.CachedCell
         * @static
         * @param {transaction.CachedCell} message CachedCell
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CachedCell.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                if ($util.Long) {
                    var long = new $util.Long(0, 0, false);
                    object.capacity = options.longs === String ? long.toString() : options.longs === Number ? long.toNumber() : long;
                } else
                    object.capacity = options.longs === String ? "0" : 0;
                object.lock = null;
                object.outPoint = null;
                object.derivedPath = "";
            }
            if (message.capacity != null && message.hasOwnProperty("capacity"))
                if (typeof message.capacity === "number")
                    object.capacity = options.longs === String ? String(message.capacity) : message.capacity;
                else
                    object.capacity = options.longs === String ? $util.Long.prototype.toString.call(message.capacity) : options.longs === Number ? new $util.LongBits(message.capacity.low >>> 0, message.capacity.high >>> 0).toNumber() : message.capacity;
            if (message.lock != null && message.hasOwnProperty("lock"))
                object.lock = $root.transaction.Script.toObject(message.lock, options);
            if (message.outPoint != null && message.hasOwnProperty("outPoint"))
                object.outPoint = $root.transaction.OutPoint.toObject(message.outPoint, options);
            if (message.derivedPath != null && message.hasOwnProperty("derivedPath"))
                object.derivedPath = message.derivedPath;
            return object;
        };

        /**
         * Converts this CachedCell to JSON.
         * @function toJSON
         * @memberof transaction.CachedCell
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CachedCell.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return CachedCell;
    })();

    transaction.CkbTxInput = (function() {

        /**
         * Properties of a CkbTxInput.
         * @memberof transaction
         * @interface ICkbTxInput
         * @property {Array.<transaction.ICellInput>|null} [inputs] CkbTxInput inputs
         * @property {Array.<transaction.IWitness>|null} [witnesses] CkbTxInput witnesses
         * @property {Array.<transaction.ICachedCell>|null} [cachedCells] CkbTxInput cachedCells
         * @property {string|null} [txHash] CkbTxInput txHash
         */

        /**
         * Constructs a new CkbTxInput.
         * @memberof transaction
         * @classdesc Represents a CkbTxInput.
         * @implements ICkbTxInput
         * @constructor
         * @param {transaction.ICkbTxInput=} [properties] Properties to set
         */
        function CkbTxInput(properties) {
            this.inputs = [];
            this.witnesses = [];
            this.cachedCells = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * CkbTxInput inputs.
         * @member {Array.<transaction.ICellInput>} inputs
         * @memberof transaction.CkbTxInput
         * @instance
         */
        CkbTxInput.prototype.inputs = $util.emptyArray;

        /**
         * CkbTxInput witnesses.
         * @member {Array.<transaction.IWitness>} witnesses
         * @memberof transaction.CkbTxInput
         * @instance
         */
        CkbTxInput.prototype.witnesses = $util.emptyArray;

        /**
         * CkbTxInput cachedCells.
         * @member {Array.<transaction.ICachedCell>} cachedCells
         * @memberof transaction.CkbTxInput
         * @instance
         */
        CkbTxInput.prototype.cachedCells = $util.emptyArray;

        /**
         * CkbTxInput txHash.
         * @member {string} txHash
         * @memberof transaction.CkbTxInput
         * @instance
         */
        CkbTxInput.prototype.txHash = "";

        /**
         * Creates a new CkbTxInput instance using the specified properties.
         * @function create
         * @memberof transaction.CkbTxInput
         * @static
         * @param {transaction.ICkbTxInput=} [properties] Properties to set
         * @returns {transaction.CkbTxInput} CkbTxInput instance
         */
        CkbTxInput.create = function create(properties) {
            return new CkbTxInput(properties);
        };

        /**
         * Encodes the specified CkbTxInput message. Does not implicitly {@link transaction.CkbTxInput.verify|verify} messages.
         * @function encode
         * @memberof transaction.CkbTxInput
         * @static
         * @param {transaction.ICkbTxInput} message CkbTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CkbTxInput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.inputs != null && message.inputs.length)
                for (var i = 0; i < message.inputs.length; ++i)
                    $root.transaction.CellInput.encode(message.inputs[i], writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
            if (message.witnesses != null && message.witnesses.length)
                for (var i = 0; i < message.witnesses.length; ++i)
                    $root.transaction.Witness.encode(message.witnesses[i], writer.uint32(/* id 2, wireType 2 =*/18).fork()).ldelim();
            if (message.cachedCells != null && message.cachedCells.length)
                for (var i = 0; i < message.cachedCells.length; ++i)
                    $root.transaction.CachedCell.encode(message.cachedCells[i], writer.uint32(/* id 3, wireType 2 =*/26).fork()).ldelim();
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                writer.uint32(/* id 4, wireType 2 =*/34).string(message.txHash);
            return writer;
        };

        /**
         * Encodes the specified CkbTxInput message, length delimited. Does not implicitly {@link transaction.CkbTxInput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.CkbTxInput
         * @static
         * @param {transaction.ICkbTxInput} message CkbTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CkbTxInput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a CkbTxInput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.CkbTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.CkbTxInput} CkbTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CkbTxInput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.CkbTxInput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    if (!(message.inputs && message.inputs.length))
                        message.inputs = [];
                    message.inputs.push($root.transaction.CellInput.decode(reader, reader.uint32()));
                    break;
                case 2:
                    if (!(message.witnesses && message.witnesses.length))
                        message.witnesses = [];
                    message.witnesses.push($root.transaction.Witness.decode(reader, reader.uint32()));
                    break;
                case 3:
                    if (!(message.cachedCells && message.cachedCells.length))
                        message.cachedCells = [];
                    message.cachedCells.push($root.transaction.CachedCell.decode(reader, reader.uint32()));
                    break;
                case 4:
                    message.txHash = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a CkbTxInput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.CkbTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.CkbTxInput} CkbTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CkbTxInput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a CkbTxInput message.
         * @function verify
         * @memberof transaction.CkbTxInput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CkbTxInput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.inputs != null && message.hasOwnProperty("inputs")) {
                if (!Array.isArray(message.inputs))
                    return "inputs: array expected";
                for (var i = 0; i < message.inputs.length; ++i) {
                    var error = $root.transaction.CellInput.verify(message.inputs[i]);
                    if (error)
                        return "inputs." + error;
                }
            }
            if (message.witnesses != null && message.hasOwnProperty("witnesses")) {
                if (!Array.isArray(message.witnesses))
                    return "witnesses: array expected";
                for (var i = 0; i < message.witnesses.length; ++i) {
                    var error = $root.transaction.Witness.verify(message.witnesses[i]);
                    if (error)
                        return "witnesses." + error;
                }
            }
            if (message.cachedCells != null && message.hasOwnProperty("cachedCells")) {
                if (!Array.isArray(message.cachedCells))
                    return "cachedCells: array expected";
                for (var i = 0; i < message.cachedCells.length; ++i) {
                    var error = $root.transaction.CachedCell.verify(message.cachedCells[i]);
                    if (error)
                        return "cachedCells." + error;
                }
            }
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                if (!$util.isString(message.txHash))
                    return "txHash: string expected";
            return null;
        };

        /**
         * Creates a CkbTxInput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.CkbTxInput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.CkbTxInput} CkbTxInput
         */
        CkbTxInput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.CkbTxInput)
                return object;
            var message = new $root.transaction.CkbTxInput();
            if (object.inputs) {
                if (!Array.isArray(object.inputs))
                    throw TypeError(".transaction.CkbTxInput.inputs: array expected");
                message.inputs = [];
                for (var i = 0; i < object.inputs.length; ++i) {
                    if (typeof object.inputs[i] !== "object")
                        throw TypeError(".transaction.CkbTxInput.inputs: object expected");
                    message.inputs[i] = $root.transaction.CellInput.fromObject(object.inputs[i]);
                }
            }
            if (object.witnesses) {
                if (!Array.isArray(object.witnesses))
                    throw TypeError(".transaction.CkbTxInput.witnesses: array expected");
                message.witnesses = [];
                for (var i = 0; i < object.witnesses.length; ++i) {
                    if (typeof object.witnesses[i] !== "object")
                        throw TypeError(".transaction.CkbTxInput.witnesses: object expected");
                    message.witnesses[i] = $root.transaction.Witness.fromObject(object.witnesses[i]);
                }
            }
            if (object.cachedCells) {
                if (!Array.isArray(object.cachedCells))
                    throw TypeError(".transaction.CkbTxInput.cachedCells: array expected");
                message.cachedCells = [];
                for (var i = 0; i < object.cachedCells.length; ++i) {
                    if (typeof object.cachedCells[i] !== "object")
                        throw TypeError(".transaction.CkbTxInput.cachedCells: object expected");
                    message.cachedCells[i] = $root.transaction.CachedCell.fromObject(object.cachedCells[i]);
                }
            }
            if (object.txHash != null)
                message.txHash = String(object.txHash);
            return message;
        };

        /**
         * Creates a plain object from a CkbTxInput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.CkbTxInput
         * @static
         * @param {transaction.CkbTxInput} message CkbTxInput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CkbTxInput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults) {
                object.inputs = [];
                object.witnesses = [];
                object.cachedCells = [];
            }
            if (options.defaults)
                object.txHash = "";
            if (message.inputs && message.inputs.length) {
                object.inputs = [];
                for (var j = 0; j < message.inputs.length; ++j)
                    object.inputs[j] = $root.transaction.CellInput.toObject(message.inputs[j], options);
            }
            if (message.witnesses && message.witnesses.length) {
                object.witnesses = [];
                for (var j = 0; j < message.witnesses.length; ++j)
                    object.witnesses[j] = $root.transaction.Witness.toObject(message.witnesses[j], options);
            }
            if (message.cachedCells && message.cachedCells.length) {
                object.cachedCells = [];
                for (var j = 0; j < message.cachedCells.length; ++j)
                    object.cachedCells[j] = $root.transaction.CachedCell.toObject(message.cachedCells[j], options);
            }
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                object.txHash = message.txHash;
            return object;
        };

        /**
         * Converts this CkbTxInput to JSON.
         * @function toJSON
         * @memberof transaction.CkbTxInput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CkbTxInput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return CkbTxInput;
    })();

    transaction.CkbTxOutput = (function() {

        /**
         * Properties of a CkbTxOutput.
         * @memberof transaction
         * @interface ICkbTxOutput
         * @property {string|null} [txHash] CkbTxOutput txHash
         * @property {Array.<string>|null} [witnesses] CkbTxOutput witnesses
         */

        /**
         * Constructs a new CkbTxOutput.
         * @memberof transaction
         * @classdesc Represents a CkbTxOutput.
         * @implements ICkbTxOutput
         * @constructor
         * @param {transaction.ICkbTxOutput=} [properties] Properties to set
         */
        function CkbTxOutput(properties) {
            this.witnesses = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * CkbTxOutput txHash.
         * @member {string} txHash
         * @memberof transaction.CkbTxOutput
         * @instance
         */
        CkbTxOutput.prototype.txHash = "";

        /**
         * CkbTxOutput witnesses.
         * @member {Array.<string>} witnesses
         * @memberof transaction.CkbTxOutput
         * @instance
         */
        CkbTxOutput.prototype.witnesses = $util.emptyArray;

        /**
         * Creates a new CkbTxOutput instance using the specified properties.
         * @function create
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {transaction.ICkbTxOutput=} [properties] Properties to set
         * @returns {transaction.CkbTxOutput} CkbTxOutput instance
         */
        CkbTxOutput.create = function create(properties) {
            return new CkbTxOutput(properties);
        };

        /**
         * Encodes the specified CkbTxOutput message. Does not implicitly {@link transaction.CkbTxOutput.verify|verify} messages.
         * @function encode
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {transaction.ICkbTxOutput} message CkbTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CkbTxOutput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.txHash);
            if (message.witnesses != null && message.witnesses.length)
                for (var i = 0; i < message.witnesses.length; ++i)
                    writer.uint32(/* id 2, wireType 2 =*/18).string(message.witnesses[i]);
            return writer;
        };

        /**
         * Encodes the specified CkbTxOutput message, length delimited. Does not implicitly {@link transaction.CkbTxOutput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {transaction.ICkbTxOutput} message CkbTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        CkbTxOutput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a CkbTxOutput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.CkbTxOutput} CkbTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CkbTxOutput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.CkbTxOutput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.txHash = reader.string();
                    break;
                case 2:
                    if (!(message.witnesses && message.witnesses.length))
                        message.witnesses = [];
                    message.witnesses.push(reader.string());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a CkbTxOutput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.CkbTxOutput} CkbTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        CkbTxOutput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a CkbTxOutput message.
         * @function verify
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        CkbTxOutput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                if (!$util.isString(message.txHash))
                    return "txHash: string expected";
            if (message.witnesses != null && message.hasOwnProperty("witnesses")) {
                if (!Array.isArray(message.witnesses))
                    return "witnesses: array expected";
                for (var i = 0; i < message.witnesses.length; ++i)
                    if (!$util.isString(message.witnesses[i]))
                        return "witnesses: string[] expected";
            }
            return null;
        };

        /**
         * Creates a CkbTxOutput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.CkbTxOutput} CkbTxOutput
         */
        CkbTxOutput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.CkbTxOutput)
                return object;
            var message = new $root.transaction.CkbTxOutput();
            if (object.txHash != null)
                message.txHash = String(object.txHash);
            if (object.witnesses) {
                if (!Array.isArray(object.witnesses))
                    throw TypeError(".transaction.CkbTxOutput.witnesses: array expected");
                message.witnesses = [];
                for (var i = 0; i < object.witnesses.length; ++i)
                    message.witnesses[i] = String(object.witnesses[i]);
            }
            return message;
        };

        /**
         * Creates a plain object from a CkbTxOutput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.CkbTxOutput
         * @static
         * @param {transaction.CkbTxOutput} message CkbTxOutput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        CkbTxOutput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.witnesses = [];
            if (options.defaults)
                object.txHash = "";
            if (message.txHash != null && message.hasOwnProperty("txHash"))
                object.txHash = message.txHash;
            if (message.witnesses && message.witnesses.length) {
                object.witnesses = [];
                for (var j = 0; j < message.witnesses.length; ++j)
                    object.witnesses[j] = message.witnesses[j];
            }
            return object;
        };

        /**
         * Converts this CkbTxOutput to JSON.
         * @function toJSON
         * @memberof transaction.CkbTxOutput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        CkbTxOutput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return CkbTxOutput;
    })();

    transaction.TronTxInput = (function() {

        /**
         * Properties of a TronTxInput.
         * @memberof transaction
         * @interface ITronTxInput
         * @property {string|null} [rawData] TronTxInput rawData
         */

        /**
         * Constructs a new TronTxInput.
         * @memberof transaction
         * @classdesc Represents a TronTxInput.
         * @implements ITronTxInput
         * @constructor
         * @param {transaction.ITronTxInput=} [properties] Properties to set
         */
        function TronTxInput(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * TronTxInput rawData.
         * @member {string} rawData
         * @memberof transaction.TronTxInput
         * @instance
         */
        TronTxInput.prototype.rawData = "";

        /**
         * Creates a new TronTxInput instance using the specified properties.
         * @function create
         * @memberof transaction.TronTxInput
         * @static
         * @param {transaction.ITronTxInput=} [properties] Properties to set
         * @returns {transaction.TronTxInput} TronTxInput instance
         */
        TronTxInput.create = function create(properties) {
            return new TronTxInput(properties);
        };

        /**
         * Encodes the specified TronTxInput message. Does not implicitly {@link transaction.TronTxInput.verify|verify} messages.
         * @function encode
         * @memberof transaction.TronTxInput
         * @static
         * @param {transaction.ITronTxInput} message TronTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronTxInput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.rawData != null && message.hasOwnProperty("rawData"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.rawData);
            return writer;
        };

        /**
         * Encodes the specified TronTxInput message, length delimited. Does not implicitly {@link transaction.TronTxInput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.TronTxInput
         * @static
         * @param {transaction.ITronTxInput} message TronTxInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronTxInput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a TronTxInput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.TronTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.TronTxInput} TronTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronTxInput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.TronTxInput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.rawData = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a TronTxInput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.TronTxInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.TronTxInput} TronTxInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronTxInput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a TronTxInput message.
         * @function verify
         * @memberof transaction.TronTxInput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        TronTxInput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.rawData != null && message.hasOwnProperty("rawData"))
                if (!$util.isString(message.rawData))
                    return "rawData: string expected";
            return null;
        };

        /**
         * Creates a TronTxInput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.TronTxInput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.TronTxInput} TronTxInput
         */
        TronTxInput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.TronTxInput)
                return object;
            var message = new $root.transaction.TronTxInput();
            if (object.rawData != null)
                message.rawData = String(object.rawData);
            return message;
        };

        /**
         * Creates a plain object from a TronTxInput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.TronTxInput
         * @static
         * @param {transaction.TronTxInput} message TronTxInput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        TronTxInput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults)
                object.rawData = "";
            if (message.rawData != null && message.hasOwnProperty("rawData"))
                object.rawData = message.rawData;
            return object;
        };

        /**
         * Converts this TronTxInput to JSON.
         * @function toJSON
         * @memberof transaction.TronTxInput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        TronTxInput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return TronTxInput;
    })();

    transaction.TronTxOutput = (function() {

        /**
         * Properties of a TronTxOutput.
         * @memberof transaction
         * @interface ITronTxOutput
         * @property {Array.<string>|null} [signatures] TronTxOutput signatures
         */

        /**
         * Constructs a new TronTxOutput.
         * @memberof transaction
         * @classdesc Represents a TronTxOutput.
         * @implements ITronTxOutput
         * @constructor
         * @param {transaction.ITronTxOutput=} [properties] Properties to set
         */
        function TronTxOutput(properties) {
            this.signatures = [];
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * TronTxOutput signatures.
         * @member {Array.<string>} signatures
         * @memberof transaction.TronTxOutput
         * @instance
         */
        TronTxOutput.prototype.signatures = $util.emptyArray;

        /**
         * Creates a new TronTxOutput instance using the specified properties.
         * @function create
         * @memberof transaction.TronTxOutput
         * @static
         * @param {transaction.ITronTxOutput=} [properties] Properties to set
         * @returns {transaction.TronTxOutput} TronTxOutput instance
         */
        TronTxOutput.create = function create(properties) {
            return new TronTxOutput(properties);
        };

        /**
         * Encodes the specified TronTxOutput message. Does not implicitly {@link transaction.TronTxOutput.verify|verify} messages.
         * @function encode
         * @memberof transaction.TronTxOutput
         * @static
         * @param {transaction.ITronTxOutput} message TronTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronTxOutput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.signatures != null && message.signatures.length)
                for (var i = 0; i < message.signatures.length; ++i)
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.signatures[i]);
            return writer;
        };

        /**
         * Encodes the specified TronTxOutput message, length delimited. Does not implicitly {@link transaction.TronTxOutput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.TronTxOutput
         * @static
         * @param {transaction.ITronTxOutput} message TronTxOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronTxOutput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a TronTxOutput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.TronTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.TronTxOutput} TronTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronTxOutput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.TronTxOutput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    if (!(message.signatures && message.signatures.length))
                        message.signatures = [];
                    message.signatures.push(reader.string());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a TronTxOutput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.TronTxOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.TronTxOutput} TronTxOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronTxOutput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a TronTxOutput message.
         * @function verify
         * @memberof transaction.TronTxOutput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        TronTxOutput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.signatures != null && message.hasOwnProperty("signatures")) {
                if (!Array.isArray(message.signatures))
                    return "signatures: array expected";
                for (var i = 0; i < message.signatures.length; ++i)
                    if (!$util.isString(message.signatures[i]))
                        return "signatures: string[] expected";
            }
            return null;
        };

        /**
         * Creates a TronTxOutput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.TronTxOutput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.TronTxOutput} TronTxOutput
         */
        TronTxOutput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.TronTxOutput)
                return object;
            var message = new $root.transaction.TronTxOutput();
            if (object.signatures) {
                if (!Array.isArray(object.signatures))
                    throw TypeError(".transaction.TronTxOutput.signatures: array expected");
                message.signatures = [];
                for (var i = 0; i < object.signatures.length; ++i)
                    message.signatures[i] = String(object.signatures[i]);
            }
            return message;
        };

        /**
         * Creates a plain object from a TronTxOutput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.TronTxOutput
         * @static
         * @param {transaction.TronTxOutput} message TronTxOutput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        TronTxOutput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.arrays || options.defaults)
                object.signatures = [];
            if (message.signatures && message.signatures.length) {
                object.signatures = [];
                for (var j = 0; j < message.signatures.length; ++j)
                    object.signatures[j] = message.signatures[j];
            }
            return object;
        };

        /**
         * Converts this TronTxOutput to JSON.
         * @function toJSON
         * @memberof transaction.TronTxOutput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        TronTxOutput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return TronTxOutput;
    })();

    transaction.TronMessageInput = (function() {

        /**
         * Properties of a TronMessageInput.
         * @memberof transaction
         * @interface ITronMessageInput
         * @property {string|null} [value] TronMessageInput value
         * @property {boolean|null} [isHex] TronMessageInput isHex
         * @property {boolean|null} [isTronHeader] TronMessageInput isTronHeader
         */

        /**
         * Constructs a new TronMessageInput.
         * @memberof transaction
         * @classdesc Represents a TronMessageInput.
         * @implements ITronMessageInput
         * @constructor
         * @param {transaction.ITronMessageInput=} [properties] Properties to set
         */
        function TronMessageInput(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * TronMessageInput value.
         * @member {string} value
         * @memberof transaction.TronMessageInput
         * @instance
         */
        TronMessageInput.prototype.value = "";

        /**
         * TronMessageInput isHex.
         * @member {boolean} isHex
         * @memberof transaction.TronMessageInput
         * @instance
         */
        TronMessageInput.prototype.isHex = false;

        /**
         * TronMessageInput isTronHeader.
         * @member {boolean} isTronHeader
         * @memberof transaction.TronMessageInput
         * @instance
         */
        TronMessageInput.prototype.isTronHeader = false;

        /**
         * Creates a new TronMessageInput instance using the specified properties.
         * @function create
         * @memberof transaction.TronMessageInput
         * @static
         * @param {transaction.ITronMessageInput=} [properties] Properties to set
         * @returns {transaction.TronMessageInput} TronMessageInput instance
         */
        TronMessageInput.create = function create(properties) {
            return new TronMessageInput(properties);
        };

        /**
         * Encodes the specified TronMessageInput message. Does not implicitly {@link transaction.TronMessageInput.verify|verify} messages.
         * @function encode
         * @memberof transaction.TronMessageInput
         * @static
         * @param {transaction.ITronMessageInput} message TronMessageInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronMessageInput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.value != null && message.hasOwnProperty("value"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.value);
            if (message.isHex != null && message.hasOwnProperty("isHex"))
                writer.uint32(/* id 2, wireType 0 =*/16).bool(message.isHex);
            if (message.isTronHeader != null && message.hasOwnProperty("isTronHeader"))
                writer.uint32(/* id 3, wireType 0 =*/24).bool(message.isTronHeader);
            return writer;
        };

        /**
         * Encodes the specified TronMessageInput message, length delimited. Does not implicitly {@link transaction.TronMessageInput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.TronMessageInput
         * @static
         * @param {transaction.ITronMessageInput} message TronMessageInput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronMessageInput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a TronMessageInput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.TronMessageInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.TronMessageInput} TronMessageInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronMessageInput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.TronMessageInput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.value = reader.string();
                    break;
                case 2:
                    message.isHex = reader.bool();
                    break;
                case 3:
                    message.isTronHeader = reader.bool();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a TronMessageInput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.TronMessageInput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.TronMessageInput} TronMessageInput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronMessageInput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a TronMessageInput message.
         * @function verify
         * @memberof transaction.TronMessageInput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        TronMessageInput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.value != null && message.hasOwnProperty("value"))
                if (!$util.isString(message.value))
                    return "value: string expected";
            if (message.isHex != null && message.hasOwnProperty("isHex"))
                if (typeof message.isHex !== "boolean")
                    return "isHex: boolean expected";
            if (message.isTronHeader != null && message.hasOwnProperty("isTronHeader"))
                if (typeof message.isTronHeader !== "boolean")
                    return "isTronHeader: boolean expected";
            return null;
        };

        /**
         * Creates a TronMessageInput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.TronMessageInput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.TronMessageInput} TronMessageInput
         */
        TronMessageInput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.TronMessageInput)
                return object;
            var message = new $root.transaction.TronMessageInput();
            if (object.value != null)
                message.value = String(object.value);
            if (object.isHex != null)
                message.isHex = Boolean(object.isHex);
            if (object.isTronHeader != null)
                message.isTronHeader = Boolean(object.isTronHeader);
            return message;
        };

        /**
         * Creates a plain object from a TronMessageInput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.TronMessageInput
         * @static
         * @param {transaction.TronMessageInput} message TronMessageInput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        TronMessageInput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults) {
                object.value = "";
                object.isHex = false;
                object.isTronHeader = false;
            }
            if (message.value != null && message.hasOwnProperty("value"))
                object.value = message.value;
            if (message.isHex != null && message.hasOwnProperty("isHex"))
                object.isHex = message.isHex;
            if (message.isTronHeader != null && message.hasOwnProperty("isTronHeader"))
                object.isTronHeader = message.isTronHeader;
            return object;
        };

        /**
         * Converts this TronMessageInput to JSON.
         * @function toJSON
         * @memberof transaction.TronMessageInput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        TronMessageInput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return TronMessageInput;
    })();

    transaction.TronMessageOutput = (function() {

        /**
         * Properties of a TronMessageOutput.
         * @memberof transaction
         * @interface ITronMessageOutput
         * @property {string|null} [signature] TronMessageOutput signature
         */

        /**
         * Constructs a new TronMessageOutput.
         * @memberof transaction
         * @classdesc Represents a TronMessageOutput.
         * @implements ITronMessageOutput
         * @constructor
         * @param {transaction.ITronMessageOutput=} [properties] Properties to set
         */
        function TronMessageOutput(properties) {
            if (properties)
                for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                    if (properties[keys[i]] != null)
                        this[keys[i]] = properties[keys[i]];
        }

        /**
         * TronMessageOutput signature.
         * @member {string} signature
         * @memberof transaction.TronMessageOutput
         * @instance
         */
        TronMessageOutput.prototype.signature = "";

        /**
         * Creates a new TronMessageOutput instance using the specified properties.
         * @function create
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {transaction.ITronMessageOutput=} [properties] Properties to set
         * @returns {transaction.TronMessageOutput} TronMessageOutput instance
         */
        TronMessageOutput.create = function create(properties) {
            return new TronMessageOutput(properties);
        };

        /**
         * Encodes the specified TronMessageOutput message. Does not implicitly {@link transaction.TronMessageOutput.verify|verify} messages.
         * @function encode
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {transaction.ITronMessageOutput} message TronMessageOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronMessageOutput.encode = function encode(message, writer) {
            if (!writer)
                writer = $Writer.create();
            if (message.signature != null && message.hasOwnProperty("signature"))
                writer.uint32(/* id 1, wireType 2 =*/10).string(message.signature);
            return writer;
        };

        /**
         * Encodes the specified TronMessageOutput message, length delimited. Does not implicitly {@link transaction.TronMessageOutput.verify|verify} messages.
         * @function encodeDelimited
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {transaction.ITronMessageOutput} message TronMessageOutput message or plain object to encode
         * @param {$protobuf.Writer} [writer] Writer to encode to
         * @returns {$protobuf.Writer} Writer
         */
        TronMessageOutput.encodeDelimited = function encodeDelimited(message, writer) {
            return this.encode(message, writer).ldelim();
        };

        /**
         * Decodes a TronMessageOutput message from the specified reader or buffer.
         * @function decode
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @param {number} [length] Message length if known beforehand
         * @returns {transaction.TronMessageOutput} TronMessageOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronMessageOutput.decode = function decode(reader, length) {
            if (!(reader instanceof $Reader))
                reader = $Reader.create(reader);
            var end = length === undefined ? reader.len : reader.pos + length, message = new $root.transaction.TronMessageOutput();
            while (reader.pos < end) {
                var tag = reader.uint32();
                switch (tag >>> 3) {
                case 1:
                    message.signature = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
                }
            }
            return message;
        };

        /**
         * Decodes a TronMessageOutput message from the specified reader or buffer, length delimited.
         * @function decodeDelimited
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
         * @returns {transaction.TronMessageOutput} TronMessageOutput
         * @throws {Error} If the payload is not a reader or valid buffer
         * @throws {$protobuf.util.ProtocolError} If required fields are missing
         */
        TronMessageOutput.decodeDelimited = function decodeDelimited(reader) {
            if (!(reader instanceof $Reader))
                reader = new $Reader(reader);
            return this.decode(reader, reader.uint32());
        };

        /**
         * Verifies a TronMessageOutput message.
         * @function verify
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {Object.<string,*>} message Plain object to verify
         * @returns {string|null} `null` if valid, otherwise the reason why it is not
         */
        TronMessageOutput.verify = function verify(message) {
            if (typeof message !== "object" || message === null)
                return "object expected";
            if (message.signature != null && message.hasOwnProperty("signature"))
                if (!$util.isString(message.signature))
                    return "signature: string expected";
            return null;
        };

        /**
         * Creates a TronMessageOutput message from a plain object. Also converts values to their respective internal types.
         * @function fromObject
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {Object.<string,*>} object Plain object
         * @returns {transaction.TronMessageOutput} TronMessageOutput
         */
        TronMessageOutput.fromObject = function fromObject(object) {
            if (object instanceof $root.transaction.TronMessageOutput)
                return object;
            var message = new $root.transaction.TronMessageOutput();
            if (object.signature != null)
                message.signature = String(object.signature);
            return message;
        };

        /**
         * Creates a plain object from a TronMessageOutput message. Also converts values to other types if specified.
         * @function toObject
         * @memberof transaction.TronMessageOutput
         * @static
         * @param {transaction.TronMessageOutput} message TronMessageOutput
         * @param {$protobuf.IConversionOptions} [options] Conversion options
         * @returns {Object.<string,*>} Plain object
         */
        TronMessageOutput.toObject = function toObject(message, options) {
            if (!options)
                options = {};
            var object = {};
            if (options.defaults)
                object.signature = "";
            if (message.signature != null && message.hasOwnProperty("signature"))
                object.signature = message.signature;
            return object;
        };

        /**
         * Converts this TronMessageOutput to JSON.
         * @function toJSON
         * @memberof transaction.TronMessageOutput
         * @instance
         * @returns {Object.<string,*>} JSON object
         */
        TronMessageOutput.prototype.toJSON = function toJSON() {
            return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
        };

        return TronMessageOutput;
    })();

    return transaction;
})();

module.exports = $root;
