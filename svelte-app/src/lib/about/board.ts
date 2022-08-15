import { board_draw_scale, board_svg_padding, board_svg_stroke_width, board_matrix_draw_scale, board_matrix_svg_padding, board_matrix_svg_stroke_width } from '$lib/about/store'

// Initialising variables
let board_scale: number = 1;
let board_stroke_width: number = 1;
let board_padding: number = 1;

let matrix_scale: number = 1;
let matrix_padding: number = 1;
let matrix_stroke_width: number = 1;

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

board_matrix_draw_scale.subscribe(value => {
    matrix_scale = value
});

board_matrix_svg_padding.subscribe(value => {
    matrix_stroke_width = value
})

board_matrix_svg_stroke_width.subscribe(value => {
    matrix_padding = value
})

export class BoardInfo {
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

function create_board(padding: number, scale: number, stroke_width: number): BoardInfo {
    return new BoardInfo(7, 7, `m${padding * scale},${padding * scale} h${6 * scale} v${2 * scale} h${1 * scale} v${4 * scale} h${-4 * scale} v${1 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [22, 53, 85])
}

export const board: BoardInfo = create_board(board_padding, board_scale, board_stroke_width)
export const board_rep: BoardInfo = create_board(matrix_padding, matrix_scale, matrix_stroke_width)

