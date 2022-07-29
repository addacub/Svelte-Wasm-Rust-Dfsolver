import { table_draw_scale, maxtrix_draw_scale, matrix_svg_padding, table_svg_padding } from '$lib/about/store'

// Initialising variables
let table_scale: number = 1;
let matrix_scale: number = 1;
let matrix_padding: number = 1;
let table_padding: number = 1;

// Subscribing to store values
table_draw_scale.subscribe(value => {
    table_scale = value
});

maxtrix_draw_scale.subscribe(value => {
    matrix_scale = value
})

matrix_svg_padding.subscribe(value => {
    matrix_padding = value
})

table_svg_padding.subscribe(value => {
    table_padding = value
})

export class Piece {
    private width: number;
    private height: number;
    private path: string;
    private fill: [number, number, number]

    constructor(width: number, height: number, path: string, fill: [number, number, number]) {
        this.width = width;
        this.height = height;
        this.path = path;
        this.fill = fill
    }

    get_width(): number {
        return this.width
    }

    get_height(): number {
        return this.height
    }

    get_path(): string {
        return this.path
    }

    get_fill(): [number, number, number] {
        return this.fill
    }
}

export const pieces: Piece[] = new Array(8)

pieces[0] = new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${3 * table_scale} v${2 * table_scale} h${-3 * table_scale} z`, [149, 88, 41])
pieces[1] = new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${1 * table_scale} v${1 * table_scale} h${1 * table_scale} v${-1 * table_scale} h${1 * table_scale} v${2 * table_scale} h${-3 * table_scale} z`, [290, 100, 60])
pieces[2] = new Piece(3, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${2 * table_scale} v${1 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-3 * table_scale} z`, [172, 100, 68])
pieces[3] = new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${4 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${1 * table_scale} h${-1 * table_scale} v${-1 * table_scale} h${-1 * table_scale} z`, [104, 99, 72])
pieces[4] = new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${4 * table_scale} v${2 * table_scale} h${-1 * table_scale} v${-1 * table_scale} h${-3 * table_scale} z`, [345, 100, 64])
pieces[5] = new Piece(4, 2, `m${table_padding * table_scale},${table_padding * table_scale} h${3 * table_scale} v${1 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${-1 * table_scale} h${-2 * table_scale} z`, [0, 100, 56])
pieces[6] = new Piece(3, 3, `m${table_padding * table_scale},${table_padding * table_scale} h${2 * table_scale} v${2 * table_scale} h${1 * table_scale} v${1 * table_scale} h${-2 * table_scale} v${-2 * table_scale} h${-1 * table_scale} z`, [58, 100, 44])
pieces[7] = new Piece(3, 3, `m${table_padding * table_scale},${table_padding * table_scale} h${1 * table_scale} v${2 * table_scale} h${2 * table_scale} v${1 * table_scale} h${-3 * table_scale} z`, [212, 100, 64])


export let animated_piece: Piece = new Piece(3, 2, `m${matrix_padding * matrix_scale},${matrix_padding * matrix_scale} h${1 * matrix_scale} v${1 * matrix_scale} h${1 * matrix_scale} v${-1 * matrix_scale} h${1 * matrix_scale} v${2 * matrix_scale} h${-3 * matrix_scale} z`, [290, 100, 60])

