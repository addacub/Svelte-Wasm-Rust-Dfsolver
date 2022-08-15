import { writable, type Writable } from 'svelte/store';

/** Store values for puzzle pieces used in tables */
export let table_draw_scale: Writable<number> = writable(20);
export let table_svg_stroke_width: Writable<number> = writable(0.05);
export let table_svg_padding: Writable<number> = writable(0.05);

/** Store values for puzzle piece used in animation for representing the piece */
export let maxtrix_draw_scale: Writable<number> = writable(50);
export let matrix_svg_stroke_width: Writable<number> = writable(0.05);
export let matrix_svg_padding: Writable<number> = writable(0.05);

/** Store values for board and puzzle pieces */
export let board_draw_scale: Writable<number> = writable(60);
export let board_svg_stroke_width: Writable<number> = writable(0.05);
export let board_svg_padding: Writable<number> = writable(0.05);

/** Store values for puzzle piece orientations */
export let orient_draw_scale: Writable<number> = writable(40);
export let orient_svg_stroke_width: Writable<number> = writable(0.05);
export let orient_svg_padding: Writable<number> = writable(0.05);

/** Store values for representing board as matrix */
export let board_matrix_draw_scale: Writable<number> = writable(40);
export let board_matrix_svg_stroke_width: Writable<number> = writable(0.05);
export let board_matrix_svg_padding: Writable<number> = writable(0.05);

/** Constant class for table headings */
export class TableHeadings {
    public static readonly piece: string = 'Piece';
    public static readonly rotations: string = 'Rotational positions (R)';
    public static readonly flips: string = 'Flip (reflection) positions (F)';
    public static readonly unique_orient_math: string = 'Unique orientations (=R&#215;F)';
    public static readonly unique_orient: string = 'Unique orientations';
    public static readonly level: string = 'Level';
    private constructor() { }
}
