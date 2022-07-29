import { BaseVector } from "geo-vector";
import { memory } from "geo-vector/geo_vector_bg.wasm";

export function createVector(inputData) {
    let data = new Vector(inputData);
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
        if (typeof data == "number") {
            _data = data.map((value) => { return Number(value) });
        } else if (typeof data == "object") {
            //TODO this is not enough to judge the type of data
            _rows = data.length
            _cols = data[0].length
            _data = data.flat()
        }
        this.Data = BaseVector.new(_rows, _cols, _data)
        this.ptr = this.get_ptr()//these are static fields, so we need use update function to update them
        this.cols = this.get_cols()
        this.rows = this.get_rows()
    }
    memoryArray() {
        return new Float64Array(memory.buffer, this.get_ptr(), this.get_cols() * this.get_rows())
    }
    array() {
        return Array.from(this.memory_Array())
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

    update(){
        this.ptr = this.get_ptr()
        this.cols = this.get_cols()
        this.rows = this.get_rows()
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
    mm(vector) {
        this.Data.mm(vector.Data)
        this.Data.log_data_string()
    }

}