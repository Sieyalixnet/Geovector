import { BaseVector } from "geo-vector";
import { memory } from "geo-vector/geo_vector_bg.wasm";

export function createVector(array, rows, cols) {//rows->height, cols->width
    const data = new Vector(array, rows, cols);
    return new Proxy(data, VectorHandler(data));
}

function VectorHandler(target) {
    return {
        get(target, key) {
            let res = Reflect.get(target, key);
            console.log('get')
            return res
        },
        set(target, key, value) {
            let res = Reflect.set(target, key, value);
            console.log('set')
            return res
        }
    }
}

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
        this.OptionalAttributes = {}
    }
    //in this part, functions mostly get/set things from WASM
    memoryArray() {
        return new Float64Array(memory.buffer, this.get_ptr(), this.get_cols() * this.get_rows())
    }
    array() {
        return Array.from(this.memoryArray())
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
    get(row, col) { return this.Data.get(row, col) }
    set(row, col, value) { this.Data.set(row, col, value) }
    get_index(index) { return this.Data.get_index(index) }
    set_index(index, value) { this.Data.set_index(index, value) }
    conv2d(kernel, stride) {
        this.Data.conv2d(kernel, stride)//kernel should reshape in one dimension
        this.update()
    }
    conv2d_array(kernel, stride) {//this whill return a 1-dimension array
        return this.Data.conv2d_array(kernel, stride)
    }

    update() {
        this.ptr = this.get_ptr()
        this.cols = this.get_cols()
        this.rows = this.get_rows()
    }
    transpose() {
        this.Data.transpose()
    }
    mul(vector) {
        this.Data.mul(vector.Data)
    }
    sub(vector) {
        this.Data.sub(vector.Data)
    }
    add(vector) {
        this.Data.add(vector.Data)
    }
    div(vector) {
        this.Data.div(vector.Data)
    }
    mul_value(value) {
        this.Data.mul_value(value)
    }
    sub_value(value) {
        this.Data.sub_value(value)
    }

    add_value(value) {
        this.Data.add_value(value)
    }

    div_value(value) {
        this.Data.div_value(value)
    }


    mm(vector) {
        this.Data.mm(vector.Data)
    }
    padding(padding_value) {
        this.Data.padding(padding_value);
        this.update()
    }
    padding_times(padding_value, times) {
        this.Data.padding_times(padding_value, times);
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
        let ctx = canvas.getContext('2d')

        let date = Date.now()
        let imageData = new ImageData(Uint8ClampedArray.from(this.Data.render_thumbnails(ratio, reflect)), Math.ceil(this.cols / ratio), Math.ceil(this.rows / ratio), { colorSpace: "srgb" })
        console.log(`${Date.now() - date}ms`);

        ctx.putImageData(imageData, (thumbnails_size - Math.ceil(this.cols / ratio)) / 2, (thumbnails_size - Math.ceil(this.rows / ratio)) / 2)


    }
    render(canvasID, reflect = true) {
        let canvas = document.getElementById(canvasID)
        let ctx = canvas.getContext('2d')
        ctx.clearRect(0, 0, canvas.width, canvas.height)
        canvas.width = this.cols
        canvas.height = this.rows

        let date = Date.now()
        // let data = (this.array().map((value) => { return [value,value,value,255] })).flat()

        // let imageData = new ImageData(Uint8ClampedArray.from(data), this.cols, this.rows,{colorSpace:"srgb"})
        let imageData = new ImageData(Uint8ClampedArray.from(this.Data.render(reflect)), this.cols, this.rows, { colorSpace: "srgb" })
        console.log(`${Date.now() - date}ms`);
        ctx.putImageData(imageData, 0, 0)


    }
}