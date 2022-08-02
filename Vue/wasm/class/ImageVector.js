import { Vector,createVector } from "./Vector"

export class ImageVector{
    //We need to use other mothod to create a ImageVector instead of using a consturctor
    constructor(){
        this.name = `Default_${Date.now()}`
        this.ChannelList = []
    }
    set_List(VectorList){this.ChannelList=VectorList}
    add_List(Vector){this.ChannelList.push(Vector)}
    set_name(name){this.name=name}
    render_to_main_canvas(index){
        this.ChannelList[index].render(`main_canvas`)
    }
    render_to_canvas(){
        for(let index in this.ChannelList){
            console.log(`${this.name}_${index}`)
            this.ChannelList[index].render(`${this.name}_${index}`)
        }
    }
    render_thumbnails_to_canvas(){
        for(let index in this.ChannelList){
            console.log(`${this.name}_${index}`)
            this.ChannelList[index].render_thumbnails(`${this.name}_${index}`)
        }
    }
}


export function createImageVector_CTX(ctx, width, height){
    const data = Array.from(ctx.getImageData(0, 0, width, height).data)
    let R = createVector(data.filter((_, index) => index % 4 == 0), height, width);
    let G = createVector(data.filter((_, index) => index % 4 == 1), height, width);
    let B = createVector(data.filter((_, index) => index % 4 == 2), height, width);
    let A = createVector(data.filter((_, index) => index % 4 == 3), height, width);
    R.OptionalAttributes.name = `R_${Date.now()}`
    G.OptionalAttributes.name = `G_${Date.now()}`
    B.OptionalAttributes.name = `B_${Date.now()}`
    A.OptionalAttributes.name = `A_${Date.now()}`
    let result = new ImageVector()
    result.set_List([R,G,B,A])
    return result
}