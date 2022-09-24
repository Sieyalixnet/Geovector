import { BaseVector,clear } from "geo-vector";
import { memory } from "geo-vector/geo_vector_bg.wasm";
import { reshape, reflect_to } from "../utils/array";
import { downloadImage, createCanvas } from "../utils/canvas";
import { downloadJSON } from "../utils/text";

export function createVector(array, rows, cols) {//rows is height, cols is width
    const data = new Vector(array, rows, cols);
    return new Proxy(data, VectorHandler(data));
}

function VectorHandler(target) {
    return {
        get(target, key) {
            let res = Reflect.get(target, key);
            return res
        },
        set(target, key, value) {
            let res = Reflect.set(target, key, value);
            return res
        }
    }
}

//Vector is a class of raw data of a channel of image. They will instantly pass to WASM, most operations will invoke the WASM and read from memory of WASM.
export class Vector {
    constructor(data, rows, cols) {
        let _rows = rows
        let _cols = cols
        let _data = data
        if (typeof data[0] == "number") {
            _data = data.map((value) => { return Number(value) });
        } else if (typeof data[0] == "object") {
            //TODO this is not enough to judge the type of data
            _rows = data.length
            _cols = data[0].length
            _data = data.flat()
        }
        this.Data = BaseVector.new(_rows, _cols, _data)
        this.ptr = this.get_ptr()//these are static fields, so we need use update function to update them
        this.cols = this.get_cols()
        this.rows = this.get_rows()
        this.max = this.get_max()
        this.min = this.get_min()
        this.OptionalAttributes = {}
    }
    //in this part, functions mostly get/set things from WASM
    memoryArray() {
        return new Float64Array(memory.buffer, this.get_ptr(), this.get_cols() * this.get_rows())
    }
    array() {
        return Array.from(this.memoryArray())
    }
    drop() {
        this.Data.clear()
        clear(this.Data)
    }
    get_ptr() {
        return this.Data.get_ptr()
    }
    get_cols() {
        return this.Data.get_cols()
    }
    get_rows() {
        return this.Data.get_rows()
    }
    get_max() {
        return this.Data.get_max()
    }
    get_min() {
        return this.Data.get_min()
    }
    // get(row, col) { return this.Data.get(row, col) }//leads to memory leak
    get(row, col) {
        let position = row * this.cols + col
        if (position < this.memoryArray().length) {
            return this.memoryArray()[position]
        }
    }
    set(row, col, value) { this.Data.set(row, col, Number(value)); this.update() }
    // get_index(index) { return this.Data.get_index(index) }// leads to memory leak
    get_index(index) {
        if (index < this.memoryArray().length) {
            return this.memoryArray()[index]
        }
    }
    set_index(index, value) { this.Data.set_index(index, Number(value)); this.update() }
    conv2d(kernel, stride) {//kernel is mapped as Number in Vue
        let kernel_final = kernel.map((x) => Number(x));

        let date = Date.now()
        this.Data.conv2d(kernel_final, Number(stride))//kernel should reshape in one dimension
        this.update()
        console.log("conv2d:", Date.now() - date, "ms")
    }

    update() {
        this.ptr = this.get_ptr()
        this.cols = this.get_cols()
        this.rows = this.get_rows()
        this.max = this.get_max()
        this.min = this.get_min()
    }
    transpose() {
        this.Data.transpose()
        this.update()
    }
    mul(vector) {
        this.Data.mul(vector.Data)
        this.update()
    }
    sub(vector) {
        this.Data.sub(vector.Data)
        this.update()
    }
    add(vector) {
        let date = Date.now()
        this.Data.add(vector.Data)
        this.update()
        console.log("add:", Date.now() - date, "ms")
    }
    div(vector) {
        this.Data.div(vector.Data)
        this.update()
    }
    mm(vector) {
        this.Data.mm(vector.Data)
        this.update()
    }

    
    mul_value(value) {//These functions is checked.
        this.Data.mul_value(value)
        this.update()
    }
    sub_value(value) {
        this.Data.sub_value(value)
        this.update()
    }

    add_value(value) {
        this.Data.add_value(value)
        this.update()
    }

    div_value(value) {
        this.Data.div_value(value)
        this.update()
    }

