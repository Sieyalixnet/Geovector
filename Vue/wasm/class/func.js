import { downloadImage} from "../utils/canvas";
export function renderRGBA(canvasID, channels) { //{R,G,B,A}=channels
    let canvas = document.getElementById(canvasID)
    if (!canvas) { return; }
    let ctx = canvas.getContext('2d')

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    const { R } = channels
    canvas.width = R.cols
    canvas.height = R.rows

    //make a default AlphaChannel
    const { A } = channels
    if (!A) {
        let temp = Array(R.memoryArray().length).fill(255)
        channels.A = temp;
    }

    let date = Date.now()
    let data = __renderRGBA__(channels)

    let imageData = new ImageData(Uint8ClampedArray.from(data), R.cols, R.rows, { colorSpace: "srgb" })

    console.log(`${Date.now() - date}ms`);
    ctx.putImageData(imageData, 0, 0)


}

function __renderRGBA__(channels) {
    const { R, G, B, A } = channels
    let R_arr, G_arr, B_arr, A_arr;
    if (!Array.isArray(A)) {
        [R_arr, G_arr, B_arr, A_arr] = [R.memoryArray(), G.memoryArray(), B.memoryArray(), A.memoryArray()]
    } else { [R_arr, G_arr, B_arr, A_arr] = [R.memoryArray(), G.memoryArray(), B.memoryArray(), A] }

    let ArrayLength = Array.from(new Set([R_arr.length, G_arr.length, B_arr.length, A_arr.length]))
    if (ArrayLength.length > 1) { throw (`Length Wrong!${ArrayLength}`); }

    let result = []
    for (let i = 0; i < R_arr.length; i++) {
        result.push(R_arr[i], G_arr[i], B_arr[i], A_arr[i])
    }
    return result

}

export function download_main_canvas(){
    let canvas = document.getElementById("main_canvas")
    downloadImage(canvas,false)

}