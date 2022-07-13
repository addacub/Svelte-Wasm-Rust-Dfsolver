import { writable, type Writable } from 'svelte/store';

// Variables used for the drag and drop game
export let drawScale: Writable<number> = writable(60);
export let lineWidth: Writable<number> = writable(0.05);
export let scaleFactor: Writable<number> = writable(0.8);
export let isSelectable: Writable<boolean> = writable(true)


/**
 * #0dc566 = hsl(149, 88, 41)
 * #dd33ff = hsl(290, 100, 60)
 * #e0d900 = hsl(58, 100, 44)
 * #5cffe9 = hsl(172, 100, 68)
 * #ff4775 = hsl(345,100,64)
 * #479dff = hsl(212,100,64)
 * #ff1f1f = hsl(0,100,56)
 * #97fe72 = hsl(104,99,72)
 */

/**
 * Is a cartesian coordinate
 * @param x The x-coordinate of the cartesian coordinate
 * @param y The y-coordinate of the cartesian coordinate
 */
export class Point {
    
    private x: number;
    private y: number;

    public constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    public getX(): number {
        return this.x;
    }

    public getY(): number {
        return this.y;
    }

}

export let mousePosition: Writable<Point> = writable(new Point(0, 0));

/**
 * Automatically calculates the stroke lightness based on input lightness
 * @param lightness The fill lightness of the piece
 * @returns the stroke lightness of the internal borders
 */
export function getStrokeLightness(lightness: number): number {
    if (lightness < 75) {
        return lightness + 25;
    } else {
        return lightness - 25;
    }
}