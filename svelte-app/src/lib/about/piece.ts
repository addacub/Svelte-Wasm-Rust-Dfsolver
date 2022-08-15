import {
    table_draw_scale,
    table_svg_stroke_width,
    table_svg_padding,
    maxtrix_draw_scale,
    matrix_svg_stroke_width,
    matrix_svg_padding,
    board_draw_scale,
    board_svg_padding,
    board_svg_stroke_width,
    orient_draw_scale,
    orient_svg_padding,
    orient_svg_stroke_width
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

let orient_scale: number = 1;
let orient_stroke_width: number = 1;
let orient_padding: number = 1;

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

orient_draw_scale.subscribe(value => {
    orient_scale = value
});

orient_svg_stroke_width.subscribe(value => {
    orient_stroke_width = value
})

orient_svg_padding.subscribe(value => {
    orient_padding = value
})


export class PieceInfo {
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

enum PieceType {
    no_hole_2x3,
    middle_hole_2x3,
    end_hole_2x3,
    tee_2x4,
    L_2x4,
    zig_zag_2x4,
    zig_zag_3x3,
    L_3x3
}

function create_no_hole_2x3(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(3, 2, `m${padding * scale},${padding * scale} h${3 * scale} v${2 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [149, 88, 41])
}

function create_middle_hole_2x3(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(3, 2, `m${padding * scale},${padding * scale} h${1 * scale} v${1 * scale} h${1 * scale} v${-1 * scale} h${1 * scale} v${2 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [290, 100, 60])
}

function create_end_hole_2x3(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(3, 2, `m${padding * scale},${padding * scale} h${2 * scale} v${1 * scale} h${1 * scale} v${1 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [172, 100, 68])
}

function create_tee_2x4(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(4, 2, `m${padding * scale},${padding * scale} h${4 * scale} v${1 * scale} h${-2 * scale} v${1 * scale} h${-1 * scale} v${-1 * scale} h${-1 * scale} z`, stroke_width, padding, scale, [104, 99, 72])
}

function create_L_2x4(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(4, 2, `m${padding * scale},${padding * scale} h${4 * scale} v${2 * scale} h${-1 * scale} v${-1 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [345, 100, 64])
}

function create_zig_zag_2x4(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(4, 2, `m${padding * scale},${padding * scale} h${3 * scale} v${1 * scale} h${1 * scale} v${1 * scale} h${-2 * scale} v${-1 * scale} h${-2 * scale} z`, stroke_width, padding, scale, [0, 100, 56])
}

function create_zig_zag_3x3(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(3, 3, `m${padding * scale},${padding * scale} h${2 * scale} v${2 * scale} h${1 * scale} v${1 * scale} h${-2 * scale} v${-2 * scale} h${-1 * scale} z`, stroke_width, padding, scale, [58, 100, 44])
}

function create_L_3x3(padding: number, scale: number, stroke_width: number): PieceInfo {
    return new PieceInfo(3, 3, `m${padding * scale},${padding * scale} h${1 * scale} v${2 * scale} h${2 * scale} v${1 * scale} h${-3 * scale} z`, stroke_width, padding, scale, [212, 100, 64])
}

function create_piece(piece_type: PieceType, padding: number, scale: number, stroke_width: number): PieceInfo {
    switch (piece_type) {
        case PieceType.no_hole_2x3:
            return create_no_hole_2x3(padding, scale, stroke_width);
        case PieceType.middle_hole_2x3:
            return create_middle_hole_2x3(padding, scale, stroke_width);
        case PieceType.end_hole_2x3:
            return create_end_hole_2x3(padding, scale, stroke_width);
        case PieceType.zig_zag_2x4:
            return create_zig_zag_2x4(padding, scale, stroke_width);
        case PieceType.tee_2x4:
            return create_tee_2x4(padding, scale, stroke_width);
        case PieceType.L_2x4:
            return create_L_2x4(padding, scale, stroke_width);
        case PieceType.zig_zag_3x3:
            return create_zig_zag_3x3(padding, scale, stroke_width);
        case PieceType.L_3x3:
            return create_L_3x3(padding, scale, stroke_width);
    }
}

// Define puzzle pieces to be used in tables
export let table_pieces: readonly PieceInfo[] = [
    create_piece(PieceType.no_hole_2x3, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.middle_hole_2x3, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.end_hole_2x3, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.tee_2x4, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.L_2x4, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.zig_zag_2x4, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.zig_zag_3x3, table_padding, table_scale, table_stroke_width),
    create_piece(PieceType.L_3x3, table_padding, table_scale, table_stroke_width)
]

// Define puzzle piece to display unique orientations
export const zig_zag_2x4_med: PieceInfo = create_piece(PieceType.zig_zag_2x4, orient_padding, orient_scale, orient_stroke_width);

// Define puzzle piece for rotational and reflection symmetry
export const no_hole_2x3_sym = create_piece(PieceType.no_hole_2x3, board_padding, board_scale, board_stroke_width);

// Define puzzle piece to be used in how to represent demo
export const middle_hole_2x3_rep: PieceInfo = create_piece(PieceType.middle_hole_2x3, matrix_padding, matrix_scale, matrix_stroke_width);


// Define puzzle pieces to be used in how to place pieces demo
export const end_hole_2x3_big: PieceInfo = create_piece(PieceType.end_hole_2x3, board_padding, board_scale, board_stroke_width);
export const middle_hole_2x3_big: PieceInfo = create_piece(PieceType.middle_hole_2x3, board_padding, board_scale, board_stroke_width);
export const zig_zag_2x4_big: PieceInfo = create_piece(PieceType.zig_zag_2x4, board_padding, board_scale, board_stroke_width);