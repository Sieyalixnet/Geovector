import { Vector, createVector } from "./Vector"
import { downloadJSON } from "../utils/text"
export class ImageVector {
    //We need to use other mothod to create a ImageVector instead of using a consturctor
    constructor() {
        this.name = `Default_${Date.now()}`
        this.ChannelList = []
    }
    set_List(VectorList) { this.ChannelList = VectorList }
    add_List(Vector) {
        this.ChannelList.push(Vector)
    }
    get_Vector(index) { return this.ChannelList[index] }
    set_name(name) { this.name = name }
    add_List_JSON(JSONFileObject) {
        for (let item of JSONFileObject.layers) {
            try {
                let newlayer = createVector(item.data, item.rows, item.cols)
                if (item.name) { newlayer.OptionalAttributes.name = `${item.name}_${Date.now()}` } else { newlayer.OptionalAttributes.name = `Default_${Date.now()}` }
                this.add_List(newlayer)
            } catch (e) {
                alert(`There is an exception, ${e}`)
            }
        }

    }
    render_to_main_canvas(index, reflect) {
        console.log(reflect)
        this.ChannelList[index].render(`main_canvas`, reflect)
    }
    render_to_canvas(reflect) {
        for (let index in this.ChannelList) {
            this.ChannelList[index].render(`${this.name}_${index}`)
        }
    }
    render_thumbnails_to_canvas(reflect) {
        for (let index in this.ChannelList) {
            console.log(`${this.name}_${index}`)
            console.log(reflect)
            this.ChannelList[index].render_thumbnails(`${this.name}_${index}`, reflect)
        }
    }
    delete(index) {
        this.ChannelList[index].drop()
        this.ChannelList[index] = null
        console.log(this.ChannelList[index])
        this.ChannelList.splice(index, 1)
    }
    upward(index) {
        let temp = this.ChannelList[index]
        this.ChannelList[index] = this.ChannelList[index - 1]
        this.ChannelList[index - 1] = temp
    }
    downward(index) {
        let temp = this.ChannelList[index]
        this.ChannelList[index] = this.ChannelList[index + 1]
        this.ChannelList[index + 1] = temp
    }
    copy(index) {

        let temp = createVector(this.ChannelList[index].array(), this.ChannelList[index].get_rows(), this.ChannelList[index].get_cols())
        temp.OptionalAttributes.name = `${this.ChannelList[index].OptionalAttributes.name}_copy`
        this.ChannelList.push(temp)

    }
    json_to_download() {
        let layers = this.ChannelList.map(x => x.createLayerJSON())

        let Imagefile = {
            "files": [{
                "filename": this.name,
                "layers":
                    layers
            }]
        }
        let json = JSON.stringify(Imagefile);
        downloadJSON(json)

    }

    __copy__(index) {//return a new Vector. 

        let temp = createVector(this.ChannelList[index].array(), this.ChannelList[index].get_rows(), this.ChannelList[index].get_cols())
        temp.OptionalAttributes.name = `${this.ChannelList[index].OptionalAttributes.name}_copy`
        return temp

    }
}


export function createImageVector_CTX(ctx, width, height) {
    const data = Array.from(ctx.getImageData(0, 0, width, height).data)
    let R = createVector(data.filter((_, index) => index % 4 == 0), height, width);
    let G = createVector(data.filter((_, index) => index % 4 == 1), height, width);
    let B = createVector(data.filter((_, index) => index % 4 == 2), height, width);
    let A = createVector(data.filter((_, index) => index % 4 == 3), height, width);
    // let R =new Vector(data.filter((_, index) => index % 4 == 0), height, width);
    // let G =new Vector(data.filter((_, index) => index % 4 == 1), height, width);
    // let B =new Vector(data.filter((_, index) => index % 4 == 2), height, width);
    // let A =new Vector(data.filter((_, index) => index % 4 == 3), height, width);
    R.OptionalAttributes.name = `R_${Date.now()}`
    G.OptionalAttributes.name = `G_${Date.now()}`
    B.OptionalAttributes.name = `B_${Date.now()}`
    A.OptionalAttributes.name = `A_${Date.now()}`
    let result = new ImageVector()
    result.set_List([R, G, B, A])
    return result
}

export function createImageVector_JSON(JSONFileObject) {//Object,not json. each file, not files.
    let result = new ImageVector()
    for (let item of JSONFileObject.layers) {
        try {
            let newlayer = createVector(item.data, item.rows, item.cols)
            if (item.name) { newlayer.OptionalAttributes.name = item.name } else { newlayer.OptionalAttributes.name = `Default_${Date.now()}` }
            result.add_List(newlayer)
        } catch (e) {
            alert(`There is an exception, ${e}`)
        }
    }
    try { result.set_name(JSONFileObject.filename) } catch (_) { }
    return result
}