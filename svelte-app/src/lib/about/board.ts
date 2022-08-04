import { board_draw_scale, board_svg_padding, board_svg_stroke_width } from '$lib/about/store'

// Initialising variables
let board_scale: number = 1;
let board_stroke_width: number = 1;
let board_padding: number = 1;


// Subscribing to store values
board_draw_scale.subscribe(value => {
    board_scale = value
});

board_svg_stroke_width.subscribe(value => {
    board_stroke_width = value
})

board_svg_padding.subscribe(value => {
    board_padding = value
})


export class Board {
    readonly width: number;
    readonly height: number;
    readonly path: string;
    readonly stroke_width: number;
    readonly padding: number;
    readonly draw_scale: number;
    readonly fill: [number, number, number]

    constructor(width: number, height: number, path: string, stroke_width: number, padding: number, draw_scale: number, fill: [number, number, number]) {
        this.width = width;
        this.height = height;
        this.path = path;
        this.stroke_width = stroke_width;
        this.padding = padding;
        this.draw_scale = draw_scale;
        this.fill = fill
    }
}

export const board: Board = new Board(7, 7,
    `m${board_padding * board_scale},${board_padding * board_scale} h${6 * board_scale} v${2 * board_scale} h${1 * board_scale} v${4 * board_scale} h${-4 * board_scale} v${1 * board_scale} h${-3 * board_scale} z`,
    board_stroke_width, board_padding, board_scale, [22, 53, 85])

