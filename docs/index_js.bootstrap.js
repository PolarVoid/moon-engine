"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkmoon_engine"] = self["webpackChunkmoon_engine"] || []).push([["index_js"],{

/***/ "../pkg/moon_engine_bg.js":
/*!********************************!*\
  !*** ../pkg/moon_engine_bg.js ***!
  \********************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"Application\": () => (/* binding */ Application),\n/* harmony export */   \"__wbindgen_object_drop_ref\": () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   \"__wbindgen_boolean_get\": () => (/* binding */ __wbindgen_boolean_get),\n/* harmony export */   \"__wbg_new_693216e109162396\": () => (/* binding */ __wbg_new_693216e109162396),\n/* harmony export */   \"__wbg_stack_0ddaca5d1abfb52f\": () => (/* binding */ __wbg_stack_0ddaca5d1abfb52f),\n/* harmony export */   \"__wbg_error_09919627ac0992f5\": () => (/* binding */ __wbg_error_09919627ac0992f5),\n/* harmony export */   \"__wbg_instanceof_WebGl2RenderingContext_df519ebc1fd4a55f\": () => (/* binding */ __wbg_instanceof_WebGl2RenderingContext_df519ebc1fd4a55f),\n/* harmony export */   \"__wbg_bindVertexArray_8020efc46272d6b1\": () => (/* binding */ __wbg_bindVertexArray_8020efc46272d6b1),\n/* harmony export */   \"__wbg_bufferData_2b2006d269bd7f65\": () => (/* binding */ __wbg_bufferData_2b2006d269bd7f65),\n/* harmony export */   \"__wbg_createVertexArray_ccfd68f784dda58d\": () => (/* binding */ __wbg_createVertexArray_ccfd68f784dda58d),\n/* harmony export */   \"__wbg_deleteVertexArray_431b44dad4d908dc\": () => (/* binding */ __wbg_deleteVertexArray_431b44dad4d908dc),\n/* harmony export */   \"__wbg_texImage2D_8e3d1e2fc4b9cf89\": () => (/* binding */ __wbg_texImage2D_8e3d1e2fc4b9cf89),\n/* harmony export */   \"__wbg_texImage2D_ea4f44f738393ea2\": () => (/* binding */ __wbg_texImage2D_ea4f44f738393ea2),\n/* harmony export */   \"__wbg_uniformMatrix4fv_8752c8df4a82f43a\": () => (/* binding */ __wbg_uniformMatrix4fv_8752c8df4a82f43a),\n/* harmony export */   \"__wbg_activeTexture_e07e910acea70faa\": () => (/* binding */ __wbg_activeTexture_e07e910acea70faa),\n/* harmony export */   \"__wbg_attachShader_2e252ab2fda53d9b\": () => (/* binding */ __wbg_attachShader_2e252ab2fda53d9b),\n/* harmony export */   \"__wbg_bindBuffer_612af2c0d1623df9\": () => (/* binding */ __wbg_bindBuffer_612af2c0d1623df9),\n/* harmony export */   \"__wbg_bindTexture_5de299363180ad48\": () => (/* binding */ __wbg_bindTexture_5de299363180ad48),\n/* harmony export */   \"__wbg_blendFunc_a1fda75b5cf06b09\": () => (/* binding */ __wbg_blendFunc_a1fda75b5cf06b09),\n/* harmony export */   \"__wbg_clear_4c5eed385310e256\": () => (/* binding */ __wbg_clear_4c5eed385310e256),\n/* harmony export */   \"__wbg_clearColor_d9d486c5ff20404c\": () => (/* binding */ __wbg_clearColor_d9d486c5ff20404c),\n/* harmony export */   \"__wbg_compileShader_e224e94272352503\": () => (/* binding */ __wbg_compileShader_e224e94272352503),\n/* harmony export */   \"__wbg_createBuffer_564dc1c3c3f058b7\": () => (/* binding */ __wbg_createBuffer_564dc1c3c3f058b7),\n/* harmony export */   \"__wbg_createProgram_e9fa1d7669773667\": () => (/* binding */ __wbg_createProgram_e9fa1d7669773667),\n/* harmony export */   \"__wbg_createShader_03233922e9b5ebf2\": () => (/* binding */ __wbg_createShader_03233922e9b5ebf2),\n/* harmony export */   \"__wbg_createTexture_7ee50a5b223f0511\": () => (/* binding */ __wbg_createTexture_7ee50a5b223f0511),\n/* harmony export */   \"__wbg_deleteBuffer_50cb909fb6b297dd\": () => (/* binding */ __wbg_deleteBuffer_50cb909fb6b297dd),\n/* harmony export */   \"__wbg_deleteProgram_0d4952ded7ec132a\": () => (/* binding */ __wbg_deleteProgram_0d4952ded7ec132a),\n/* harmony export */   \"__wbg_deleteShader_67c4f4b03b5c074a\": () => (/* binding */ __wbg_deleteShader_67c4f4b03b5c074a),\n/* harmony export */   \"__wbg_deleteTexture_b4643da89823c0c1\": () => (/* binding */ __wbg_deleteTexture_b4643da89823c0c1),\n/* harmony export */   \"__wbg_drawElements_8f3cfd28610fd46e\": () => (/* binding */ __wbg_drawElements_8f3cfd28610fd46e),\n/* harmony export */   \"__wbg_enable_8e888a63831a3fe5\": () => (/* binding */ __wbg_enable_8e888a63831a3fe5),\n/* harmony export */   \"__wbg_enableVertexAttribArray_d1b2636395bdaa7a\": () => (/* binding */ __wbg_enableVertexAttribArray_d1b2636395bdaa7a),\n/* harmony export */   \"__wbg_generateMipmap_35669af1ecd88073\": () => (/* binding */ __wbg_generateMipmap_35669af1ecd88073),\n/* harmony export */   \"__wbg_getProgramInfoLog_dbd8d8cedcc8cdcc\": () => (/* binding */ __wbg_getProgramInfoLog_dbd8d8cedcc8cdcc),\n/* harmony export */   \"__wbg_getProgramParameter_4b9d43902599c2d2\": () => (/* binding */ __wbg_getProgramParameter_4b9d43902599c2d2),\n/* harmony export */   \"__wbg_getShaderInfoLog_5aab05280bd0fe1b\": () => (/* binding */ __wbg_getShaderInfoLog_5aab05280bd0fe1b),\n/* harmony export */   \"__wbg_getShaderParameter_e5f7e371d4eec000\": () => (/* binding */ __wbg_getShaderParameter_e5f7e371d4eec000),\n/* harmony export */   \"__wbg_getUniformLocation_9541edb0d39d1646\": () => (/* binding */ __wbg_getUniformLocation_9541edb0d39d1646),\n/* harmony export */   \"__wbg_linkProgram_116382e2dc17af64\": () => (/* binding */ __wbg_linkProgram_116382e2dc17af64),\n/* harmony export */   \"__wbg_pixelStorei_ea8cf13cf2f14a47\": () => (/* binding */ __wbg_pixelStorei_ea8cf13cf2f14a47),\n/* harmony export */   \"__wbg_shaderSource_0066bb6817bf9e88\": () => (/* binding */ __wbg_shaderSource_0066bb6817bf9e88),\n/* harmony export */   \"__wbg_texParameteri_52fb3e85a6d2c636\": () => (/* binding */ __wbg_texParameteri_52fb3e85a6d2c636),\n/* harmony export */   \"__wbg_uniform1f_96a968d4f5cb18de\": () => (/* binding */ __wbg_uniform1f_96a968d4f5cb18de),\n/* harmony export */   \"__wbg_uniform1i_a6ce351ee8cef296\": () => (/* binding */ __wbg_uniform1i_a6ce351ee8cef296),\n/* harmony export */   \"__wbg_uniform4f_0ff24ef1f3ab8946\": () => (/* binding */ __wbg_uniform4f_0ff24ef1f3ab8946),\n/* harmony export */   \"__wbg_useProgram_de22d1e01c430663\": () => (/* binding */ __wbg_useProgram_de22d1e01c430663),\n/* harmony export */   \"__wbg_vertexAttribPointer_4e139167926d5080\": () => (/* binding */ __wbg_vertexAttribPointer_4e139167926d5080),\n/* harmony export */   \"__wbg_viewport_caffbaa3e8b9568b\": () => (/* binding */ __wbg_viewport_caffbaa3e8b9568b),\n/* harmony export */   \"__wbg_instanceof_Window_434ce1849eb4e0fc\": () => (/* binding */ __wbg_instanceof_Window_434ce1849eb4e0fc),\n/* harmony export */   \"__wbg_document_5edd43643d1060d9\": () => (/* binding */ __wbg_document_5edd43643d1060d9),\n/* harmony export */   \"__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638\": () => (/* binding */ __wbg_instanceof_HtmlCanvasElement_a6157e470d06b638),\n/* harmony export */   \"__wbg_getContext_bd4e9445094eda84\": () => (/* binding */ __wbg_getContext_bd4e9445094eda84),\n/* harmony export */   \"__wbg_getElementById_b30e88aff96f66a1\": () => (/* binding */ __wbg_getElementById_b30e88aff96f66a1),\n/* harmony export */   \"__wbg_instanceof_HtmlImageElement_637549c09c0d94b5\": () => (/* binding */ __wbg_instanceof_HtmlImageElement_637549c09c0d94b5),\n/* harmony export */   \"__wbg_width_6c4cad65073b3852\": () => (/* binding */ __wbg_width_6c4cad65073b3852),\n/* harmony export */   \"__wbg_height_133772b066cfc559\": () => (/* binding */ __wbg_height_133772b066cfc559),\n/* harmony export */   \"__wbg_newnoargs_f579424187aa1717\": () => (/* binding */ __wbg_newnoargs_f579424187aa1717),\n/* harmony export */   \"__wbg_call_89558c3e96703ca1\": () => (/* binding */ __wbg_call_89558c3e96703ca1),\n/* harmony export */   \"__wbg_self_e23d74ae45fb17d1\": () => (/* binding */ __wbg_self_e23d74ae45fb17d1),\n/* harmony export */   \"__wbg_window_b4be7f48b24ac56e\": () => (/* binding */ __wbg_window_b4be7f48b24ac56e),\n/* harmony export */   \"__wbg_globalThis_d61b1f48a57191ae\": () => (/* binding */ __wbg_globalThis_d61b1f48a57191ae),\n/* harmony export */   \"__wbg_global_e7669da72fd7f239\": () => (/* binding */ __wbg_global_e7669da72fd7f239),\n/* harmony export */   \"__wbindgen_is_undefined\": () => (/* binding */ __wbindgen_is_undefined),\n/* harmony export */   \"__wbindgen_object_clone_ref\": () => (/* binding */ __wbindgen_object_clone_ref),\n/* harmony export */   \"__wbg_random_7b8246250fd79f60\": () => (/* binding */ __wbg_random_7b8246250fd79f60),\n/* harmony export */   \"__wbindgen_debug_string\": () => (/* binding */ __wbindgen_debug_string),\n/* harmony export */   \"__wbindgen_throw\": () => (/* binding */ __wbindgen_throw)\n/* harmony export */ });\n/* harmony import */ var _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./moon_engine_bg.wasm */ \"../pkg/moon_engine_bg.wasm\");\n/* module decorator */ module = __webpack_require__.hmd(module);\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction debugString(val) {\n    // primitive types\n    const type = typeof val;\n    if (type == 'number' || type == 'boolean' || val == null) {\n        return  `${val}`;\n    }\n    if (type == 'string') {\n        return `\"${val}\"`;\n    }\n    if (type == 'symbol') {\n        const description = val.description;\n        if (description == null) {\n            return 'Symbol';\n        } else {\n            return `Symbol(${description})`;\n        }\n    }\n    if (type == 'function') {\n        const name = val.name;\n        if (typeof name == 'string' && name.length > 0) {\n            return `Function(${name})`;\n        } else {\n            return 'Function';\n        }\n    }\n    // objects\n    if (Array.isArray(val)) {\n        const length = val.length;\n        let debug = '[';\n        if (length > 0) {\n            debug += debugString(val[0]);\n        }\n        for(let i = 1; i < length; i++) {\n            debug += ', ' + debugString(val[i]);\n        }\n        debug += ']';\n        return debug;\n    }\n    // Test for built-in\n    const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n    let className;\n    if (builtInMatches.length > 1) {\n        className = builtInMatches[1];\n    } else {\n        // Failed to match the standard '[object ClassName]'\n        return toString.call(val);\n    }\n    if (className == 'Object') {\n        // we're a user defined class or Object\n        // JSON.stringify avoids problems with cycles, and is generally much\n        // easier than looping through ownProperties of `val`.\n        try {\n            return 'Object(' + JSON.stringify(val) + ')';\n        } catch (_) {\n            return 'Object';\n        }\n    }\n    // errors\n    if (val instanceof Error) {\n        return `${val.name}: ${val.message}\\n${val.stack}`;\n    }\n    // TODO we could test for more things here, like `Set`s and `Map`s.\n    return className;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetInt32Memory0 = new Int32Array(_moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nfunction handleError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_exn_store(addHeapObject(e));\n    }\n}\n\nlet cachegetFloat32Memory0 = null;\nfunction getFloat32Memory0() {\n    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetFloat32Memory0 = new Float32Array(_moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetFloat32Memory0;\n}\n\nfunction getArrayF32FromWasm0(ptr, len) {\n    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n\nfunction notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }\n/**\n* The [`Application`] struct acts as the communicator between the browser and the game logic. It consists of calls made from JavaScript.\n*/\nclass Application {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Application.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_application_free(ptr);\n    }\n    /**\n    * Initilize a default [`Application`].\n    */\n    constructor() {\n        var ret = _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_new();\n        return Application.__wrap(ret);\n    }\n    /**\n    * Set up data before render loop.\n    */\n    init() {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_init(this.ptr);\n    }\n    /**\n    * Called when window gets resized.\n    * @param {number} width\n    * @param {number} height\n    */\n    resize(width, height) {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_resize(this.ptr, width, height);\n    }\n    /**\n    * Called when a keyboard input event is generated.\n    * @param {number} key_code\n    * @param {boolean} is_down\n    */\n    input(key_code, is_down) {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_input(this.ptr, key_code, is_down);\n    }\n    /**\n    * Handles Mouse movement.\n    * @param {number} mouse_x\n    * @param {number} mouse_y\n    */\n    mouse_move(mouse_x, mouse_y) {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_mouse_move(this.ptr, mouse_x, mouse_y);\n    }\n    /**\n    * Renders a new frame.\n    *\n    * Called every frame, and draws its output onto the [Canvas](web_sys::HtmlCanvasElement).\n    * @param {number} delta_time\n    */\n    render(delta_time) {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.application_render(this.ptr, delta_time);\n    }\n}\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbindgen_boolean_get(arg0) {\n    const v = getObject(arg0);\n    var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;\n    return ret;\n};\n\nfunction __wbg_new_693216e109162396() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_09919627ac0992f5(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);\n    }\n};\n\nfunction __wbg_instanceof_WebGl2RenderingContext_df519ebc1fd4a55f(arg0) {\n    var ret = getObject(arg0) instanceof WebGL2RenderingContext;\n    return ret;\n};\n\nfunction __wbg_bindVertexArray_8020efc46272d6b1(arg0, arg1) {\n    getObject(arg0).bindVertexArray(getObject(arg1));\n};\n\nfunction __wbg_bufferData_2b2006d269bd7f65(arg0, arg1, arg2, arg3, arg4) {\n    getObject(arg0).bufferData(arg1 >>> 0, getArrayU8FromWasm0(arg2, arg3), arg4 >>> 0);\n};\n\nfunction __wbg_createVertexArray_ccfd68f784dda58d(arg0) {\n    var ret = getObject(arg0).createVertexArray();\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_deleteVertexArray_431b44dad4d908dc(arg0, arg1) {\n    getObject(arg0).deleteVertexArray(getObject(arg1));\n};\n\nfunction __wbg_texImage2D_8e3d1e2fc4b9cf89() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {\n    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));\n}, arguments) };\n\nfunction __wbg_texImage2D_ea4f44f738393ea2() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {\n    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));\n}, arguments) };\n\nfunction __wbg_uniformMatrix4fv_8752c8df4a82f43a(arg0, arg1, arg2, arg3, arg4) {\n    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));\n};\n\nfunction __wbg_activeTexture_e07e910acea70faa(arg0, arg1) {\n    getObject(arg0).activeTexture(arg1 >>> 0);\n};\n\nfunction __wbg_attachShader_2e252ab2fda53d9b(arg0, arg1, arg2) {\n    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));\n};\n\nfunction __wbg_bindBuffer_612af2c0d1623df9(arg0, arg1, arg2) {\n    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));\n};\n\nfunction __wbg_bindTexture_5de299363180ad48(arg0, arg1, arg2) {\n    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));\n};\n\nfunction __wbg_blendFunc_a1fda75b5cf06b09(arg0, arg1, arg2) {\n    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);\n};\n\nfunction __wbg_clear_4c5eed385310e256(arg0, arg1) {\n    getObject(arg0).clear(arg1 >>> 0);\n};\n\nfunction __wbg_clearColor_d9d486c5ff20404c(arg0, arg1, arg2, arg3, arg4) {\n    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);\n};\n\nfunction __wbg_compileShader_e224e94272352503(arg0, arg1) {\n    getObject(arg0).compileShader(getObject(arg1));\n};\n\nfunction __wbg_createBuffer_564dc1c3c3f058b7(arg0) {\n    var ret = getObject(arg0).createBuffer();\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_createProgram_e9fa1d7669773667(arg0) {\n    var ret = getObject(arg0).createProgram();\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_createShader_03233922e9b5ebf2(arg0, arg1) {\n    var ret = getObject(arg0).createShader(arg1 >>> 0);\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_createTexture_7ee50a5b223f0511(arg0) {\n    var ret = getObject(arg0).createTexture();\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_deleteBuffer_50cb909fb6b297dd(arg0, arg1) {\n    getObject(arg0).deleteBuffer(getObject(arg1));\n};\n\nfunction __wbg_deleteProgram_0d4952ded7ec132a(arg0, arg1) {\n    getObject(arg0).deleteProgram(getObject(arg1));\n};\n\nfunction __wbg_deleteShader_67c4f4b03b5c074a(arg0, arg1) {\n    getObject(arg0).deleteShader(getObject(arg1));\n};\n\nfunction __wbg_deleteTexture_b4643da89823c0c1(arg0, arg1) {\n    getObject(arg0).deleteTexture(getObject(arg1));\n};\n\nfunction __wbg_drawElements_8f3cfd28610fd46e(arg0, arg1, arg2, arg3, arg4) {\n    getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);\n};\n\nfunction __wbg_enable_8e888a63831a3fe5(arg0, arg1) {\n    getObject(arg0).enable(arg1 >>> 0);\n};\n\nfunction __wbg_enableVertexAttribArray_d1b2636395bdaa7a(arg0, arg1) {\n    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);\n};\n\nfunction __wbg_generateMipmap_35669af1ecd88073(arg0, arg1) {\n    getObject(arg0).generateMipmap(arg1 >>> 0);\n};\n\nfunction __wbg_getProgramInfoLog_dbd8d8cedcc8cdcc(arg0, arg1, arg2) {\n    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));\n    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_getProgramParameter_4b9d43902599c2d2(arg0, arg1, arg2) {\n    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_getShaderInfoLog_5aab05280bd0fe1b(arg0, arg1, arg2) {\n    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));\n    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_getShaderParameter_e5f7e371d4eec000(arg0, arg1, arg2) {\n    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_getUniformLocation_9541edb0d39d1646(arg0, arg1, arg2, arg3) {\n    var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_linkProgram_116382e2dc17af64(arg0, arg1) {\n    getObject(arg0).linkProgram(getObject(arg1));\n};\n\nfunction __wbg_pixelStorei_ea8cf13cf2f14a47(arg0, arg1, arg2) {\n    getObject(arg0).pixelStorei(arg1 >>> 0, arg2);\n};\n\nfunction __wbg_shaderSource_0066bb6817bf9e88(arg0, arg1, arg2, arg3) {\n    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));\n};\n\nfunction __wbg_texParameteri_52fb3e85a6d2c636(arg0, arg1, arg2, arg3) {\n    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);\n};\n\nfunction __wbg_uniform1f_96a968d4f5cb18de(arg0, arg1, arg2) {\n    getObject(arg0).uniform1f(getObject(arg1), arg2);\n};\n\nfunction __wbg_uniform1i_a6ce351ee8cef296(arg0, arg1, arg2) {\n    getObject(arg0).uniform1i(getObject(arg1), arg2);\n};\n\nfunction __wbg_uniform4f_0ff24ef1f3ab8946(arg0, arg1, arg2, arg3, arg4, arg5) {\n    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);\n};\n\nfunction __wbg_useProgram_de22d1e01c430663(arg0, arg1) {\n    getObject(arg0).useProgram(getObject(arg1));\n};\n\nfunction __wbg_vertexAttribPointer_4e139167926d5080(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {\n    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);\n};\n\nfunction __wbg_viewport_caffbaa3e8b9568b(arg0, arg1, arg2, arg3, arg4) {\n    getObject(arg0).viewport(arg1, arg2, arg3, arg4);\n};\n\nfunction __wbg_instanceof_Window_434ce1849eb4e0fc(arg0) {\n    var ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nfunction __wbg_document_5edd43643d1060d9(arg0) {\n    var ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_instanceof_HtmlCanvasElement_a6157e470d06b638(arg0) {\n    var ret = getObject(arg0) instanceof HTMLCanvasElement;\n    return ret;\n};\n\nfunction __wbg_getContext_bd4e9445094eda84() { return handleError(function (arg0, arg1, arg2) {\n    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_getElementById_b30e88aff96f66a1(arg0, arg1, arg2) {\n    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_instanceof_HtmlImageElement_637549c09c0d94b5(arg0) {\n    var ret = getObject(arg0) instanceof HTMLImageElement;\n    return ret;\n};\n\nfunction __wbg_width_6c4cad65073b3852(arg0) {\n    var ret = getObject(arg0).width;\n    return ret;\n};\n\nfunction __wbg_height_133772b066cfc559(arg0) {\n    var ret = getObject(arg0).height;\n    return ret;\n};\n\nfunction __wbg_newnoargs_f579424187aa1717(arg0, arg1) {\n    var ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbg_call_89558c3e96703ca1() { return handleError(function (arg0, arg1) {\n    var ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_self_e23d74ae45fb17d1() { return handleError(function () {\n    var ret = self.self;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_window_b4be7f48b24ac56e() { return handleError(function () {\n    var ret = window.window;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_globalThis_d61b1f48a57191ae() { return handleError(function () {\n    var ret = globalThis.globalThis;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_global_e7669da72fd7f239() { return handleError(function () {\n    var ret = __webpack_require__.g.global;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_is_undefined(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nfunction __wbindgen_object_clone_ref(arg0) {\n    var ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_random_7b8246250fd79f60 = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');\n\nfunction __wbindgen_debug_string(arg0, arg1) {\n    var ret = debugString(getObject(arg1));\n    var ptr0 = passStringToWasm0(ret, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _moon_engine_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack://moon-engine/../pkg/moon_engine_bg.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var moon_engine__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! moon-engine */ \"../pkg/moon_engine_bg.js\");\n\r\nconst canvas = document.getElementById(\"canvas\");\r\nconst gl = canvas.getContext('webgl2');\r\nconst counter = document.getElementById(\"fpsCounter\");\r\nconst app = new moon_engine__WEBPACK_IMPORTED_MODULE_0__.Application();\r\n\r\nconst FPS_LIMIT = 1000.0 / 30.0;\r\nlet lastDrawTime = -1;\r\n\r\nfunction init() {\r\n    if (!gl) {\r\n        alert('Failed to initialize WebGL2 Context!');\r\n        return;\r\n    }\r\n    app.init();\r\n\r\n    canvas.addEventListener(\"keydown\", event => {\r\n        app.input(event.which, true);\r\n    });\r\n    canvas.addEventListener(\"keyup\", event => {\r\n        app.input(event.which, false);\r\n    });\r\n    canvas.addEventListener(\"mousemove\", event => {\r\n        app.mouse_move(event.clientX, event.clientY);\r\n    }, false);\r\n\r\n    app.resize(window.innerWidth, window.innerHeight);\r\n    function render() {\r\n        window.requestAnimationFrame(render);\r\n        let currentTime = performance.now();\r\n        let deltaTime = currentTime - lastDrawTime;\r\n        if (deltaTime >= 0) { // Ignoring FPS_LIMIT for now\r\n            lastDrawTime = currentTime;\r\n\r\n            if (canvas.height != window.innerHeight || canvas.width != window.innerWidth) {\r\n                canvas.height = window.innerHeight;\r\n                canvas.style.height = window.innerHeight;\r\n                \r\n                canvas.width = window.innerWidth;\r\n                canvas.style.width = window.innerWidth;\r\n                app.resize(window.innerWidth, window.innerHeight);\r\n            }\r\n            app.render(deltaTime);\r\n            counter.innerText = Math.round(1000/deltaTime);\r\n        }\r\n    }\r\n\r\n    render();\r\n}\r\n\r\ninit();\n\n//# sourceURL=webpack://moon-engine/./index.js?");

/***/ }),

/***/ "../pkg/moon_engine_bg.wasm":
/*!**********************************!*\
  !*** ../pkg/moon_engine_bg.wasm ***!
  \**********************************/
/***/ ((module, exports, __webpack_require__) => {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.id];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name) exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./moon_engine_bg.js */ \"../pkg/moon_engine_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"\"]()\n\n//# sourceURL=webpack://moon-engine/../pkg/moon_engine_bg.wasm?");

/***/ })

}]);