    padding(padding_value) {
        this.Data.padding(padding_value);
        this.update()
    }
    padding_times(padding_value, times) {
        this.Data.padding_times(padding_value, times);
        this.update()
    }
    normalize(type) {
        let types = ["min_max", "z_score"]
        if (types.includes(type)) {
            this.Data.normalize(type)
        }
        this.update()
    }
    reflect_to(min, max) {
        this.Data.reflect_to(min, max)
        this.update()
    }
    rescale_to(size) {
        let ratio = 1;
        if (this.rows > size || this.cols > size) {
            ratio = this.rows > this.cols ? Math.ceil(this.rows / size) : Math.ceil(this.cols / size)
        }
        this.Data.rescale_to(ratio)
        this.update()
    }
    replace(min, max, new_value) {
        this.Data.replace(min, max, new_value)
        this.update()
    }
    range_reflect(min_range, max_range, min_reflect, max_reflect) {
        this.Data.range_reflect(min_range, max_range, min_reflect, max_reflect)
        this.update()
    }
    abs() {
        this.Data.abs()
        this.update()
    }
    reverse_horizontal() {
        this.Data.reverse_horizontal()
        this.update()
    }
    reverse_vertical() {
        this.Data.reverse_vertical()
        this.update()
    }
    //TODO description
    render_thumbnails(canvasID, reflect = true) {
        let ratio = 1;
        const thumbnails_size = 100;//set this param according to the size of the canvas
        if (this.rows > thumbnails_size || this.cols > thumbnails_size) {
            ratio = this.rows > this.cols ? Math.ceil(this.rows / thumbnails_size) : Math.ceil(this.cols / thumbnails_size)
        }
        let canvas = document.getElementById(canvasID)
        if (!canvas) { return; }
        let ctx = canvas.getContext('2d')
        ctx.clearRect(0, 0, canvas.width, canvas.height)
        // let date = Date.now()
        let temp = __render_thumbnails__(this, ratio, reflect)
        let imageData = new ImageData(Uint8ClampedArray.from(temp), Math.ceil(this.cols / ratio), Math.ceil(this.rows / ratio), { colorSpace: "srgb" })
        // console.log(`${Date.now() - date}ms`);
        ctx.putImageData(imageData, (thumbnails_size - Math.ceil(this.cols / ratio)) / 2, (thumbnails_size - Math.ceil(this.rows / ratio)) / 2)


    }
    render(canvasID, reflect = true) {
        let canvas = document.getElementById(canvasID)
        if (!canvas) { return; }
        let ctx = canvas.getContext('2d')

        ctx.clearRect(0, 0, canvas.width, canvas.height)
        canvas.width = this.cols
        canvas.height = this.rows

        let date = Date.now()
        let data = __render__(this, reflect)

        let imageData = new ImageData(Uint8ClampedArray.from(data), this.cols, this.rows, { colorSpace: "srgb" })
        // let imageData = new ImageData(Uint8ClampedArray.from(this.Data.render(reflect)), this.cols, this.rows, { colorSpace: "srgb" })
        console.log(`Render to Main: ${Date.now() - date}ms`);
        ctx.putImageData(imageData, 0, 0)


    }

    render_to_downlaod() {
        this.update()
        let data = __render__(this, false)
        let canvas = createCanvas(Uint8ClampedArray.from(data), this.cols, this.rows)
        downloadImage(canvas)
    }
    json_to_download(filename) {//download this layer only
        this.update()
        let Imagefile = {
            files: [{
                filename: filename,
                layers: [
                    this.createLayerJSON()
                ]
            }]

        }
        let json = JSON.stringify(Imagefile);
        // console.log(json)
        // console.log(JSON.parse(json))
        downloadJSON(json)
    }

    createLayerJSON() { //create an object that can be used to download
        this.update()
        return {
            "name": this.OptionalAttributes.name,
            "cols": this.cols,
            "rows": this.rows,
            "data": this.array()
        }
    }
}

function __render__(Vector, reflect) {
    let data_reflected
    let data = Vector.memoryArray()
    if (reflect) {
        data_reflected = reflect_to(data, 0.0, 255.0);
    } else {
        data_reflected = data
    }

    let result = []
    for (let i = 0; i < data_reflected.length; i++) {
        result.push(data_reflected[i], data_reflected[i], data_reflected[i], 255)
    }
    return result

}

function __render_thumbnails__(Vector, ratio, reflect) {
    let temp = reshape(Vector.memoryArray(), [Vector.get_rows(), -1]);
    let temp_rows = [];
    for (let i = 0; i < temp.length; i++) {
        if (i % ratio == 0) {
            let new_row = [];
            for (let j = 0; j < temp[i].length; j++) {
                if (j % ratio == 0) {
                    new_row.push(temp[i][j]);
                }
            }
            temp_rows.push(new_row);
        }
    }
    temp_rows = temp_rows.flat();
    if (reflect) {
        temp_rows = reflect_to(temp_rows, 0.0, 255.0);
    }
    let result = (temp_rows.map((value) => { return [value, value, value, 255] })).flat()
    return result
}