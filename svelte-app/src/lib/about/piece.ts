import {
    table_draw_scale,
    table_svg_stroke_width,
    table_svg_padding,
    maxtrix_draw_scale,
    matrix_svg_stroke_width,
    matrix_svg_padding,
    board_draw_scale,
    board_svg_padding,
    board_svg_stroke_width
} from '$lib/about/store'

// Initialising variables
let table_scale: number = 1;
let table_stroke_width: number = 1;
let table_padding: number = 1;

let matrix_scale: number = 1;
let matrix_stroke_width: number = 1;
let matrix_padding: number = 1;

let board_scale: number = 1;
let board_stroke_width: number = 1;
let board_padding: number = 1;

// Subscribing to store values
table_draw_scale.subscribe(value => {
    table_scale = value
});

table_svg_stroke_width.subscribe(value => {
    table_stroke_width = value
})

table_svg_padding.subscribe(value => {
    table_padding = value
})

maxtrix_draw_scale.subscribe(value => {
    matrix_scale = value
})

matrix_svg_stroke_width.subscribe(value => {
    matrix_stroke_width = value
})

matrix_svg_padding.subscribe(value => {
    matrix_padding = value
})

board_draw_scale.subscribe(value => {
    board_scale = value
});

board_svg_stroke_width.subscribe(value => {
    board_stroke_width = value
})

board_svg_padding.subscribe(value => {
    board_padding = value
})

export class Piece {
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

// Define puzzle pieces to be used in tables
export let table_pieces: readonly Piece[] = [
    new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${3 * table_scale} v${2 * table_scale} h${-3 * table_scale} z`, table_stroke_width, table_padding, table_scale, [149, 88, 41]),
    new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${1 * table_scale} v${1 * table_scale} h${1 * table_scale} v${-1 * table_scale} h${1 * table_scale} v${2 * table_scale} h${-3 * table_scale} z`, table_stroke_width, table_padding, table_scale, [290, 100, 60]),
    new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${2 * table_scale} v${1 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-3 * table_scale} z`, table_stroke_width, table_padding, table_scale, [172, 100, 68]),
    new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${4 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${1 * table_scale} h${-1 * table_scale} v${-1 * table_scale} h${-1 * table_scale} z`, table_stroke_width, table_padding, table_scale, [104, 99, 72]),
    new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${4 * table_scale} v${2 * table_scale} h${-1 * table_scale} v${-1 * table_scale} h${-3 * table_scale} z`, table_stroke_width, table_padding, table_scale, [345, 100, 64]),
    new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${3 * table_scale} v${1 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${-1 * table_scale} h${-2 * table_scale} z`, table_stroke_width, table_padding, table_scale, [0, 100, 56]),
    new Piece(3, 3, `m${table_padding * table_scale},${table_padding * table_scale} h${2 * table_scale} v${2 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${-2 * table_scale} h${-1 * table_scale} z`, table_stroke_width, table_padding, table_scale, [58, 100, 44]),
    new Piece(3, 3, `m${table_padding * table_scale},${table_padding * table_scale} h${1 * table_scale} v${2 * table_scale} h${2 * table_scale} v${1 * table_scale} h${-3 * table_scale} z`, table_stroke_width, table_padding, table_scale, [212, 100, 64])
]

// Define puzzle piece to be used in how to represent demo
export const represented_piece: Piece = new Piece(3, 2, `m${matrix_padding * matrix_scale},${matrix_padding * matrix_scale} h${1 * matrix_scale} v${1 * matrix_scale} h${1 * matrix_scale} v${-1 * matrix_scale} h${1 * matrix_scale} v${2 * matrix_scale} h${-3 * matrix_scale} z`, matrix_stroke_width, matrix_padding, matrix_scale, [290, 100, 60])

// Define puzzle pieces to be used in how to place pieces demo
export const end_hole_2x3: Piece = new Piece(3, 2, `m${board_padding * board_scale},${board_padding * board_scale} h${3 * board_scale} v${1 * board_scale} h${-1 * board_scale} v${1 * board_scale} h${-2 * board_scale} z`, board_stroke_width, board_padding, board_scale, [172, 100, 68]);
export const middle_hole_2x3: Piece = new Piece(3, 2, `m${board_padding * board_scale},${board_padding * board_scale} h${1 * board_scale} v${1 * board_scale} h${1 * board_scale} v${-1 * board_scale} h${1 * board_scale} v${2 * board_scale} h${-3 * board_scale} z`, board_stroke_width, board_padding, board_scale, [290, 100, 60])
export const zig_zag_2x4: Piece = new Piece(4, 2, `m${board_padding * board_scale},${(board_padding + 1) * board_scale} h${1 * board_scale} v${-1 * board_scale} h${3 * board_scale} v${1 * board_scale} h${-2 * board_scale} v${1 * board_scale} h${-2 * board_scale} z`, table_stroke_width, board_padding, board_scale, [0, 100, 56])